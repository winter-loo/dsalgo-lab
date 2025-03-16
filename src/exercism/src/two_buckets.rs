/// source: https://exercism.org/tracks/rust/exercises/two-bucket
/// recursive implementation, diagram:
///     https://excalidraw.com/#json=izMJYe0PF44XEYsFrAcqq,ww4JBcul0hyLDaw1suTvqQ
#[test]
fn test_solve() {
    let output = solve(3, 5, 1, &Bucket::One);
    println!("{output:?}");
    let output = solve(3, 5, 1, &Bucket::Two);
    println!("{output:?}");
    let output = solve(7, 11, 2, &Bucket::One);
    println!("{output:?}");
    let output = solve(7, 11, 2, &Bucket::Two);
    println!("{output:?}");
}

#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

#[derive(Debug, Clone, Copy)]
enum Action {
    FillBucketOne(u8),
    FillBucketTwo(u8),
    EmptyBucketOne,
    EmptyBucketTwo,
    PourFromBucketOne(u8),
    PourToBucketOne(u8),
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Default, Hash)]
struct State {
    one: u8,
    two: u8,
}

impl State {
    fn new() -> Self {
        State { one: 0, two: 0 }
    }

    // state machine transition
    fn transition(&self, action: Action) -> Option<State> {
        match action {
            Action::FillBucketOne(one_cap) => {
                if self.one != 0 {
                    None
                } else {
                    Some(State {
                        one: one_cap,
                        two: self.two,
                    })
                }
            }
            Action::FillBucketTwo(two_cap) => {
                if self.two != 0 {
                    None
                } else {
                    Some(State {
                        one: self.one,
                        two: two_cap,
                    })
                }
            }
            Action::EmptyBucketOne => {
                if self.one == 0 {
                    None
                } else {
                    Some(State {
                        one: 0,
                        two: self.two,
                    })
                }
            }
            Action::EmptyBucketTwo => {
                if self.two == 0 {
                    None
                } else {
                    Some(State {
                        one: self.one,
                        two: 0,
                    })
                }
            }
            Action::PourFromBucketOne(two_cap) => {
                if self.one == 0 || self.two == two_cap {
                    None
                } else {
                    let delta = std::cmp::min(self.one, two_cap - self.two);
                    Some(State {
                        one: self.one - delta,
                        two: self.two + delta,
                    })
                }
            }
            Action::PourToBucketOne(one_cap) => {
                if self.one == one_cap || self.two == 0 {
                    None
                } else {
                    let delta = std::cmp::min(self.two, one_cap - self.one);
                    Some(State {
                        one: self.one + delta,
                        two: self.two - delta,
                    })
                }
            }
        }
    }

    fn goal_state_reached(&self, goal: u8) -> Option<Bucket> {
        if self.one == goal {
            Some(Bucket::One)
        } else if self.two == goal {
            Some(Bucket::Two)
        } else {
            None
        }
    }
}

use std::collections::HashSet;

struct PuzzleContext {
    goal: u8,
    capacity_1: u8,
    capacity_2: u8,
    invalid_state: State,
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    if goal > capacity_1.max(capacity_2) {
        return None;
    }
    let mut seen = HashSet::new();
    let (initial_action, invalid_state) = match start_bucket {
        Bucket::One => (
            Action::FillBucketOne(capacity_1),
            State {
                one: 0,
                two: capacity_2,
            },
        ),
        Bucket::Two => (
            Action::FillBucketTwo(capacity_2),
            State {
                two: 0,
                one: capacity_1,
            },
        ),
    };
    let initial_state = State::new();
    seen.insert(initial_state);

    let mut bs = BucketStats {
        moves: 0,
        goal_bucket: Bucket::One,
        other_bucket: 0,
    };
    let context = PuzzleContext {
        goal,
        capacity_1,
        capacity_2,
        invalid_state,
    };
    if solve_puzzle(&initial_state, initial_action, &context, &mut seen, &mut bs).is_some() {
        Some(bs)
    } else {
        None
    }
}

fn solve_puzzle(
    state: &State,
    action: Action,
    ctx: &PuzzleContext,
    seen: &mut HashSet<State>,
    bs: &mut BucketStats,
) -> Option<State> {
    let new_state = state.transition(action)?;

    if new_state == ctx.invalid_state || seen.contains(&new_state) {
        return None;
    }

    bs.moves += 1;
    if let Some(bucket) = new_state.goal_state_reached(ctx.goal) {
        bs.goal_bucket = bucket;
        bs.other_bucket = if matches!(bs.goal_bucket, Bucket::One) {
            new_state.two
        } else {
            new_state.one
        };
        return Some(new_state);
    }
    seen.insert(new_state);

    let actions = [
        Action::FillBucketOne(ctx.capacity_1),
        Action::FillBucketTwo(ctx.capacity_2),
        Action::EmptyBucketOne,
        Action::EmptyBucketTwo,
        Action::PourFromBucketOne(ctx.capacity_2),
        Action::PourToBucketOne(ctx.capacity_1),
    ];

    for act in actions {
        let ans = solve_puzzle(&new_state, act, ctx, seen, bs);
        if ans.is_some() {
            return ans;
        }
    }

    // backtracking
    bs.moves -= 1;
    None
}
