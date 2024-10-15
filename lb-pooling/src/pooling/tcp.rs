use std::{net::SocketAddrV4, sync::Arc, time::{Duration, Instant}};
use errors::{CustomError, Error};
use tokio::{net::TcpStream, sync::{Mutex, RwLock}, time::timeout};

pub struct PooledConnection {
  id: u8,
  // connection: Arc<Mutex<TcpStream>>,
  pub connection: TcpStream,
  last_used: Instant
}


impl PooledConnection {
  pub fn get_instant() -> Instant {
    Instant::now()
  }
}

pub struct Server {
  pub ip_addr: SocketAddrV4,
  pub name: String
}


pub struct TcpConnectionPooling {
  initial_pool_size: u8,
  max_size: u8,
  max_idle_time: Duration, // for example: if a connection is idle for a long time, then unlock() the server;
  connections: RwLock<Vec<PooledConnection>>,
  server_address: Server,
}

impl TcpConnectionPooling {
  pub fn new(ip_addr: SocketAddrV4, name: String) -> TcpConnectionPooling {
    Self {
      initial_pool_size: 5,
      max_size: 10,
      max_idle_time: Duration::from_millis(1000), // 10 sec
      connections: RwLock::new(Vec::with_capacity(10)),
      server_address: Server {ip_addr, name}
    }
  }

  pub async fn preheat_connections(&self) -> Result<(), Error> {

    let mut connections: tokio::sync::RwLockWriteGuard<Vec<PooledConnection>> = self.connections.write().await;

    for i in 0..self.initial_pool_size {
      println!("CREATING CONNECTION!! TO BACKEND :)");
      let connection = self.create_new_connection(i).await;
      connections.push(connection);
    }
    // let mut connections = self.connections.lock().await;
    println!("PREHEATINGGGG");

   // let mut connections = self.connections.lock().map_err(|e| e.into());

    Ok(())
  }

  pub async fn create_new_connection(&self, id:u8) -> PooledConnection  {
    let tcp_connection = TcpStream::connect(self.server_address.ip_addr).await.unwrap();

    // Arc::new(Mutex::new(tcp_connection))
    PooledConnection { connection: tcp_connection, last_used: PooledConnection::get_instant(), id: id}

  // let duration = Duration::from_secs(5);
  // match timeout(duration, TcpStream::connect(&self.server_address.ip_addr)).await {
  //     Ok(Ok(stream)) => {
  //         println!("Connected to the server");
  //         Some(PooledConnection {
  //             connection: stream,
  //             last_used: PooledConnection::get_instant(),
  //             id,
  //         })
  //     }
  //     Ok(Err(e)) => {
  //         eprintln!("Failed to connect: {:?}", e);
  //         None
  //     }
  //     Err(_) => {
  //         eprintln!("Connection attempt timed out");
  //         None
  //     }
  // }
  }

  // pub async fn get_connection(&self, count:usize) -> Result<TcpStream, Error> {
  //   // let tcp_connections = self.connections.read().await;

  //   // if let Some(pool_connection) = tcp_connections.get(count) {
  //   //   Ok(Arc::clone(&pool_connection.connection))
  //   // } else {
  //   // println!("WOW");
  //     let connection = self.create_new_connection(count as u8).await;
  //     Ok(connection.connection)
  //     //Err(Error::Custom(errors::CustomError::TcpOnListenError))
  //   //}
  // }

  // pub async fn release_connection(&self, connection: PooledConnection) {
  //   let mut test = connection.connection.lock().await;
  //   test.shutdown();
  // }
}

// /**
//  * It will contain pooling layer for the tcp connection
//  * - max_size having number of connections it supports
//  * ok so, set up the pooling layer by passing backend_address[],
//  * for each backend, it creates a specific number of connections to the server (maintains it) - idk what to call that speicific number tho
//  * a request comes in, select from pooling connection and make it lock()
//  * if no pooling connection is available, then create a new pooling < max_size: if no, error or wait for a while (use like spin lock lol)
//  * after serving, unlock() the pooling connection!
//  * preheat the connections for every 5 mins or something (estimated!)
//  */