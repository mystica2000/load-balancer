pub struct RoundRobin<T> {
  nodes: Vec<T>,
  current_index: usize,
}

impl <T: Clone> RoundRobin<T> {
  pub fn new(nodes: Vec<T>) -> Self {
    RoundRobin {
      nodes,
      current_index: 0,
    }
  }

  pub fn get_next_backend(&mut self) -> Option<T> {
    if self.nodes.is_empty() {
      return None;
    }

    let current_node = self.nodes.remove(0);
    self.nodes.push(current_node.clone());
    Some(current_node)

      // get peek of the vector
      // add it as last
  }
}