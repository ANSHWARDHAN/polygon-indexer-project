# polygon-indexer-project


Real-time Polygon Blockchain Data Indexer 
1. Introduction 
This document outlines the scope, requirements, and proposed architecture for 
developing a real-time blockchain data indexing system on the Polygon network. The 
primary goal is to process raw blockchain data, specifically focusing on the POL 
token, and calculate cumulative net-flows to the Binance exchange. This system will 
utilize SQLite or any other database for data storage and be designed with future 
scalability in mind to support additional exchanges. 
2. Key Metrics & Deliverables 
Metric: 
● Cumulative Net-Flows to Binance: The sum of POL tokens transferred to 
Binance addresses minus the sum of POL tokens transferred from Binance 
addresses, aggregated over time. 
Deliverables: 
1. Schema Design & Implementation: A well-defined database schema optimized 
for efficient storage and retrieval of raw transaction data and processed net-flow 
data. 
2. Indexing Logic: A Rust application capable of connecting to the Polygon network 
via RPC, listening for new blocks in real-time, fetching transaction details, 
identifying POL token transfers, and storing the relevant raw data in the designed 
schema. 
3. Data Transformation Flow: Clear demonstration (through code and internal 
documentation) of how raw blockchain data is processed and structured into the 
f
 inal end-user table (cumulative net-flows). 
4. Query Mechanism: A simple interface (e.g., a command-line tool or a basic HTTP 
API endpoint) to retrieve the current cumulative net-flow data. 
5. Scalability Strategy: A documented plan and architectural considerations for 
easily extending the system to support indexing and net-flow calculations for 
multiple exchanges in the future. 
6. No Backfill: The system should focus on real-time indexing from the point of 
deployment; historical transaction backfilling is not required for this phase. 
7. Presentation: A clear presentation of the implemented schema and the overall 
system architecture. 
3. Technology Stack 
● Blockchain: Polygon Network 
● Token: POL  
● Database: SQLite or any other database 
● Programming Language: Rust 
● Data Sources: 
○ Polygon RPC Access (for blockchain data) 
○ Provided 
Binance labels (list of Ethereum addresses associated with Binance) Binance 
addresses, please use the following: 
● 0xF977814e90dA44bFA03b6295A0616a897441aceC 
● 0xe7804c37c13166fF0b37F5aE0BB07A3aEbb6e245 
● 0x505e71695E9bc45943c58adEC1650577BcA68fD9 
● 0x290275e3db66394C52272398959845170E4DCb88 
● 0xD5C08681719445A5Fdce2Bda98b341A49050d821 
● 0x082489A616aB4D46d1947eE3F912e080815b08DA 
Instructions for task submission: 
1. Complete the task and submit your work in a Git repository. 
2. Include a README file with all necessary instructions, explanations, and project 
details for easy review. 
3. Ensure the repository is set to public and share the link with us once your final 
submission is pushed.