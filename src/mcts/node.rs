use crate::stratego::{GameState, Move};
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard, Weak};

pub struct NodeStats {
    pub visits: usize,
    pub availability: usize,
    pub reward: f32,
    pub value: f32,
}

impl NodeStats {
    pub fn new(availability: usize, value: f32) -> Self {
        Self {
            visits: 0,
            availability,
            reward: 0.0,
            value,
        }
    }
}

pub struct Node {
    mov: Option<Move>,
    parent: Option<Weak<Node>>,
    state: RwLock<GameState>,
    policy: RwLock<f32>,
    children: RwLock<Vec<Arc<Node>>>,
    stats: RwLock<NodeStats>,
}

impl Node {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            mov: None,
            parent: None,
            state: RwLock::new(GameState::default()),
            policy: RwLock::new(0.0),
            children: Default::default(),
            stats: RwLock::new(NodeStats::new(0, 0.0)),
        })
    }

    pub fn add(self: Arc<Self>, mov: Move, state: GameState, value: f32) -> Arc<Node> {
        let mut children = self.children.write().unwrap();
        let parent = Arc::downgrade(&self);

        let child = Arc::new(Node {
            mov: Some(mov),
            parent: Some(parent),
            state: RwLock::new(state),
            policy: RwLock::new(0.0),
            children: Default::default(),
            stats: RwLock::new(NodeStats::new(1, value)),
        });

        children.push(Arc::clone(&child));
        child
    }

    pub fn untried(&self, moves: &[Move]) -> Vec<Move> {
        moves
            .iter()
            .filter(|mov| !self.children().any(|c| c.mov.as_ref().unwrap() == *mov))
            .cloned()
            .collect()
    }

    pub fn update(&self, reward: f32) {
        let mut stats = self.stats.write().unwrap();

        stats.visits += 1;
        stats.reward += reward;
    }

    pub fn mov(&self) -> Option<Move> {
        self.mov
    }

    pub fn parent(&self) -> Option<Arc<Node>> {
        self.parent.as_ref().and_then(Weak::upgrade)
    }

    pub fn children(&self) -> impl Iterator<Item = Arc<Node>> {
        self.children.read().unwrap().clone().into_iter()
    }

    pub fn is_empty(&self) -> bool {
        self.children.read().unwrap().is_empty()
    }

    pub fn game_state(&self) -> GameState {
        *self.state.read().unwrap()
    }

    pub fn policy(&self) -> RwLockReadGuard<'_, f32> {
        self.policy.read().unwrap()
    }

    pub fn policy_mut(&self) -> RwLockWriteGuard<'_, f32> {
        self.policy.write().unwrap()
    }

    pub fn stats(&self) -> RwLockReadGuard<'_, NodeStats> {
        self.stats.read().unwrap()
    }

    pub fn stats_mut(&self) -> RwLockWriteGuard<'_, NodeStats> {
        self.stats.write().unwrap()
    }

    pub fn parent_visits(&self) -> usize {
        self.parent().unwrap().stats.read().unwrap().visits
    }

    pub fn max_visits(&self) -> Option<Arc<Node>> {
        self.children().max_by_key(|c| c.stats().visits)
    }
}
