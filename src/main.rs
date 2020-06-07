use amiquip::{Connection, ConsumerMessage, ConsumerOptions, QueueDeclareOptions, Result};
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::message::OwnedHeaders;

fn produce(producer: &FutureProducer, topic_name: &str, key: &str, message: &str) {

    producer
        .send(
            FutureRecord::to(topic_name)
                .payload(&format!("{}", message))
                .key(&format!("{}", key))
                .headers(OwnedHeaders::new().add("header_key", "header_value")),
            0,
        );
}

fn main() -> Result<()> {
    // Open connection.
    let brokers = "localhost:9092";
    let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672")?;

    // Open a channel - None says let the library choose the channel ID.
    let channel = connection.open_channel(None)?;

    // Declare the "hello" queue.
    let queue = channel.queue_declare("hello", QueueDeclareOptions::default())?;

    let producer: &FutureProducer = &ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

    // Start a consumer.
    let consumer = queue.consume(ConsumerOptions::default())?;
    println!("Waiting for messages. Press Ctrl-C to exit.");

    for (i, message) in consumer.receiver().iter().enumerate() {
        match message {
            ConsumerMessage::Delivery(delivery) => {
                let body = String::from_utf8_lossy(&delivery.body);
                println!("({:>3}) Received [{}]", i, body);
                produce(producer, "hello", "message", &format!("{}", body));
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
