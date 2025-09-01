use rand::rng;
use rand::seq::IndexedRandom;
use rayon::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::rc::Rc;

use crate::game_state::GameState;
use crate::node::Node;

/// Parallel Multiple Observer IS-MCTS
/// - `root_state`: initial game state
/// - `iterations`: number of simulations
/// - `num_threads`: number of threads
/// Returns the best move according to visit counts.
pub fn parallel_multi_observer_ismcts<M, S>(
    root_state: &S,
    iterations: usize,
    num_threads: usize,
) -> M
where
    M: Clone + PartialEq + Display + Send + Sync,
    S: GameState<M> + Send + Sync,
{
    let job_results: Vec<Vec<(M, u32, f64)>> = (0..num_threads)
        .into_par_iter()
        .map(|_| {
            let state_clone = root_state.clone_state();
            multi_observer_ismcts_job(state_clone, iterations)
        })
        .collect();

    // go through all thread's results and aggregate visits and wins for the same M.
    let mut results: Vec<(M, u32, f64)> = vec![];
    for r in job_results {
        for (m, visits, wins) in r {
            // Search for an existing entry with the same move
            let mut found = false;
            for (existing_m, existing_visits, existing_wins) in results.iter_mut() {
                if *existing_m == m {
                    *existing_visits += visits;
                    *existing_wins += wins;
                    found = true;
                    break;
                }
            }

            // If no matching move was found, add a new entry
            if !found {
                results.push((m, visits, wins));
            }
        }
    }

    // Choose most visited child
    let best_child = results.iter().max_by_key(|c| c.1).unwrap();

    best_child.0.clone()
}

/// Multi Observer IS-MCTS
/// - `root_state`: initial game state
/// - `iterations`: number of simulations
/// Returns the best move according to visit counts.
fn multi_observer_ismcts_job<M: Clone + PartialEq + Display, S: GameState<M>>(
    root_state: S,
    iterations: usize,
) -> Vec<(M, u32, f64)> {
    let mut root_trees = HashMap::<usize, Rc<RefCell<Node<M>>>>::new();
    for p in 0..root_state.number_of_players() {
        root_trees.insert(p, Node::new(None, None, None));
    }

    for _ in 0..iterations {
        let mut trees = HashMap::<usize, Rc<RefCell<Node<M>>>>::new();
        for p in 0..root_state.number_of_players() {
            trees.insert(p, root_trees.get(&p).unwrap().clone());
        }

        let mut current_player = root_state.player_to_move();
        // Determinize
        let mut state = root_state.clone_and_randomize(current_player);

        // Selection: descend tree while fully expanded & non-terminal
        while !state.get_moves().is_empty()
            && trees
                .get(&current_player)
                .unwrap()
                .borrow()
                .get_untried_moves(&state.get_moves())
                .is_empty()
        {
            let next = trees
                .get(&current_player)
                .unwrap()
                .borrow_mut()
                .ucb_select_child(&state.get_moves(), 0.7);
            let mv = next.borrow().move_made.clone().unwrap();
            for p in 0..state.number_of_players() {
                let n = trees.get(&p).unwrap();
                trees.insert(p, Node::find_or_add_child(n, mv.clone(), p));
            }
            state.do_move(&mv);
            current_player = state.player_to_move();
        }

        // Expansion: try an untried move
        let untried_moves = trees
            .get(&current_player)
            .unwrap()
            .borrow()
            .get_untried_moves(&state.get_moves());
        if !untried_moves.is_empty() {
            let m = untried_moves.choose(&mut rng()).unwrap().clone();
            state.do_move(&m);
            for p in 0..state.number_of_players() {
                trees.insert(p, Node::add_child(trees.get(&p).unwrap(), m.clone(), p));
            }
        }

        // Simulation: rollout until terminal
        while !state.get_moves().is_empty() {
            let m = state.get_moves().choose(&mut rng()).unwrap().clone();
            state.do_move(&m);
        }

        assert!(state.get_result(state.player_to_move()).is_some());

        // Backpropagation
        for p in 0..state.number_of_players() {
            let node = trees.get(&p).unwrap();
            let mut current: Option<Rc<RefCell<Node<M>>>> = Some(node.clone());
            while let Some(n) = current {
                let player = n.borrow().player_just_moved;
                if let Some(p) = player {
                    let result = state.get_result(p);
                    n.borrow_mut().update(result.unwrap());
                }
                current = n.borrow().parent.as_ref().and_then(|weak| weak.upgrade());
            }
        }
    }

    // root_node
    //     .borrow()
    //     .children
    //     .iter()
    //     .map(|n| {
    //         print!(
    //             "{} {}, ",
    //             n.borrow().move_made.as_ref().unwrap(),
    //             n.borrow().visits
    //         )
    //     })
    //     .count();
    // println!();

    let result: Vec<(M, u32, f64)> = root_trees
        .get(&root_state.player_to_move())
        .unwrap()
        .borrow()
        .children
        .iter()
        .map(|c| {
            (
                c.borrow().move_made.as_ref().unwrap().clone(),
                c.borrow().visits,
                c.borrow().wins,
            )
        })
        .collect();
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::games::game_state::GameState;
    use crate::games::test_knockout_whist_state;
    use crate::knockout_whist_state::{Card, KnockoutWhistState};
    use crate::node::Node;
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;

    fn setup_state_and_node() -> (KnockoutWhistState, Rc<RefCell<Node<Card>>>) {
        let state = KnockoutWhistState::new(2);
        let node = Node::new(None, None, None);
        (state, node)
    }

    #[test]
    fn test_ismcts_with_predefined_state() {
        let mut results = HashMap::<test_knockout_whist_state::Card, usize>::new();
        for _ in 0..5 {
            let mut state = test_knockout_whist_state::TestKnockoutWhistState::new(2);
            let deck = vec![
                // player 1
                test_knockout_whist_state::Card {
                    rank: 12,
                    suit: 'C',
                },
                test_knockout_whist_state::Card { rank: 4, suit: 'C' },
                test_knockout_whist_state::Card { rank: 3, suit: 'C' },
                test_knockout_whist_state::Card { rank: 5, suit: 'S' },
                test_knockout_whist_state::Card { rank: 6, suit: 'S' },
                test_knockout_whist_state::Card { rank: 7, suit: 'S' },
                test_knockout_whist_state::Card { rank: 8, suit: 'S' },
                // player 2
                test_knockout_whist_state::Card {
                    rank: 10,
                    suit: 'C',
                },
                test_knockout_whist_state::Card { rank: 5, suit: 'C' },
                test_knockout_whist_state::Card { rank: 2, suit: 'C' },
                test_knockout_whist_state::Card { rank: 8, suit: 'H' },
                test_knockout_whist_state::Card { rank: 7, suit: 'H' },
                test_knockout_whist_state::Card { rank: 6, suit: 'H' },
                test_knockout_whist_state::Card { rank: 5, suit: 'H' },
            ];
            state.deal(deck);
            state.do_move(&&test_knockout_whist_state::Card {
                rank: 12,
                suit: 'C',
            });
            let best_move = multi_observer_ismcts(&state, 10000);
            println!("best move: {}", best_move);
            *results.entry(best_move).or_insert(0) += 1;
        }
        println!("{:?}", results);
    }
}
