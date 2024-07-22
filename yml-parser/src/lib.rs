use std::{fs::File, io::{self, BufRead}};

use parser::LoadBalancer;
mod preprocess;
mod parser;


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


  let reader = io::BufReader::new(file);
  let mut processed_buffer : Vec<String>= Vec::new();

  for line in reader.lines() {

    match line {
      Ok(mut str) => {
        let initial_check = str.trim().to_string();

        if initial_check.is_empty() || initial_check.starts_with("#") {
          continue;
        }

        str = str.trim_end().to_string();
        let mut processed_line = preprocess::preprocess_by_line(&str)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        // to remove the end of the line space
        processed_line = processed_line.trim_end().to_string();
        if processed_line.is_empty() {
          continue;
        }

        if processed_line.starts_with(":") {
          io::Error::new(io::ErrorKind::InvalidData, "Invalid Syntax: Unexpected : ");
        }

        processed_buffer.push(processed_line);
      }
      Err(e) => {
        eprintln!("Error Reading Line: {}",e);
        io::Error::new(io::ErrorKind::InvalidData, e);
      }
    }
  }

  if !processed_buffer.is_empty() {
    match parser::parse_to_object(&processed_buffer) {
      Ok(res) => Ok(res),
      Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, "No YML Content to process")),
    }
  } else {
    Err(io::Error::new(io::ErrorKind::InvalidData, "No YML Content to process"))
  }

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
