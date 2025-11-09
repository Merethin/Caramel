use lapin::{
    options::{BasicAckOptions, BasicConsumeOptions, ExchangeDeclareOptions, QueueBindOptions, QueueDeclareOptions}, 
    types::FieldTable
};
use futures_util::StreamExt;
use log::error;

use crate::types::akari::Event;

pub async fn create_consumer(
    channel: &lapin::Channel, 
    exchange_name: &str,
    keys: Option<Vec<&str>>
) -> Result<lapin::Consumer, Box<dyn std::error::Error>> {
    channel.exchange_declare(
        exchange_name,
        lapin::ExchangeKind::Topic,
        ExchangeDeclareOptions::default(),
        FieldTable::default()
    ).await?;

    let queue = channel.queue_declare(
        "", QueueDeclareOptions {
                exclusive: true,
                auto_delete: true,
                ..Default::default()
            }, FieldTable::default()
    ).await?;

    if let Some(keys) = keys {
        for key in keys {
            channel.queue_bind(
                queue.name().as_str(), exchange_name, key, 
                QueueBindOptions::default(), FieldTable::default()
            ).await?;
        }
    } else {
        channel.queue_bind(
            queue.name().as_str(), exchange_name, "*", 
            QueueBindOptions::default(), FieldTable::default()
        ).await?;
    }

    Ok(channel.basic_consume(
        queue.name().as_str(), "consumer", 
        BasicConsumeOptions::default(), FieldTable::default()
    ).await?)
}

pub async fn consume(consumer: &mut lapin::Consumer) -> Option<Event> {
    loop {
        match consumer.next().await {
            Some(Ok(delivery)) => {
                if let Err(err) = delivery.ack(BasicAckOptions::default()).await {
                    error!("error while acknowledging delivery: {}", err);
                }

                let event = str::from_utf8(&delivery.data).ok().and_then(
                    |v| serde_json::from_str(v).ok()
                );

                if event.is_some() {
                    return event;
                }
            },
            Some(Err(err)) => {
                error!("error from consumer: {}", err);
                return None;
            },
            None => { return None; }
        }
    }
}