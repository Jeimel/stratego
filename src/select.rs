use crate::mcts::Node;

pub enum Select {
    UCT,
    ISUCT,
    ProgressiveUCT,
    ProgressiveISUCT,
}

impl Select {
    pub fn get(&self, node: &Node) -> f32 {
        match self {
            Select::UCT => uct(node),
            Select::ISUCT => isuct(node),
            Select::ProgressiveUCT => progressive_uct(node),
            Select::ProgressiveISUCT => progressive_isuct(node),
        }
    }
}

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

pub fn progressive_uct(node: &Node) -> f32 {
    let stats = node.stats();

    uct(node) + (-stats.value * 10.0) / stats.visits as f32
}

pub fn progressive_isuct(node: &Node) -> f32 {
    let stats = node.stats();

    isuct(node) + (-stats.value * 10.0) / stats.visits as f32
}
