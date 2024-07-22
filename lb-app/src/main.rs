use std::{env, fs::{File}, io};

use lb_core::initialize_load_balancer;
use yml_parser::{yml_parser};

fn main() -> io::Result<()> {
    println!("Hello, world!");


    match env::current_dir() {
      Ok(path) => println!("PWD {:?}",path),
      Err(e) => eprintln!("Error {}",e)
    }

    match File::open("lb-app/test/sample.yml") {
      Ok(file) => {

       let load_balancer_config = yml_parser(file).map_err(|e| e);
       println!("{:?}",load_balancer_config);

       initialize_load_balancer(load_balancer_config.unwrap());
       Ok(())
      },
      Err(e) => {
        eprintln!("Error Opening the file: {}",e);
        Err(e)
      }
    }



}
