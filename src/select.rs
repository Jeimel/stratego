use crate::mcts::Node;

pub enum Select {
    UCT(f32),
    ISUCT(f32),
    ProgressiveUCT(f32, f32),
    ProgressiveISUCT(f32, f32),
}

impl Select {
    pub fn get(&self, node: &Node) -> f32 {
        match self {
            Select::UCT(c) => uct(node, *c),
            Select::ISUCT(c) => isuct(node, *c),
            Select::ProgressiveUCT(c, d) => progressive_uct(node, *c, *d),
            Select::ProgressiveISUCT(c, d) => progressive_isuct(node, *c, *d),
        }
    }
}

pub fn uct(node: &Node, c: f32) -> f32 {
    let stats = node.stats();

    let u = stats.reward / stats.visits as f32;
    let v = c * ((node.parent_visits() as f32).ln() / stats.visits as f32).sqrt();

    u + v
}

pub fn isuct(node: &Node, c: f32) -> f32 {
    let stats = node.stats();

    let u = stats.reward / stats.visits as f32;
    let v = c * ((stats.availability as f32).ln() / stats.visits as f32).sqrt();

    u + v
}

pub fn progressive_uct(node: &Node, c: f32, d: f32) -> f32 {
    let stats = node.stats();

    uct(node, c) + (-stats.value * d) / stats.visits as f32
}

pub fn progressive_isuct(node: &Node, c: f32, d: f32) -> f32 {
    let stats = node.stats();

    isuct(node, c) + (-stats.value * d) / stats.visits as f32
}
