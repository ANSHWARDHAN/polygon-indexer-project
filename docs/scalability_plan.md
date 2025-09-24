# Scalability Plan (short)

1. Move from SQLite to a production DB:
   - Use PostgreSQL (or ClickHouse for analytics) to handle higher write throughput and concurrent readers.
2. Decouple ingestion and processing:
   - Ingestion service: cheap workers that read blocks and push raw logs to a message queue (Kafka/RabbitMQ).
   - Processor workers: consume from queue, decode events, and update aggregates.
3. Partitioning:
   - Partition by token and by exchange label to allow independent reprocessing.
4. Backfill strategy:
   - Use snapshot + incremental replay with checkpoints if backfilling is required in future.
5. Monitoring & Observability:
   - Add metrics (Prometheus) and tracing (Jaeger).
6. High-availability RPC:
   - Use multiple RPC providers (Alchemy/QuickNode/Polygon RPC) with failover and rate-limit handling.
