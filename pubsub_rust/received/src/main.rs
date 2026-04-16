use futures_util::StreamExt;
mod setup_redis;

#[tokio::main]
async fn main() -> redis::RedisResult<()> {

    let (client, channel_name) = setup_redis::setup_redis()?;

    let mut pubsub = client.get_async_pubsub().await?;
    pubsub.subscribe(&channel_name).await?;
    
    println!("\nConnecté à '{}'\n", channel_name);

    let mut stream = pubsub.on_message();

    while let Some(msg) = stream.next().await {
        let payload: String = msg.get_payload()?;
        
        println!("Message reçu sur {}: '{}'", channel_name, payload);

        if payload == "exit" { 
            println!("\nFIN");
            break; 
        }
    }

    Ok(())
}