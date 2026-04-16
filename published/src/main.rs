use redis::AsyncCommands;
use tokio::io::{self, AsyncBufReadExt, BufReader};
mod clear;

#[tokio::main]
async fn main() -> redis::RedisResult<()> {
    clear::clear_terminal();
    
    let channel_name = "local_channel";
    let client = redis::Client::open("redis://127.0.0.1:6379")?;
    
    let mut con = client.get_multiplexed_tokio_connection().await?;
    
    println!("\nConnecté à '{}'\n", channel_name);
    println!("Tapez les mess ('exit' pour sortir):\n");

    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin).lines();

    while let Ok(Some(line)) = reader.next_line().await {
        let message = line.trim();

        if message.is_empty() { continue; }

        let _: () = con.publish(channel_name, message).await?;

        if message == "exit" {
            println!("\nFIN");
            break;
        }
    }

    Ok(())
}