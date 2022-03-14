use std::thread;
use std::time::Duration;

use futures::executor::block_on;

async fn second_greet() {
    println!("Hello again, world");
}

async fn greet() {
    thread::sleep(Duration::from_secs(5));
    println!("Hello, world");
}

async fn async_main() {
    // greet().await;
    // second_greet().await;

    futures::join!(greet(), second_greet());
}

fn main() {
    block_on(async_main());
}
