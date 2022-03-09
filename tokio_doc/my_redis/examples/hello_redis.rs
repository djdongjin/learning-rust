use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {

    // 1. set a key, then get the value.
    let mut client = client::connect("127.0.0.1:6379").await?;
    client.set("hello", "world".into()).await?;
    let result = client.get("hello").await?;
    println!("got value from the server; result={:?}", result);

    // 2. calling `async fn` returns a value representing the operation.
    // Use `.await` operator to execute the function.
    demo_async().await;

    Ok(())
}

async fn say_world() {
    println!("world");
}

async fn demo_async() {
    let op = say_world();
    println!("hello"); 
    op.await;   // print hello first and then world.
}