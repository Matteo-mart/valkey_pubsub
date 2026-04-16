use redis::Client;

pub fn setup_redis() -> redis::RedisResult<(Client, String)> {
    let channel_name = "local_channel".to_string();
    let client = Client::open("redis://127.0.0.1:6379")?;
    
    Ok((client, channel_name))
}