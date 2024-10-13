use std::{env, fs::File};

use lb_core::initialize_load_balancer;
use yml_parser::yml_parser;

#[tokio::main()]
async fn main() ->  Result<(), Box<dyn std::error::Error>> {

    match env::current_dir() {
      Ok(path) => println!("PWD {:?}",path),
      Err(e) => eprintln!("Error {}",e)
    }

    match File::open("lb-app/test/sample.yml") {
      Ok(file) => {

       let load_balancer_config = yml_parser(file).map_err(|e| e);

       let _ = initialize_load_balancer(load_balancer_config.unwrap()).await;
      },
      Err(e) => {
        eprintln!("Error Opening the file: {}",e);
      }
    }

    Ok(())
}
