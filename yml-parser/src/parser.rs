
#[derive(Debug)]
struct Server {
  name: String,
  ip: String,
  port: u16 // for port 0-65,535
}
impl Server {
  pub fn new() -> Self {
    Self {
      name: String::new(),
      ip: String::new(),
      port: 0
    }
  }
}

#[derive(Debug)]
struct Listeners {
  protocol: String,
  port: u16 // for port 0-65,535
}
impl Listeners {
  pub fn new() -> Self{
    Self {
      protocol: String::new(),
      port: 0
    }
  }
}

#[derive(Debug)]
pub struct LoadBalancer {
  name: String,
  lb_type: String,
  listeners: Vec<Listeners>,
  backend_servers: Vec<Server>,
  algorithm: String
}
impl LoadBalancer {
  pub fn new() -> Self{
    Self {
      name: String::new(),
      lb_type: String::new(),
      backend_servers: Vec::new(),
      listeners: Vec::new(),
      algorithm: String::new()
    }
  }
}

#[derive(PartialEq, Debug)]
enum NestedParserKeywords {
  Name,
  Ip,
  Protocol,
  Port,
  None,
}

#[derive(PartialEq, Debug)]
enum ParserKeyWords {
  LoadBalancer,
  Type,
  Listeners,
  Name,
  BackendServers,
  Algorithm,
  None,
  KeyValue(NestedParserKeywords, String)
}

impl ParserKeyWords {
  fn from_str(s: &str) -> Option<ParserKeyWords> {

    match s {
      "algorithm" => Some(ParserKeyWords::Algorithm),
      "load_balancer" => Some(ParserKeyWords::LoadBalancer),
      "type" => Some(ParserKeyWords::Type),
      "backend_servers" => Some(ParserKeyWords::BackendServers),
      "listeners" => Some(ParserKeyWords::Listeners),
      _ => {
        let mut temp: &str = s;
        temp = temp.trim_start_matches("-").trim();

        let parser_keyword = match temp {
          "name" => NestedParserKeywords::Name,
          "protocol" => NestedParserKeywords::Protocol,
          "port" => NestedParserKeywords::Port,
          "ip" => NestedParserKeywords::Ip,
          _ => NestedParserKeywords::None,
        };

        Some(ParserKeyWords::KeyValue(parser_keyword,s.to_string()))
      }
    }
  }
}

fn count_indentation(lines:&Vec<String>) -> Vec<u8> {
  let mut count_list = vec![];

  for line in lines {
    let mut count = 0;
    for i in line.chars() {
      if i == ' ' {
        count = count + 1;
      } else {
        count_list.push(count);
        break;
      }
    }
  }
  count_list
}

#[derive(PartialEq, Debug)]
struct NestedObjectProperties {
  keyword: ParserKeyWords,
  ident: u8
}

impl NestedObjectProperties {
  pub fn new() -> Self {
    Self {
      keyword : ParserKeyWords::None,
      ident : 0
    }
  }
}

fn is_indented(str: &str) {

  true;
}

pub(crate) fn parse_to_object(processed_buffer: &Vec<String>) -> Result<LoadBalancer, ()> {

  let mut load_balancer: LoadBalancer = LoadBalancer::new();

  let key_indentation: Vec<u8> = count_indentation(&processed_buffer);
  let mut current_index = 0;
  let mut nested_object_properties:NestedObjectProperties = NestedObjectProperties { keyword :  ParserKeyWords::None, ident: 0};

  for line in processed_buffer {

    let parts: Vec<&str> = line.splitn(2, ':').collect();
    let key = parts[0].trim();
    match ParserKeyWords::from_str(key) {
      Some(parser_keyword) => {
        match parser_keyword {
          ParserKeyWords::LoadBalancer => {
            let is_valid = key_indentation.iter().all(|&value| value >= key_indentation[current_index]);
            if !is_valid {
              println!("Return Error INVALID KEYWORD;")
              // it should be less and equal to others, not more
            }
            nested_object_properties = NestedObjectProperties { keyword : ParserKeyWords::LoadBalancer, ident: key_indentation[current_index] };
          }
          ParserKeyWords::Listeners => {
            nested_object_properties = NestedObjectProperties { keyword : ParserKeyWords::Listeners, ident: key_indentation[current_index] };
          }
          ParserKeyWords::BackendServers => {
            nested_object_properties = NestedObjectProperties { keyword : ParserKeyWords::BackendServers, ident: key_indentation[current_index] };
          }
          ParserKeyWords::Algorithm => {
            if load_balancer.algorithm.is_empty() {
              load_balancer.algorithm = parts[1].trim().to_string();
            } else {
              println!("Map keys must be unique!!!")
              // return error
            }

            nested_object_properties = NestedObjectProperties { keyword : ParserKeyWords::Algorithm, ident: key_indentation[current_index] };
          }
          ParserKeyWords::Type => {
            if load_balancer.lb_type.is_empty() {
              load_balancer.lb_type = parts[1].trim().to_string();
            } else {
              println!("Map keys must be unique!!!")
              // return error
            }
            nested_object_properties = NestedObjectProperties { keyword : ParserKeyWords::Type, ident: key_indentation[current_index] };
          }
          ParserKeyWords::KeyValue(keyword,str) => {
            println!("{:?}", nested_object_properties);
                if nested_object_properties.keyword == ParserKeyWords::BackendServers && nested_object_properties.ident < key_indentation[current_index]  {

                  let mut backend_server = Server::new();

                  let is_create_new_object = str.trim().starts_with("-");

                  match keyword {
                    NestedParserKeywords::Name => {

                      if is_create_new_object {
                        backend_server.name = parts[1].trim().parse().unwrap();
                        load_balancer.backend_servers.push(backend_server);
                      } else {
                        let len = load_balancer.backend_servers.len();

                        if load_balancer.backend_servers[len-1].name.is_empty() {
                          load_balancer.backend_servers[len-1].name = parts[1].trim().parse().unwrap();
                        } else {
                          println!("Map keys must be unique!!!")
                    // return error
                        }
                      }

                    }
                    NestedParserKeywords::Port => {

                      if is_create_new_object {
                        backend_server.port = parts[1].trim().parse().unwrap();
                        load_balancer.backend_servers.push(backend_server);
                      } else {
                        let len = load_balancer.backend_servers.len();


                        if load_balancer.backend_servers[len-1].port == 0 {
                          load_balancer.backend_servers[len-1].port = parts[1].trim().parse().unwrap();
                        } else {
                          println!("Map keys must be unique!!!")
                    // return error
                        }

                      }

                    },
                    NestedParserKeywords::Ip => {

                      if is_create_new_object {
                        backend_server.ip = parts[1].trim().parse().unwrap();
                        load_balancer.backend_servers.push(backend_server);
                      } else {
                        let len = load_balancer.backend_servers.len();


                        if load_balancer.backend_servers[len-1].ip.is_empty() {
                          load_balancer.backend_servers[len-1].ip = parts[1].trim().parse().unwrap();
                        } else {
                          println!("Map keys must be unique!!! + backend server")
                    // return error
                        }
                      }

                    }
                    _ => {

                    }
                  }
                }

                 else if nested_object_properties.keyword == ParserKeyWords::Listeners && nested_object_properties.ident < key_indentation[current_index] {
                  let mut listener_object = Listeners::new();

                  let is_create_new_object = str.trim().starts_with("-");
                  let key = str.trim_start_matches('-').trim();

                  match key {
                    "protocol" => {

                      if is_create_new_object {
                        listener_object.protocol = parts[1].trim().parse().unwrap();
                        load_balancer.listeners.push(listener_object);
                      } else {
                        let len = load_balancer.listeners.len();

                        if load_balancer.listeners[len-1].protocol.is_empty() {
                          load_balancer.listeners[len-1].protocol = parts[1].trim().parse().unwrap();
                        }
                        else {
                          println!("Map keys must be unique!!!")
                          // return error
                        }
                      }

                    }
                    "port" => {

                      if is_create_new_object {
                        listener_object.port = parts[1].trim().parse().unwrap();
                        load_balancer.listeners.push(listener_object);
                      } else {
                        let len = load_balancer.listeners.len();

                        if load_balancer.listeners[len-1].port == 0 {
                          load_balancer.listeners[len-1].port = parts[1].trim().parse().unwrap();
                        }
                        else {
                          println!("Map keys must be unique!!!")
                          // return error
                        }
                      }

                    },
                    _ => {

                    }
                  }
                } else if keyword == NestedParserKeywords::Name {
                  if load_balancer.name.is_empty() {
                    load_balancer.name = parts[1].trim().to_string();
                  }  else {
                    println!("Map keys must be unique!!!")
                    // return error
                  }
                  nested_object_properties.keyword = ParserKeyWords::Name;
                  nested_object_properties.ident = key_indentation[current_index];
                }
          }
          _ => {
            println!("None of the keywords matched, Return Error: {:?}",parser_keyword);
          }
        }
      }
      None => {
        println!("Return Error: ");
      }
    }

    current_index = current_index + 1;
  }

  Ok(load_balancer)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_to_object() {

  }
}

// check with identation for - nested etc