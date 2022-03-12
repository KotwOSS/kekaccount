use std::io::Error;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::database::PgPool;

use crate::colors;

pub async fn main(pool: &PgPool, address: String, port: u16) -> Result<(), Error> {
    println!("{}START{} tcp on {}{}:{}", colors::LIGHT_BLUE, colors::RESET, colors::ORANGE,  address, port);

    match TcpListener::bind(format!("{}:{}", address, port)).await {
        Ok(tcp_listener)=>{
            accept_loop(tcp_listener, pool).await;
            println!("{}STOP{} tcp", colors::ORANGE, colors::RESET);
            return Ok(());
        }
        Err(error)=>{
            println!("{}ERROR{} while binding tcp: {}", colors::RED, colors::RESET, error);
            return Err(error);
        }
    }
}

async fn accept_loop(tcp_listener: TcpListener, _pool: &PgPool) {
    while let Ok((mut tcp_stream, _socket_addr)) = tcp_listener.accept().await {
        tokio::spawn(async move {
            println!("CONNECTED: {}", _socket_addr);
            
            let mut buf = [0; 1024];

            let amount = match tcp_stream.read(&mut buf).await {
                // socket closed
                Ok(n) if n == 0 => return,
                Ok(n) => n,
                Err(e) => {
                    eprintln!("failed to read from socket; err = {:?}", e);
                    return;
                }
            };

            if amount != 256 {
                println!("TOKEN INVALID: {}", amount);
                tcp_stream.write(b"TOKEN_INVALID").await
                    .expect("Socket write error:");
                drop(tcp_stream);
                return;
            }
        });
    }
}