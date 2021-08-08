use crate::{Candidate, Ballot, PairwisePreferences};

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
    fn wiki_schults_method() {
        let ballots = wiki_ballots();
        let winner = schulze_method(ballots);
        assert_eq!(winner, ELSA);
    }

    #[test]
    fn valid_ballot_test() {
        // TODO test with the election method
        let ballot = vec![(ALICE, 1), (BOB, 2), (CHAD, 3)];
        assert!(valid_ballot(&ballot));
    }

    #[test]
    fn invalid_ballot() {
        // TODO change to check if the election fails
        let ballot = vec![(ALICE, 1), (ALICE, 2)];
        assert!(!valid_ballot(&ballot));
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
