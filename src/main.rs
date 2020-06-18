use amiquip::{
    Connection, ConsumerMessage, ConsumerOptions, ExchangeDeclareOptions, ExchangeType, FieldTable,
    QueueDeclareOptions, Result,
};
use libloading::Library;
use rdkafka::config::ClientConfig;
use rdkafka::message::OwnedHeaders;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::env;

struct TransformResult {
    key: String,
    body: String,
}

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
    let mut transform_lib_path = String::from("transform-lib/target/debug/libtransform_lib.so");
    match env::var("TRANSFORM_LIB_PATH") {
        Ok(value) => transform_lib_path = value.to_owned(),
        Err(e) => println!("Couldn't read TRANSFORM_LIB_PATH ({})", e),
    };

    let lib = Library::new(transform_lib_path).expect("library not found");

    let transform: libloading::Symbol<fn(message: String) -> Result<TransformResult>> =
        unsafe { lib.get(b"transform") }.expect("error on load symbol");

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
                let body = String::from_utf8_lossy(&delivery.body);
                match transform(format!("{}", body)) {
                    Ok(transformed) => {
                        println!(
                            "({:>3}) Received {}:[{}]",
                            i, transformed.key, transformed.body
                        );
                        produce(producer, &*topic, &*transformed.key, &*transformed.body);
                        consumer.ack(delivery)?;
                    }
                    Err(e) => println!("Error on transform message {}", e),
                }
            }
            other => {
                println!("Consumer ended: {:?}", other);
                break;
            }
        }
    }

    connection.close()
}
