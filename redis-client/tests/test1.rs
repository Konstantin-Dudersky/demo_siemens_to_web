use redis::AsyncCommands;

use redis_client::RedisHash;

async fn do_something() -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_async_connection().await?;
    let val: u32 = con.get("my_key1").await?;
    con.set("my_key", val + 1).await?;

    /* do something here */

    Ok(())
}
