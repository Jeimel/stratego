use crate::mcts::Node;

pub type Select = fn(&Node) -> f32;

pub fn uct(node: &Node) -> f32 {
    const C: f32 = 0.7;

    let stats = node.stats();

    let u = stats.reward / stats.visits as f32;
    let v = C * ((node.parent_visits() as f32).ln() / stats.visits as f32).sqrt();

    u + v
}

pub fn isuct(node: &Node) -> f32 {
    const C: f32 = 0.7;

    let stats = node.stats();

    let u = stats.reward / stats.visits as f32;
    let v = C * ((stats.availability as f32).ln() / stats.visits as f32).sqrt();

    u + v
}
