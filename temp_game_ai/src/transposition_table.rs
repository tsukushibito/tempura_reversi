use crate::{hasher::Fnv1aHashMap, GameState};

/// TTEntry stores the search depth, evaluation value, and node type.
#[derive(Debug, Clone)]
struct TTEntry {
    depth: usize,
    value: i32,
    node_type: NodeType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NodeType {
    Exact,
    LowerBound, // Fail-high
    UpperBound, // Fail-low
}

pub enum LookupResult {
    Value(i32),
    AlphaBeta(i32, i32),
}

#[derive(Debug, Clone, Default)]
pub struct TranspositionTable<S>
where
    S: GameState,
{
    table: Fnv1aHashMap<S, TTEntry>,
    pub hits: usize,
}

impl<S> TranspositionTable<S>
where
    S: GameState,
{
    pub fn lookup(&mut self, state: &S, alpha: i32, beta: i32, depth: usize) -> LookupResult {
        let mut alpha = alpha;
        let mut beta = beta;
        if let Some(entry) = self.table.get(state) {
            if entry.depth >= depth {
                self.hits += 1;
                match entry.node_type {
                    NodeType::Exact => return LookupResult::Value(entry.value),
                    NodeType::LowerBound => alpha = alpha.max(entry.value),
                    NodeType::UpperBound => beta = beta.min(entry.value),
                }

                if alpha >= beta {
                    return LookupResult::Value(entry.value);
                }
            }
        }
        return LookupResult::AlphaBeta(alpha, beta);
    }

    pub fn get_value(&self, state: &S) -> Option<i32> {
        self.table.get(state).map(|entry| entry.value)
    }

    pub fn store(&mut self, state: S, depth: usize, value: i32, alpha: i32, beta: i32) {
        let node_type = if value <= alpha {
            NodeType::UpperBound
        } else if value >= beta {
            NodeType::LowerBound
        } else {
            NodeType::Exact
        };
        self.table.insert(
            state,
            TTEntry {
                depth,
                value,
                node_type,
            },
        );
    }
}
