use crate::stratego::{GameState, Move};
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub struct Node {
    pub mov: Option<Move>,
    pub parent: Option<Weak<Node>>,
    state: RefCell<GameState>,
    children: RefCell<Vec<Rc<Node>>>,
    visits: RefCell<usize>,
    pub availability: RefCell<usize>,
    reward: RefCell<f32>,
}

impl Node {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            mov: None,
            parent: None,
            state: RefCell::new(GameState::default()),
            children: Default::default(),
            visits: RefCell::new(0),
            availability: RefCell::new(0),
            reward: RefCell::new(0.0),
        })
    }

    pub fn add(self: Rc<Self>, mov: Move, state: GameState) -> Rc<Node> {
        let mut children = self.children.borrow_mut();
        let parent = Rc::downgrade(&self);

        let child = Rc::new(Node {
            mov: Some(mov),
            parent: Some(parent.clone()),
            children: Default::default(),
            state: RefCell::new(state),
            visits: RefCell::new(0),
            availability: RefCell::new(1),
            reward: RefCell::new(0.0),
        });

        children.push(Rc::clone(&child));
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
        *self.visits.borrow_mut() += 1;
        *self.reward.borrow_mut() += reward;
    }

    pub fn children(&self) -> impl Iterator<Item = Rc<Node>> {
        self.children.borrow().clone().into_iter()
    }

    pub fn is_empty(&self) -> bool {
        self.children.borrow().is_empty()
    }

    pub fn game_state(&self) -> GameState {
        *self.state.borrow()
    }

    pub fn visits(&self) -> usize {
        *self.visits.borrow()
    }

    pub fn max_visits(&self) -> Option<Rc<Node>> {
        self.children().max_by_key(|c| c.visits())
    }

    pub fn availability(&self) -> usize {
        *self.availability.borrow()
    }

    pub fn reward(&self) -> f32 {
        *self.reward.borrow()
    }
}
