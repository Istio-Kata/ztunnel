use tokio::net::UnixListener;
use tokio::net::UnixStream;

const KATA_UDS_DIR: &str = "/run/var/kata.sock";

pub struct UDSServer {
    listener: UnixListener,
}

impl UDSServer {
    pub async fn new() -> Result<Self, std::io::Error> {
        let listener = UnixListener::bind(KATA_UDS_DIR)?;
        Ok(UDSServer { listener })
    }

    pub async fn spawn(self) {
        while let Ok((stream, _)) = self.listener.accept().await {
            // Spawn a new async task to handle each incoming connection.
            tokio::spawn(handle_connection(stream));
        }
    }
}

async fn handle_connection(stream: UnixStream) {
    // Implement your logic to handle the client connection here.
    // You can read and write data to/from the stream.
    println!("Accepted connection from {:?}", stream.peer_addr());
}
