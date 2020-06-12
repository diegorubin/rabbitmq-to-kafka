use amiquip::{
    AmqpProperties, AmqpValue, Connection, ConsumerMessage, ConsumerOptions,
    ExchangeDeclareOptions, ExchangeType, FieldTable, QueueDeclareOptions, Result,
};
use rdkafka::config::ClientConfig;
use rdkafka::message::OwnedHeaders;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::env;

fn produce(producer: &FutureProducer, topic_name: &str, key: &str, message: &str) {
    producer.send(
        FutureRecord::to(topic_name)
            .payload(&format!("{}", message))
            .key(&format!("{}", key))
            .headers(OwnedHeaders::new().add("header_key", "header_value")),
        0,
    );
}

fn main() -> Result<()> {
    let mut brokers = String::from("localhost:9092");
    match env::var("KAFKA_BROKER") {
        Ok(value) => brokers = value.to_owned(),
        Err(e) => println!("Couldn't read KAFKA_BROKER ({})", e),
    };

    let mut topic = String::from("events");
    match env::var("KAFKA_TOPIC") {
        Ok(value) => topic = value.to_owned(),
        Err(e) => println!("Couldn't read KAFKA_TOPIC ({})", e),
    };

    let mut rabbitmq_url = String::from("amqp://guest:guest@localhost:5672");
    match env::var("RABBITMQ_URL") {
        Ok(value) => rabbitmq_url = value.to_owned(),
        Err(e) => println!("Couldn't read RABBITMQ_URL ({})", e),
    };

    let mut rabbitmq_exchange = String::from("exchange");
    match env::var("RABBITMQ_EXCHANGE") {
        Ok(value) => rabbitmq_exchange = value.to_owned(),
        Err(e) => println!("Couldn't read RABBITMQ_EXCHANGE ({})", e),
    };

    let mut rabbitmq_queue = String::from("queue");
    match env::var("RABBITMQ_QUEUE") {
        Ok(value) => rabbitmq_queue = value.to_owned(),
        Err(e) => println!("Couldn't read RABBITMQ_QUEUE ({})", e),
    };

    let mut connection = Connection::insecure_open(&*rabbitmq_url)?;
    let channel = connection.open_channel(None)?;

    let exchange = channel.exchange_declare(
        ExchangeType::Direct,
        rabbitmq_exchange,
        ExchangeDeclareOptions::default(),
    )?;
    let queue = channel.queue_declare(rabbitmq_queue, QueueDeclareOptions::default())?;

    channel.queue_bind(queue.name(), exchange.name(), "", FieldTable::default())?;

    let producer: &FutureProducer = &ClientConfig::new()
        .set("bootstrap.servers", &*brokers)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

    let consumer = queue.consume(ConsumerOptions::default())?;
    println!("Waiting for messages. Press Ctrl-C to exit.");

    for (i, message) in consumer.receiver().iter().enumerate() {
        match message {
            ConsumerMessage::Delivery(delivery) => {
                let properties: &AmqpProperties = &delivery.properties;
                let mut key: String = String::from("unknown");

                match properties.headers() {
                    Some(headers) => {
                        if headers.contains_key("key") {
                            let content_key = &headers["key"];
                            match content_key {
                                AmqpValue::LongString(content) => key = content.to_string(),
                                _ => println!("invalid key type"),
                            }
                        }
                    }
                    None => {}
                }

                let body = String::from_utf8_lossy(&delivery.body);
                println!("({:>3}) Received {}:[{}]", i, key, body);
                produce(producer, &*topic, &*key, &format!("{}", body));
                consumer.ack(delivery)?;
            }
            other => {
                println!("Consumer ended: {:?}", other);
                break;
            }
        }
    }

    connection.close()
}
