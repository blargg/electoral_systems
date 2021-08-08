#[derive(Copy,Clone,PartialEq,Eq,Debug)]
pub struct Candidate {
    id: usize,
}

type Rank = i32;

// Multiple candidates with the same rank is allowed
// Should allow for Candidates with no rank (eg. give them the lowest rank to tie)
type Ballot = Vec<(Candidate, Rank)>;

/// Schulze method election. Ballots give a list of candidates and a number ranking.
/// Lower numbers are more preferred candidates. Each candidate can only be listed
/// once on a ballot. Multiple candidates may have the same ranking.
///
/// See [reference](https://en.wikipedia.org/wiki/Schulze_method) for more information.
pub fn schulze_method(votes: Vec<Ballot>) -> Candidate {
    // Check that the ballots are valid.
    for ballot in votes.iter() {
        assert!(valid_ballot(ballot));
    }

    let count = PairwisePreferences::from_ballots(&votes);
    let widest_paths = floyd_warshall_widest_paths(&count.count);

    // TODO, there can be more than one candidate that satisfies this. Should return them all
    for candidate in count.candidates() {
        if preferred_to_all_others(&widest_paths, candidate) {
            return candidate;
        }
    }
    unreachable!("At least one candidate must be preferred to all others using widest paths as the preference.")
}

// TODO make or expand a struct for pairwise preferences
fn preferred_to_all_others(preferences: &Vec<Vec<i32>>, candidate: Candidate) -> bool {
    for other in 0..preferences.len() {
        if candidate.id == other { continue; }
        // This candidate is strictly less preferred that some other candidate, return false.
        if preferences[candidate.id][other] < preferences[other][candidate.id] {
            return false;
        }
    }

    // At this point, candidate must be equal to or greater than all other direct comparisons.
    return true;
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

/// Pairwise Preferences in an election. For some number of candidates, stores the preference of
/// one candidate to another.
///
/// This can be used for counting the number of ballots that
/// prefer candidate x to candidate y.
#[derive(Debug)]
struct PairwisePreferences {
    // count[x][y] is the number of voters who prefer candidate x to candidate y.
    count: Vec<Vec<i32>>,
}

impl PairwisePreferences {
    fn new(num_candidates: usize) -> PairwisePreferences {
        PairwisePreferences {
            count: vec![vec![0; num_candidates]; num_candidates],
        }
    }

    fn num_candidates(&self) -> usize {
        self.count.len()
    }

    fn candidates(&self) -> impl Iterator<Item=Candidate> {
        (0..self.num_candidates()).map(|candidate_id| candidate_id.into())
    }

    fn from_ballots(ballots: &Vec<Ballot>) -> Self {
        let highest_id = highest_id(ballots);
        let mut count = PairwisePreferences::new(highest_id + 1);

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
    const DAVE: Candidate = Candidate { id: 3 };
    const ELSA: Candidate = Candidate { id: 4 };

    /// Example ballots from https://en.wikipedia.org/wiki/Schulze_method
    fn wiki_ballots() -> Vec<Ballot> {
        let ballots = vec![
            vec![ vec![(ALICE, 1), (CHAD, 2), (BOB, 3), (ELSA, 4), (DAVE, 5)]; 5],
            vec![ vec![(ALICE, 1), (DAVE, 2), (ELSA, 3), (CHAD, 4), (BOB, 5)]; 5],
            vec![ vec![(BOB, 1), (ELSA, 2), (DAVE, 3), (ALICE, 4), (CHAD, 5)]; 8],
            vec![ vec![(CHAD, 1), (ALICE, 2), (BOB, 3), (ELSA, 4), (DAVE, 5)]; 3],
            vec![ vec![(CHAD, 1), (ALICE, 2), (ELSA, 3), (BOB, 4), (DAVE, 5)]; 7],
            vec![ vec![(CHAD, 1), (BOB, 2), (ALICE, 3), (DAVE, 4), (ELSA, 5)]; 2],
            vec![ vec![(DAVE, 1), (CHAD, 2), (ELSA, 3), (BOB, 4), (ALICE, 5)]; 7],
            vec![ vec![(ELSA, 1), (BOB, 2), (ALICE, 3), (DAVE, 4), (CHAD, 5)]; 8],
        ];

        return ballots.concat();
    }

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
        let count = PairwisePreferences::from_ballots(&ballots);
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
        let count = PairwisePreferences::from_ballots(&ballots);
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
        let count = PairwisePreferences::from_ballots(&ballots);
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
        let count = PairwisePreferences::from_ballots(&ballots);
        assert_eq!(count.count, vec![
            vec![0, 1, 2],
            vec![1, 0, 2],
            vec![1, 1, 0],
        ]);
    }

    #[test]
    fn wiki_ballot_count() {
        let ballots = wiki_ballots();
        let count = PairwisePreferences::from_ballots(&ballots);
        assert_eq!(count.count, vec![
            vec![0, 20, 26, 30, 22],
            vec![25, 0, 16, 33, 18],
            vec![19, 29, 0, 17, 24],
            vec![15, 12, 28, 0, 14],
            vec![23, 27, 21, 31, 0],
        ]);
    }

    #[test]
    fn wiki_floyd_warshall() {
        let ballots = wiki_ballots();
        let count = PairwisePreferences::from_ballots(&ballots);
        let widest_paths = floyd_warshall_widest_paths(&count.count);
        assert_eq!(widest_paths, vec![
            vec![i32::MAX, 28, 28, 30, 24],
            vec![25, i32::MAX, 28, 33, 24],
            vec![25, 29, i32::MAX, 29, 24],
            vec![25, 28, 28, i32::MAX, 24],
            vec![25, 28, 28, 31, i32::MAX],
        ]);
    }

    #[test]
    fn wiki_schults_method() {
        let ballots = wiki_ballots();
        let winner = schulze_method(ballots);
        assert_eq!(winner, ELSA);
    }

    #[test]
    fn simple_floyd_warshall() {
        let edge_weights = vec![
            vec![0, 5, 0],
            vec![0, 0, 5],
            vec![0, 0, 0],
        ];
        let widest_paths = floyd_warshall_widest_paths(&edge_weights);
        assert_eq!(widest_paths, vec![
            vec![i32::MAX, 5, 5],
            vec![0, i32::MAX, 5],
            vec![0, 0, i32::MAX],
        ]);
    }

    #[test]
    fn simple_floyd_warshall_reversed() {
        let edge_weights = vec![
            vec![0, 0, 0],
            vec![5, 0, 0],
            vec![1, 5, 0],
        ];
        let widest_paths = floyd_warshall_widest_paths(&edge_weights);
        assert_eq!(widest_paths, vec![
            vec![i32::MAX, 0, 0],
            vec![5, i32::MAX, 0],
            vec![5, 5, i32::MAX],
        ]);
    }

    #[test]
    fn floyd_warshall_chain() {
        let edge_weights = vec![
            vec![0, 5, 1],
            vec![1, 0, 5],
            vec![3, 1, 0],
        ];
        let widest_paths = floyd_warshall_widest_paths(&edge_weights);
        assert_eq!(widest_paths, vec![
            vec![i32::MAX, 5, 5],
            vec![3, i32::MAX, 5],
            vec![3, 3, i32::MAX],
        ]);
    }

    #[test]
    fn complex_floyd_warshall() {
        let edge_weights = vec![
            vec![0, 5, 1, 0],
            vec![1, 0, 5, 0],
            vec![0, 1, 0, 5],
            vec![3, 0, 1, 0],
        ];
        let widest_paths = floyd_warshall_widest_paths(&edge_weights);
        assert_eq!(widest_paths, vec![
            vec![i32::MAX, 5, 5, 5],
            vec![3, i32::MAX, 5, 5],
            vec![3, 3, i32::MAX, 5],
            vec![3, 3, 3, i32::MAX],
        ]);
    }
}
