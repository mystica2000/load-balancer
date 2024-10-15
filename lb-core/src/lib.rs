use errors::Error;
use yml_parser::parser::LoadBalancerConfig;
use listener::tcp::TcpListenerNode;
use connector::tcp::TcpConnector;

mod listener;
mod connector;
mod algorithms;

pub async fn initialize_load_balancer(lb_config: LoadBalancerConfig) -> Result<(), Error> {
 // println!("Initialize Load Balancer Via Config : {:?} ", lb_config);
  println!("\n Initializing {}",lb_config.name);

  // initialize connector thread - connect to couple of the servers

  // initialize listener thread -
  match lb_config.lb_type.as_str() {
    "application" => {
      println!("THIS WILL WORK!!")
    },
    "transport" => {
      println!("THIS WILL WORK!! TRANSPORT");
      // setting up connector
      let tcp_connector = TcpConnector::add_servers(lb_config.backend_servers).map_err(|e| { eprintln!("{}",e); e })?;
      tcp_connector.initialize_pooling_layer().await;
      tcp_connector.initialize_algorithm(lb_config.algorithm);

      // setting up listener
      let tcp_listener_node = TcpListenerNode::new(lb_config.listener.port);
      let tcp_listener = TcpListenerNode::bind(tcp_listener_node)
                      .await.map_err(|e| {
                         eprintln!("{}",e);
                         e
                      })?;


      tcp_connector.accept(tcp_listener).await;
    }
    _ => {
      println!("Error");
    }
  };

  Ok(())

  // background health check initialize
}

#[cfg(test)]
mod tests {
    use super::*;

}
