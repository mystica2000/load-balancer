

pub struct RoundRobin<T> {
  nodes: Vec<T>
}

impl<T: Clone> RoundRobin<T> {
  pub fn new(nodes: Vec<T>) -> Self {
    RoundRobin { nodes }
}

  fn get_next_backend(mut self) -> T {
    let current_node = self.nodes.remove(0);
    self.nodes.push(current_node.clone());
    current_node

    // get peek of the vector
    // add it as last
  }
}