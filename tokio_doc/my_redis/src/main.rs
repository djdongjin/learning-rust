use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        // ignore socketAddr returned by `accept` for now.
        let (socket, _) = listener.accept().await.unwrap();
        process(socket).await;
    }
}

async fn process(socket: TcpStream) {
    let mut conn = Connection::new(socket);
    if let Some(frame) = conn.read_frame().await.unwrap() {
        println!("GOT: {:?}", frame);

        // respond error for now before adding logic.
        let resp = Frame::Error("unimplemented".to_string());
        conn.write_frame(&resp).await.unwrap();
    }
}