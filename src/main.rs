fn main() {
    println!("Hello, world!");
}

#[derive(Copy,Clone)]
pub struct Candidate {
    id: usize,
}

type Rank = i32;

// Multiple candidates with the same rank is allowed
// Should allow for Candidates with no rank (eg. give them the lowest rank to tie)
type Ballot = Vec<(Candidate, Rank)>;

pub fn schulze_election(votes: Vec<Ballot>) -> Candidate {
    // Check that the ballots are valid.
    for ballot in votes.iter() {
        assert!(valid_ballot(ballot));
    }

    let count = VoteCount::from_ballots(&votes);
    todo!()
}

fn highest_id(votes: &Vec<Ballot>) -> usize {
    let mut num_candidates = 0;
    for ballot in votes {
        for (candidate, _) in ballot {
            num_candidates = std::cmp::max(num_candidates, candidate.id);
        }
    }

    return num_candidates;
}

#[derive(Debug)]
struct VoteCount {
    // Pairwise preferences.
    // self.count[x][y] is the number of voters who prefer candidate x to candidate y.
    count: Vec<Vec<i32>>,
}

impl VoteCount {
    fn new(num_candidates: usize) -> VoteCount {
        VoteCount {
            count: vec![vec![0; num_candidates]; num_candidates],
        }
    }

    fn from_ballots(ballots: &Vec<Ballot>) -> Self {
        let highest_id = highest_id(ballots);
        let mut count = VoteCount::new(highest_id + 1);

        for ballot in ballots {
            count.count_ballot(ballot);
        }

        return count;
    }

    // Adds a new ballot to the total count.
    fn count_ballot(&mut self, ballot: &Ballot) {
        for i in 0..ballot.len() {
            for j in (i+1)..ballot.len() {
                let (candidate_a, rank_a) = ballot[i];
                let (candidate_b, rank_b) = ballot[j];

                if rank_a < rank_b {
                    // candidate_a is preferred to candidate_b
                    self.count[candidate_a.id][candidate_b.id] += 1;
                } else if rank_b < rank_a {
                    self.count[candidate_b.id][candidate_a.id] += 1;
                } // otherwise rank_a == rank_b, do not change the count
            }
        }
    }
}

// Checks if the ballot is valid.
fn valid_ballot(ballot: &Ballot) -> bool {
    let mut candates: Vec<usize> = ballot.iter().map(|(candidate, _)| candidate.id).collect();

    if candates.len() < 1 {
        // empty ballot is valid
        return true;
    }

    // invalid if there are duplicates
    // sort the list, duplicates will be sequential
    candates.sort();
    for window in candates.windows(2) {
        if window[0] == window[1] {
            return false;
        }
    }
    return true;
}

impl Into<Candidate> for usize {
    fn into(self) -> Candidate {
        Candidate { id: self }
    }
}

/// Returns widest_path[x][y] which is the capacity of the widest path from x to y.
fn floyd_warshall_widest_paths(weights: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let dim = weights.len();
    if dim == 0 {
        return vec![];
    }
    assert!(dim == weights[0].len(), "only valid for square matrices");
    // Initialize widest path to all 0 and 1 step widest paths.
    let mut current_widest = weights.clone();
    for i in 0..dim {
        // self loop assumed to have maximum width.
        current_widest[i][i] = i32::MAX;
    }

    // For each k, a new node to introduce into the possible paths, check if k can be used in a new
    // wider path between any two nodes.
    for k in 0..dim {
        for i in 0..dim {
            for j in 0..dim {
                let width_using_k = std::cmp::min(current_widest[i][k], current_widest[k][j]);
                if current_widest[i][j] < width_using_k {
                    current_widest[i][j] = width_using_k;
                }
            }
        }
    }

    return current_widest;
}

#[cfg(test)]
mod test {
    use super::*;

    const ALICE: Candidate = Candidate { id: 0 };
    const BOB: Candidate = Candidate { id: 1 };
    const CHAD: Candidate = Candidate { id: 2 };

    #[test]
    fn valid_ballot_test() {
        let ballot = vec![(ALICE, 1), (BOB, 2), (CHAD, 3)];
        assert!(valid_ballot(&ballot));
    }

    #[test]
    fn invalid_ballot() {
        let ballot = vec![(ALICE, 1), (ALICE, 2)];
        assert!(!valid_ballot(&ballot));
    }

    #[test]
    fn ballot_count_forward() {
        let ballots = vec![
            vec![(ALICE, 1), (BOB, 2)],
        ];
        let count = VoteCount::from_ballots(&ballots);
        assert_eq!(count.count, vec![
            vec![0, 1],
            vec![0, 0],
        ]);
    }

    #[test]
    fn ballot_count_backward() {
        let ballots = vec![
            vec![(ALICE, 2), (BOB, 1)],
        ];
        let count = VoteCount::from_ballots(&ballots);
        assert_eq!(count.count, vec![
            vec![0, 0],
            vec![1, 0],
        ]);
    }

    #[test]
    fn ballot_count_equal() {
        let ballots = vec![
            vec![(ALICE, 1), (BOB, 1)],
        ];
        let count = VoteCount::from_ballots(&ballots);
        assert_eq!(count.count, vec![
            vec![0, 0],
            vec![0, 0],
        ]);
    }

    #[test]
    fn ballot_counts() {
        let ballots = vec![
            vec![(ALICE, 1), (BOB, 2), (CHAD, 3)],
            vec![(ALICE, 1), (BOB, 1), (CHAD, 3)],
            vec![(ALICE, 3), (BOB, 2), (CHAD, 1)],
        ];
        let count = VoteCount::from_ballots(&ballots);
        assert_eq!(count.count, vec![
            vec![0, 1, 2],
            vec![1, 0, 2],
            vec![1, 1, 0],
        ]);
    }

    #[test]
    fn simple_floyd_warshal() {
        let edge_weights = vec![
            vec![0, 5, 1],
            vec![1, 0, 5],
            vec![0, 0, 0],
        ];
        let widest_paths = floyd_warshall_widest_paths(&edge_weights);
        assert_eq!(widest_paths, vec![
            vec![i32::MAX, 5, 5],
            vec![1, i32::MAX, 5],
            vec![0, 0, i32::MAX],
        ]);
    }
}
