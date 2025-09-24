use rusqlite::{params, Connection, Result};
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref DB_PATH: String = "indexer.db".to_string();
    static ref DB_CONN: Mutex<Connection> = {
        let conn = Connection::open(&*DB_PATH).expect("Failed to open DB");
        Mutex::new(conn)
    };
}

pub fn init() -> Result<()> {
    let conn = DB_CONN.lock().unwrap();
    conn.execute_batch(include_str!("../schema.sql"))?;
    Ok(())
}

pub fn insert_transfer(block_number: i64, tx_hash: &str, from: &str, to: &str, amount: &str, timestamp: i64) -> Result<()> {
    let conn = DB_CONN.lock().unwrap();
    conn.execute(
        "INSERT INTO transfers (block_number, tx_hash, from_address, to_address, amount, timestamp) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![block_number, tx_hash, from, to, amount, timestamp],
    )?;
    Ok(())
}

pub fn adjust_netflow(exchange: &str, incoming: bool, amount: &str) -> Result<()> {
    let conn = DB_CONN.lock().unwrap();

    // Try fetch existing
    let mut stmt = conn.prepare("SELECT id, cumulative_in, cumulative_out FROM net_flows WHERE exchange = ?1")?;
    let mut rows = stmt.query(params![exchange])?;
    if let Some(row) = rows.next()? {
        let id: i64 = row.get(0)?;
        let cum_in: String = row.get(1)?;
        let cum_out: String = row.get(2)?;
        // store as decimal string; for simplicity we'll store as strings and do arithmetic in SQL using CAST to REAL
        if incoming {
            conn.execute(
                "UPDATE net_flows SET cumulative_in = (CAST(cumulative_in AS REAL) + CAST(?1 AS REAL))::TEXT, net_flow = (CAST(cumulative_in AS REAL) + CAST(?1 AS REAL) - CAST(cumulative_out AS REAL))::TEXT, last_updated = CURRENT_TIMESTAMP WHERE id = ?2",
                params![amount, id],
            )?;
        } else {
            conn.execute(
                "UPDATE net_flows SET cumulative_out = (CAST(cumulative_out AS REAL) + CAST(?1 AS REAL))::TEXT, net_flow = (CAST(cumulative_in AS REAL) - (CAST(cumulative_out AS REAL) + CAST(?1 AS REAL)))::TEXT, last_updated = CURRENT_TIMESTAMP WHERE id = ?2",
                params![amount, id],
            )?;
        }
    } else {
        // Insert new row
        let (ci, co) = if incoming { (amount.to_string(), "0".to_string()) } else { ("0".to_string(), amount.to_string()) };
        let net: String = format!("{}", ci.parse::<f64>().unwrap_or(0.0) - co.parse::<f64>().unwrap_or(0.0));
        conn.execute(
            "INSERT INTO net_flows (exchange, cumulative_in, cumulative_out, net_flow) VALUES (?1, ?2, ?3, ?4)",
            params![exchange, ci, co, net],
        )?;
    }
    Ok(())
}

pub fn get_netflows() -> Result<Vec<(String, String, String, String)>> {
    let conn = DB_CONN.lock().unwrap();
    let mut stmt = conn.prepare("SELECT exchange, cumulative_in, cumulative_out, net_flow FROM net_flows")?;
    let rows = stmt.query_map([], |row| {
        Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
    })?;

    let mut out = Vec::new();
    for r in rows {
        out.push(r?);
    }
    Ok(out)
}
