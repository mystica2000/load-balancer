use std::{fs::File, io::{self, BufRead}};
mod parser;

#[derive(Debug)]
struct Server {
  name: String,
  ip: String,
  port: u16 // for port 0-65,535
}

#[derive(Debug)]
pub struct LoadBalancer {
  name: String,
  lb_type: String,
  backend_servers: Vec<Server>,
  algorithm: String
}

impl LoadBalancer {
  fn new() -> Self{
    Self {
      name: String::new(),
      lb_type: String::new(),
      backend_servers: Vec::new(),
      algorithm: String::new()
    }
  }
}

// enum LoadBalancerType {
//   Application = "application",
//   Network = "network"
// first if # is present, then get the index and skip after
// match it with keyword ":" before.. then you will know
// the following have to be one of the name, type,listeners,algorithms

// if listeners then next have to be protocol, port
// if backend servers, then next have to be name, ip, port as array tada over!
// }


pub fn yml_parser(file: File) -> Result<LoadBalancer, io::Error> {

  let load_balancer = LoadBalancer::new();

  let reader = io::BufReader::new(file);

  for line in reader.lines() {

    match line {
      Ok(mut str) => {
        str = str.trim().to_string();

        if str.is_empty() || str.starts_with("#") {
          continue;
        }

        let processed_line = parser::preprocess_by_line(&str)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        if processed_line.is_empty() {
          continue;
        }

        if processed_line.starts_with(":") {
          io::Error::new(io::ErrorKind::InvalidData, "Invalid Syntax: Unexpected : ");
        }

      }
      Err(e) => {
        eprintln!("Error Reading Line: {}",e);
        io::Error::new(io::ErrorKind::InvalidData, e);
      }
    }
  }

  Ok(load_balancer)
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     //#[test]
//     // fn it_works() {
//     //     let result = add(2, 2);
//     //     assert_eq!(result, 4);
//     // }
// }
