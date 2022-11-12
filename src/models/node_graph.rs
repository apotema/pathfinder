
struct NodeGraph<'a> {
  nodes: Vec<& 'a PositionNode>,
  max_distance: f64
}

impl<'a> NodeGraph<'a> {
  pub fn new(max_distance: f64) -> Self {
   return Self { nodes: Vec::new(), max_distance };
  }

  pub fn add_node(&mut self, node: &PositionNode) {
    self.nodes.push(node)
  }

  pub fn reload_connections(& mut self) {
    for (index, node) in self.nodes.iter_mut().enumerate() {
      for (connection_index, connection_node) in self.nodes.iter().enumerate() {
        if index == connection_index {
          continue;
        }
        if Vector2::distance(node.position, connection_node.position) < self.max_distance {
          node.connect(connection_node);
        }
      }
    }
  }
}


#[cfg(test)]
mod node_graph_tests {
    use super::*;

    mod add_node {
      use super::*;
      #[test]
      fn add_a_node_to_the_list() {
        let mut node_graph = NodeGraph::new(1.0);
        node_graph.add_node(PositionNode::new(0.0,0.0));
        assert_eq!(node_graph.nodes.len(), 1);
      }
    }

    mod reload_connections {
      use super::*;
      #[test]
      fn reloads_all_connections_on_the_node_graph() {
        let mut node_graph = NodeGraph::new(1.0);
        node_graph.add_node(PositionNode::new(0.0,0.0));
        node_graph.add_node(PositionNode::new(1.0,0.0));
        node_graph.reload_connections();
        assert_eq!(node_graph.nodes[0].connections[0].position.x, 1.0);
      }
    }
}
