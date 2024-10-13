use std::{io::{self, ErrorKind}, net::{Ipv4Addr, SocketAddr, SocketAddrV4}, time::Duration};

use errors::{CustomError, Error};
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}, select, sync::mpsc};

use tokio::net::TcpSocket;

pub struct TcpListenerNode {
  ip_addr: Ipv4Addr,
  port: u16
}

impl TcpListenerNode {
  // parses ip address and port
  pub fn new(port: u16) -> TcpListenerNode {
    Self{
        ip_addr: Ipv4Addr::new(0, 0, 0, 0),
        port: port
    }
  }

  // start connection
  pub async fn bind(tcp_listener: TcpListenerNode) -> Result<TcpListener, Error> {

    let mut retry_counter = 0;

    loop {
    let socket_addr = SocketAddrV4::new(tcp_listener.ip_addr, tcp_listener.port);
    let listener_socket = TcpSocket::new_v4().unwrap();

    match listener_socket.bind(SocketAddr::V4(socket_addr)) {
      Ok(()) => {
        return listener_socket.listen(10).map_err(|e| Error::Io(io::Error::new(e.kind(), "listen() on tcp failed")));
      },
      Err(e) => {
        if e.kind() != ErrorKind::AddrInUse {
          return Err(Error::Io(io::Error::new(e.kind(), "bind() failed")));
        }

        retry_counter += 1;
        if retry_counter >= 3 {
          return Err(Error::Io(io::Error::new(ErrorKind::AddrInUse,"bind() failed - address in use")));
        }
        tokio::time::sleep(Duration::from_millis(1000)).await;
      }
    }
    }
  }

}


// TODO
// [] Make connect configurable!
// [] Connection Pooling for the server
// [] Error handling
// [] Add to see how many bytes transmitted etc

// in the end, configurable, error handling, connection pooling