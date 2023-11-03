use tokio::net::UnixListener;
use tokio::net::UnixStream;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::unix::{ReadHalf, WriteHalf};

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

async fn handle_connection(mut stream: UnixStream) -> io::Result<()> {
    println!("Accepted connection from {:?}", stream.peer_addr());

    // Split the stream into read and write halves
    let (read_half, write_half) = stream.split();

    // Spawn separate tasks to handle reading and writing
    let reader_task = handle_read(read_half);
    let writer_task = handle_write(write_half);

    tokio::select! {
        _ = reader_task => (),
        _ = writer_task => (),
    };

    Ok(())
}

async fn handle_read(mut read_half: ReadHalf<'_>) {
    let mut buf = [0; 2048];
    loop {
        match read_half.read(&mut buf).await {
            Ok(n) => {
                if n == 0 {
                    // The client disconnected
                    break;
                }
                // Process the data read from the client here
                let data = &buf[..n];
                println!("Received: {:?}", String::from_utf8_lossy(data));
            }
            Err(e) => {
                eprintln!("Error reading from client: {}", e);
                break;
            }
        }
    }

    println!("Client disconnected from the uds");
}

async fn handle_write(mut write_half: WriteHalf<'_>) {
    // handle_write can send data to the client using the `WriteHalf`
    let data = b"Hello World!\n";

    if let Err(e) = write_half.write_all(data).await {
        eprintln!("Error writing to client: {}", e);
    }
}
