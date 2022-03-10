use tokio::io;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let (mut reader, mut writer) = socket.split();

            if io::copy(&mut reader, &mut writer).await.is_err() {
                eprintln!("failed to copy");
            }
        });
    }
}
