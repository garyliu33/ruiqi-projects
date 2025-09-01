use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::f64;
use std::cmp::Ordering;
use std::fmt::Display;

/// A node in the game tree.
/// `M` = Move type (generic, e.g., Card or (row,col))
#[derive(Clone, Debug)]
pub struct Node<M: Clone + PartialEq + Display> {
    pub move_made: Option<M>,                       // Move that led to this node
    pub parent: Option<Weak<RefCell<Node<M>>>>,     // Parent node
    pub children: Vec<Rc<RefCell<Node<M>>>>,        // Child nodes
    pub wins: f64,                                  // Wins from playerJustMoved's perspective
    pub visits: u32,                                // Number of times visited
    pub avails: u32,                                // Availability count
    pub player_just_moved: Option<usize>,           // Player who made this move
}

impl<M: Clone + PartialEq + Display> Node<M> {
    pub fn new(move_made: Option<M>, parent: Option<Weak<RefCell<Node<M>>>>, player: Option<usize>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            move_made,
            parent,
            children: vec![],
            wins: 0.0,
            visits: 0,
            avails: 1,
            player_just_moved: player,
        }))
    }

    /// Return moves that are legal but not yet tried
    pub fn get_untried_moves(&self, legal_moves: &[M]) -> Vec<M> {
        let tried_moves: Vec<M> = self.children.iter()
            .filter_map(|c| c.borrow().move_made.clone())
            .collect();

        legal_moves.iter()
            .filter(|m| !tried_moves.contains(m))
            .cloned()
            .collect()
    }

    /// UCB1 selection
    pub fn ucb_select_child(&mut self, legal_moves: &[M], exploration: f64) -> Rc<RefCell<Node<M>>> {
        let legal_children: Vec<Rc<RefCell<Node<M>>>> = self.children
            .iter()
            .filter(|c| {
                if let Some(m) = &c.borrow().move_made {
                    legal_moves.contains(m)
                } else {
                    false
                }
            })
            .cloned()
            .collect();

        // choose child with max UCB1
        let best = legal_children.iter()
            .max_by(|a, b| {
                let ca = a.borrow();
                let cb = b.borrow();

                let score_a = ca.wins / ca.visits as f64
                    + exploration * ((ca.avails as f64).ln() / ca.visits as f64).sqrt();
                let score_b = cb.wins / cb.visits as f64
                    + exploration * ((cb.avails as f64).ln() / cb.visits as f64).sqrt();

                score_a.partial_cmp(&score_b).unwrap_or(Ordering::Equal)
            })
            .unwrap()
            .clone();

        // update availability
        for c in &legal_children {
            c.borrow_mut().avails += 1;
        }

        best
    }

    /// Add a new child node
    pub fn add_child(parent: &Rc<RefCell<Node<M>>>, m: M, p: usize) -> Rc<RefCell<Node<M>>> {
        let child = Node::new(Some(m), Some(Rc::downgrade(parent)), Some(p));
        parent.borrow_mut().children.push(child.clone());
        child
    }

    /// Find or add child
    pub fn find_or_add_child(parent: &Rc<RefCell<Node<M>>>, m: M, p: usize) -> Rc<RefCell<Node<M>>> {
        if let Some(c) = parent.borrow().children.iter()
            .find(|c| c.borrow().move_made.as_ref() == Some(&m)) {
            return c.clone();
        }
        Node::add_child(parent, m, p)
    }

    /// Update node stats given a terminal state
    pub fn update(&mut self, result: f64) {
        self.visits += 1;
        if self.player_just_moved.is_some() {
            self.wins += result;
        }
    }

    /// String representation
    pub fn to_string(&self) -> String {
        format!(
            "[M:{} W/V/A:{}/{}/{}]",
            self.move_made.as_ref().map(|m| m.to_string()).unwrap_or("-".to_string()),
            self.wins, self.visits, self.avails
        )
    }

    pub fn tree_to_string(node: &Rc<RefCell<Node<M>>>, indent: usize) -> String {
        let mut s = Node::<M>::indent_string(indent) + &node.borrow().to_string();
        for c in &node.borrow().children {
            s.push_str(&Node::tree_to_string(c, indent + 1));
        }
        s
    }

    fn indent_string(indent: usize) -> String {
        let mut s = String::from("\n");
        for _ in 0..indent {
            s.push_str("| ");
        }
        s
    }

    pub fn children_to_string(&self) -> String {
        self.children.iter()
            .map(|c| c.borrow().to_string() + "\n")
            .collect()
    }
}
