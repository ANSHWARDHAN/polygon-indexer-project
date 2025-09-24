use std::time::Duration;
use web3::types::{FilterBuilder, H160, H256, U256, Log};
use web3::ethabi::{Event, EventParam, ParamType};
use hex::ToHex;
use tokio::time::sleep;
use crate::db;
use std::str::FromStr;
use web3::transports::Http;
use web3::Web3;

// POL contract address (on Polygon) - update if needed
// Source: CoinGecko / token listings. See README for citation.
const POL_CONTRACT: &str = "0x455e53cbb86018ac2b8092fdcd39d8444affc3f6";

// Binance addresses (from provided task doc)
const BINANCE_ADDRESSES: [&str; 6] = [
    "0xF977814e90dA44bFA03b6295A0616a897441aceC",
    "0xe7804c37c13166fF0b37F5aE0BB07A3aEbb6e245",
    "0x505e71695E9bc45943c58adEC1650577BcA68fD9",
    "0x290275e3db66394C52272398959845170E4DCb88",
    "0xD5C08681719445A5Fdce2Bda98b341A49050d821",
    "0x082489A616aB4D46d1947eE3F912e080815b08DA",
];

fn is_binance(addr: &H160) -> bool {
    let s = format!("0x{}", addr.encode_hex::<String>());
    BINANCE_ADDRESSES.iter().any(|a| a.eq_ignore_ascii_case(&s))
}

pub async fn start() {
    // Use public polygon RPC; recommended to use your own provider for production.
    let transport = Http::new("https://polygon-rpc.com").expect("transport creation failed");
    let web3 = Web3::new(transport);

    // Prepare Transfer event signature topic
    // Transfer(address,address,uint256)
    let transfer_sig = web3::signing::keccak256(b"Transfer(address,address,uint256)");
    let transfer_topic = H256::from_slice(&transfer_sig);

    // Create filter for logs from POL contract with Transfer topic
    let pol_addr = H160::from_str(&POL_CONTRACT.replace("0x", "")).expect("Invalid POL address");

    let filter = FilterBuilder::default()
        .address(vec![pol_addr.into()])
        .topic0(web3::types::Topic::This(transfer_topic))
        .build();

    println!("Indexer: starting loop, polling logs every 5s...");

    let mut last_block = web3.eth().block_number().await.expect("get block number").as_u64();

    loop {
        // fetch latest block
        match web3.eth().block_number().await {
            Ok(latest) => {
                let latest_u = latest.as_u64();
                if latest_u > last_block {
                    // query logs between last_block+1 and latest
                    let logs = web3.eth().logs(web3::types::Filter {
                        from_block: Some(web3::types::BlockNumber::Number((last_block+1).into())),
                        to_block: Some(web3::types::BlockNumber::Number((latest).into())),
                        address: Some(vec![pol_addr.into()]),
                        topics: Some(vec![Some(vec![transfer_topic])]),
                        block_hash: None,
                        ..Default::default()
                    }).await;

                    if let Ok(entries) = logs {
                        for log in entries {
                            process_log(&web3, &log).await;
                        }
                    }
                    last_block = latest_u;
                }
            }
            Err(e) => {
                eprintln!("Error fetching block number: {:?}", e);
            }
        }

        sleep(Duration::from_secs(5)).await;
    }
}

async fn process_log(web3: &Web3<Http>, log: &Log) {
    // Transfer indexed topics: topic1 = from, topic2 = to, data = value (uint256)
    if log.topics.len() < 3 {
        return;
    }
    let from = H160::from_slice(&log.topics[1].as_bytes()[12..]);
    let to = H160::from_slice(&log.topics[2].as_bytes()[12..]);

    // decode value from data
    let value = if log.data.0.len() >= 32 {
        let mut b = [0u8;32];
        b.copy_from_slice(&log.data.0[0..32]);
        U256::from_big_endian(&b)
    } else {
        U256::zero()
    };

    let value_f = {
        // POL token uses 18 decimals usually; convert to float string for storage (simple)
        let decimals = 18u32;
        let denom = U256::from(10u64).pow(U256::from(decimals));
        let whole = value / denom;
        let frac = value % denom;
        let v = (whole.as_u128() as f64) + (frac.as_u128() as f64) / 10f64.powi(decimals as i32);
        format!("{}", v)
    };

    let tx_hash = format!("0x{}", hex::encode(log.transaction_hash.unwrap_or_default().0));

    // insert raw transfer
    let _ = db::insert_transfer(log.block_number.unwrap_or_default().as_u64() as i64, &tx_hash, &format!("0x{}", from.encode_hex::<String>()), &format!("0x{}", to.encode_hex::<String>()), &value_f, 0);

    // If either side is Binance, update netflow
    if is_binance(&to) {
        // incoming to Binance
        let _ = db::adjust_netflow("Binance", true, &value_f);
    }
    if is_binance(&from) {
        // outgoing from Binance
        let _ = db::adjust_netflow("Binance", false, &value_f);
    }

    println!("Processed transfer tx={} from=0x{} to=0x{} value={}", tx_hash, from.encode_hex::<String>(), to.encode_hex::<String>(), value_f);
}
