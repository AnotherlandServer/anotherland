use std::collections::{LinkedList, VecDeque};

use std::time::Duration;
use tokio::{net::{TcpListener, ToSocketAddrs, TcpStream}, io::{self, AsyncWriteExt}, task::JoinHandle};

pub struct QueueServer {  
    listen_thread: JoinHandle<()>,
    client_thread: JoinHandle<()>,
}

impl QueueServer {
    pub async fn bind_server<A: ToSocketAddrs>(addr: A) -> io::Result<QueueServer> {
        let listener = TcpListener::bind(addr).await?;
        let (tx, mut rx) = tokio::sync::mpsc::channel::<TcpStream>(10);

        Ok(QueueServer {
            listen_thread: tokio::spawn(async move {
                loop {
                    if let Ok((client, _)) = listener.accept().await {
                        tx.send(client).await;
                    }

                }
            }),
            client_thread: tokio::spawn(async move {
                let mut clients: VecDeque<TcpStream> = VecDeque::new();

                loop {
                    // Check for new clients
                    if let Ok(client) = rx.try_recv() {
                        clients.push_back(client);
                    }

                    for c in clients.iter_mut() {
                        // Send client hello / keepalive
                        let _ = c.write_u8(0x01u8);
                    }

                    std::thread::sleep(Duration::from_secs(1));
                }
            })
        })
    }
}
