use redis::AsyncCommands;
use tokio::io::{self, AsyncBufReadExt, BufReader};
mod setup_redis;

#[tokio::main]
async fn main() -> redis::RedisResult<()> {
   
    let (client, channel_name) = setup_redis::setup_redis()?;
    let mut con = client.get_multiplexed_tokio_connection().await?;
    
    println!("\nConnecté à '{}'\n", channel_name);
    println!("Tapez les messages ('exit' pour sortir):\n");

    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin).lines();

    while let Ok(Some(line)) = reader.next_line().await {
        let message = line.trim();

        if message.is_empty() { continue; }

        let _: () = con.publish(&channel_name, message).await?;
        
        
        if message == "exit" {
            println!("\nFIN");
            break;
        }
    }

    Ok(())
}