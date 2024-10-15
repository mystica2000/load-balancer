use std::{io, net::{Ipv4Addr, SocketAddrV4}, sync::Arc};

use errors::{CustomError, Error};
use lb_pooling::pooling::{self, tcp::{Server, TcpConnectionPooling}};
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}, select, sync::{mpsc, Mutex}, try_join};
use yml_parser::parser::ServerConfig;



pub struct TcpConnector {
  pooling: Vec<TcpConnectionPooling>,
}

impl TcpConnector {

  pub fn add_servers(backend_servers: Vec<ServerConfig>) -> Result<TcpConnector, CustomError> {
    let mut temp = Vec::new();

    for server in backend_servers {
      let ip_addr = server.ip.parse::<Ipv4Addr>().map_err(|e| CustomError::Ipv4AddrParseError)?;
      let sock_addr = SocketAddrV4::new(ip_addr, server.port);
      temp.push(TcpConnectionPooling::new(sock_addr, server.name));
    }

    Ok(TcpConnector {
      pooling: temp,
    })
  }

  pub async fn initialize_pooling_layer(&self) {
    for pool in self.pooling.iter() {
      let _ = &pool.preheat_connections().await;
    }
  }

  pub async fn initialize_algorithm(&self, algorithm: String) {
    if algorithm == "round-robin" {

    }
  }

  pub async fn accept(self,tcp_listener: TcpListener) -> Result<(), Box<dyn std::error::Error>> {
    let (sender, mut recv) = mpsc::channel::<TcpStream>(1024);
    let count = Arc::new(Mutex::new(0));
    let pooling = Arc::new(self.pooling);
    tokio::spawn(async move {
        while let Some(stream) = recv.recv().await {
          println!("New connection received!");
          let count_clone = Arc::clone(&count);
          let pooling_clone = Arc::clone(&pooling);

          tokio::spawn(async move {
            println!("Processing new connection");
            let mut count_lock = count_clone.lock().await;
            *count_lock += 1;
            let current_count = *count_lock;
            drop(count_lock); // Explicitly drop the lock as soon as possible

            // TODO
          //   let pooling_instance = if let Some(rr) = round_robin_clone.as_ref() {
          //     rr.get_next_backend()
          // } else {
          //     None
          // };


          //   if let Some(pooling_instance) = pooling_instance {
          //     if let Err(e) = handle_connection(pooling_instance, stream, current_count).await {
          //       eprintln!("Failed Connection Result: {}", e);
          //     }
          //   }
          //   else {
          //   eprintln!("No pooling instances available");
          //   }
          });

          //  // Choose a pooling instance (for example, round-robin)
          //  let pooling_index = current_count % self_clone.len();
          //  let mut pooling_instance = &self_clone[pooling_index];
      }
    });


    loop {
      let (stream, _) = tcp_listener.accept().await?;
      sender.send(stream).await?;
    }

  }

}

  async fn handle_connection(stream: &TcpConnectionPooling,mut client_stream: TcpStream, mut count: usize) -> Result<(), Box<dyn std::error::Error>> {
  let index = count % 2;

  //let mut stream = pool.pooling.get(index).unwrap();
  println!("TESTING?!!!!!!! {}", count);
  let mut server_stream = stream.create_new_connection(count as u8).await;


  println!("{:?}", client_stream);

  let mut server_buf = [0;1024];
  let mut client_buf = [0;1024];

  let mut server_stream_temp = server_stream.connection;


  // let (mut ri, mut wi) = client_stream.split();
  // let (mut ro, mut wo) = server_stream_temp.split();

  // let client_to_server = io::copy(&mut ri, &mut wo);
  // let server_to_client = io::copy(&mut ro, &mut wi);

  // let (bytes_tx, bytes_rx) = try_join(client_to_server, server_to_client).await?;

  loop {
      tokio::select! {
        result = client_stream.read(&mut client_buf) => {
          match result? {
              0 => return Ok(()), // Connection closed
              n => {
                server_stream_temp.write_all(&client_buf[..n]).await?;
                //server_stream_temp.shutdown();
              }
          }
      }
      result = server_stream_temp.read(&mut server_buf) => {
          match result? {
              0 => return Ok(()), // Connection closed
              n => client_stream.write_all(&server_buf[..n]).await?
          }
      }
      }
  }

  Ok(())
  }



// pub async fn initialize_pooling(&self) -> Result<(), Error> {
//   for pool in &self.pooling {
//     pool.preheat_connections().await;
//   }

//   Ok(())
// }