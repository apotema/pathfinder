pub struct PositionNode {
  position: Vector2<f64>,
  connections: Vec<Rc<PositionNode>>
}

impl PositionNode {
  pub fn new(x: f64, y: f64) -> Self {
    Self { position: Vector2::new(x, y), connections: Vec::new() }
  }

  pub fn position(&self) -> Vector2<f64> {
    return self.position;
  }

  pub fn connect(&mut self, position_node: Rc<PositionNode>) {
    self.connections.push(position_node);
  }

  pub fn is_connected_to(&self, position_node: Rc<PositionNode>) -> bool {
    return self.connections.iter().any(|v| Rc::ptr_eq(&v, &position_node));
  }
}


#[cfg(test)]
mod tests {
    use super::*;

    mod connect {
      use super::*;

      #[test]
      fn connects_to_another_node() {
        let mut first_node: PositionNode = PositionNode::new(0.0, 0.0);
        let second_node: PositionNode = PositionNode::new(2.0, 2.0);
        first_node.connect(Rc::new(second_node));
        assert_eq!(first_node.connections[0].position.x, 2.0);
      }
    }

    mod is_connected_to {
      use super::*;

      #[test]
      fn returns_true_if_node_is_connected_to_another_node() {
        let mut first_node: PositionNode = PositionNode::new(0.0, 0.0);
        let second_node = Rc::new(PositionNode::new(2.0, 2.0));
        let second_node_clone = Rc::clone(&second_node);
        first_node.connect(second_node);
        assert_eq!(first_node.is_connected_to(second_node_clone), true);
      }

      #[test]
      fn returns_false_if_node_is_connected_to_another_node() {
        let first_node: PositionNode = PositionNode::new(0.0, 0.0);
        let second_node = Rc::new(PositionNode::new(2.0, 2.0));
        assert_eq!(first_node.is_connected_to(second_node), false);
      }
    }
}
