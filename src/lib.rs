mod models;
use models::PositionNode;

pub fn add(left: usize, right: usize) -> usize {
    // let x = PositionNode{ x: 1.0, y: 2.0 };
    let position_node: PositionNode = PositionNode::new(0.0, 0.0);

    left + right
}
