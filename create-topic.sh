#!/bin/bash

bin/kafka-topics.sh --create --zookeeper localhost:2181 \
  --topic $1 \
  --replication-factor 1 \
  --partitions 1 \
  --config "cleanup.policy=compact" \
  --config "min.compaction.lag.ms=1"
