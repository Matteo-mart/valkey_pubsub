use futures_util::StreamExt;
mod clear;

#[tokio::main]
async fn main() -> redis::RedisResult<()> {

    clear::clear_terminal();
    
    let channel_name = "local_channel";
    let client = redis::Client::open("redis://127.0.0.1:6379")?;
    let mut pubsub = client.get_async_pubsub().await?;
    
    pubsub.subscribe(channel_name).await?;
    
    println!("\nConnecté à '{}'\n", channel_name);

    let mut stream = pubsub.on_message();

    while let Some(msg) = stream.next().await {
        let playload: String = msg.get_payload()?;
        
        println!("Reçu sur {}: '{}'", channel_name, playload);

        if playload == "exit" { 
            println!("\nFIN");
            break; 
        }
    }

    Ok(())
}