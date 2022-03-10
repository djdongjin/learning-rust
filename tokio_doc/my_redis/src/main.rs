use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};
use std::{sync::{Arc, Mutex}, collections::HashMap};
use bytes::Bytes;

// `Arc` provides thread-safe reference counter.
// `Mutex` provides locking.
type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    println!("Listening");

    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        // ignore socketAddr returned by `accept` for now.
        let (socket, _) = listener.accept().await.unwrap();

        let db = db.clone();
        // tokio::spawn accepts a `async` block and returns a `JoinHandle`.
        // if there are values returned, call `await` on the `JoinHandle`.
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

async fn process(socket: TcpStream, db: Db) {
    use mini_redis::Command::{self, Get, Set};

    let mut conn = Connection::new(socket);

    // use `while let` so that more than one command can be accepted in a connection.
    while let Some(frame) = conn.read_frame().await.unwrap() {
        let resp = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            },
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }
            },
            cmd => panic!("unimplemented {:?}", cmd),
        };

        conn.write_frame(&resp).await.unwrap();
    }
}