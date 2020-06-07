# RabbitMQ to Kafka

## Start Kafka

```
# Start Kafka
bin/zookeeper-server-start.sh config/zookeeper.properties
bin/kafka-server-start.sh config/server.properties

# Simple consumer
bin/kafka-console-consumer.sh --bootstrap-server localhost:9092 --from-beginning --topic hello
```
