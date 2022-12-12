use petgraph::Graph;

struct NodeGraph {
  graph: Graph<PositionNode, ()>,
  max_distance: f64
}

impl NodeGraph{
  pub fn new(max_distance: f64) -> Self {
    Self { graph: Graph::<PositionNode, ()>::new(), max_distance }
  }

  pub fn add_node(&mut self, position_node: PositionNode) {
    self.graph.add_node(position_node);
  }

  pub fn reload_connections(& mut self) {
    self.graph.raw_nodes().iter().enumerate().for_each(|(item, iter)| {
      println!("{:?}", iter.weight.position());
    });
  }
}


#[cfg(test)]
mod node_graph_tests {
    use super::*;

    mod add_node {
      use super::*;
      #[test]
      fn add_a_node_to_the_graph() {
        let mut node_graph = NodeGraph::new(1.0);
        node_graph.add_node(PositionNode::new(0.0, 0.0));
        assert_eq!(node_graph.graph.node_count(), 1);
      }
    }

    mod reload_connections {
      use super::*;
      #[test]
      fn reloads_all_connections_on_the_node_graph() {
        let mut node_graph = NodeGraph::new(1.0);
        node_graph.add_node(PositionNode::new(0.0, 0.0));
        node_graph.reload_connections();
        assert_eq!(node_graph.graph.node_count(), 1);
      }
    }
}
