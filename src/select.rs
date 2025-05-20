use crate::mcts::Node;

pub enum Select {
    UCT(f32),
    ISUCT(f32),
    ProgressiveUCT(f32, f32),
    ProgressiveISUCT(f32, f32),
    PUCT(f32, f32),
    ISPUCT(f32, f32),
    UCTC(f32, f32),
    ISUCTC(f32, f32),
}

impl Select {
    pub fn get(&self, node: &Node) -> f32 {
        match self {
            Select::UCT(c) => uct(node, *c),
            Select::ISUCT(c) => isuct(node, *c),
            Select::ProgressiveUCT(c, d) => progressive_uct(node, *c, *d),
            Select::ProgressiveISUCT(c, d) => progressive_isuct(node, *c, *d),
            Select::PUCT(c_1, c_2) => puct(node, *c_1, *c_2),
            Select::ISPUCT(c_1, c_2) => ispuct(node, *c_1, *c_2),
            Select::UCTC(c_1, c_2) => uctc(node, *c_1, *c_2),
            Select::ISUCTC(c_1, c_2) => isuctc(node, *c_1, *c_2),
        }
    }
}

pub fn uct(node: &Node, c: f32) -> f32 {
    let stats = node.stats();

    let u = stats.reward / stats.visits as f32;
    let v = ((node.parent_visits() as f32).ln() / stats.visits as f32).sqrt();

    u + c * v
}

pub fn isuct(node: &Node, c: f32) -> f32 {
    let stats = node.stats();

    let u = stats.reward / stats.visits as f32;
    let v = ((stats.availability as f32).ln() / stats.visits as f32).sqrt();

    u + c * v
}

pub fn progressive_uct(node: &Node, c: f32, d: f32) -> f32 {
    let stats = node.stats();

    uct(node, c) + (-stats.value) / (stats.visits as f32 * d)
}

pub fn progressive_isuct(node: &Node, c: f32, d: f32) -> f32 {
    let stats = node.stats();

    isuct(node, c) + (-stats.value) / (stats.visits as f32 * d)
}

pub fn puct(node: &Node, c_1: f32, c_2: f32) -> f32 {
    let stats = node.stats();

    let n = node.parent_visits() as f32;
    let c = c_1 + ((n + c_2 + 1.0) / c_2).ln();
    let u = stats.reward / stats.visits as f32;
    let v = *node.policy() * n.sqrt() / (1.0 + stats.visits as f32);

    u + c * v
}

pub fn ispuct(node: &Node, c_1: f32, c_2: f32) -> f32 {
    let stats = node.stats();

    let n = stats.availability as f32;
    let c = c_1 + ((n + c_2 + 1.0) / c_2).ln();
    let u = stats.reward / stats.visits as f32;
    let v = *node.policy() * n.sqrt() / (1.0 + stats.visits as f32);

    u + c * v
}

pub fn uctc(node: &Node, c_1: f32, c_2: f32) -> f32 {
    let stats = node.stats();

    let n = node.parent_visits() as f32;
    let u = stats.reward / stats.visits as f32;
    let v = (n.ln() / stats.visits as f32).sqrt();

    u + c_1 + ((n + c_2 + 1.0) / c_2).ln() * v
}

pub fn isuctc(node: &Node, c_1: f32, c_2: f32) -> f32 {
    let stats = node.stats();

    let n = stats.availability as f32;
    let u = stats.reward / stats.visits as f32;
    let v = (n.ln() / stats.visits as f32).sqrt();

    u + c_1 + ((n + c_2 + 1.0) / c_2).ln() * v
}
