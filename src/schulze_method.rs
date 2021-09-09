use crate::{Candidate, Ballot, BallotSlice, unique_candidates, PairwisePreferences};

/// Schulze method election. Ballots give a list of candidates and a number ranking.
/// Lower numbers are more preferred candidates. Each candidate can only be listed
/// once on a ballot. Multiple candidates may have the same ranking.
///
/// The candidates are returned in order of preference, with the first element being the most
/// preferred candidate, and the last being the least preferred.
///
/// Ties are broken arbitrarily.
///
/// See [reference](https://en.wikipedia.org/wiki/Schulze_method) for more information.
pub fn schulze_method(votes: Vec<Ballot>) -> Vec<Candidate> {
    // Check that the ballots are valid.
    for ballot in votes.iter() {
        assert!(valid_ballot(ballot));
    }

    let count = PairwisePreferences::from_ballots(&votes);
    let widest_paths = floyd_warshall_widest_paths(&count.counts);

    let mut candidates_to_sort = count
        .candidates()
        .collect::<Vec<_>>();
    candidates_to_sort.sort_by_key(|candidate| preferred_above_count(&widest_paths, *candidate));
    candidates_to_sort.reverse();
    candidates_to_sort
}

/// schulze_method, but only returns the first candidate.
pub fn schulze_method_single(votes: Vec<Ballot>) -> Candidate {
    *schulze_method(votes).get(0).expect("Expecting there to be at least one candidate.")
}

// Checks if the ballot is valid.
fn valid_ballot(ballot: &BallotSlice) -> bool {
    unique_candidates(ballot)
}

/// Returns widest_path[x][y] which is the capacity of the widest path from x to y.
fn floyd_warshall_widest_paths(weights: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let dim = weights.len();
    if dim == 0 {
        return vec![];
    }
    assert!(dim == weights[0].len(), "only valid for square matrices");
    // Initialize widest path to all 0 and 1 step widest paths.
    let mut current_widest = weights.to_owned();
    #[allow(clippy::needless_range_loop)]
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

    current_widest
}

/// For the given candidate, count the number of challengers that the candidate beats.
fn preferred_above_count(preferences: &[Vec<i32>], candidate: Candidate) -> usize {
    let mut count = 0;
    for other in 0..preferences.len() {
        if candidate.id == other { continue; }
        if preferences[candidate.id][other] >= preferences[other][candidate.id] {
            count += 1;
        }
    }

    // At this point, candidate must be equal to or greater than all other direct comparisons.
    count
}

#[cfg(test)]
mod test {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        /// Forall edge_weights (square matrix of edge capacities), and
        /// Forall path that starts at x and ends at y (a path through some subset of nodes in that graph)
        /// The capacity of that path must be less than or equal to the widest path that
        /// floyd_warshall_widest_paths finds from x to y.
        fn prop_floyd_warshall_widest_path_finds_widest((edge_weights, path) in weights_and_path()) {
            let widest = floyd_warshall_widest_paths(&edge_weights);

            let mut path_width = i32::MAX;
            for window in path.windows(2) {
                let current_width = edge_weights[window[0]][window[1]];
                path_width = std::cmp::min(path_width, current_width);
            }

            let first = path[0];
            let last = path[path.len() - 1];
            // TODO can probably just check all the paths at once
            assert!(path_width <= widest[first][last]);
        }
    }

    fn square_vec(length: impl Strategy<Value = usize>) -> impl Strategy<Value = Vec<Vec<i32>>> {
        use proptest::collection::vec;
        length.prop_flat_map(|length| vec(vec(0..100, length), length))
    }

    fn shuffled_subsequence(values: std::ops::Range<usize>, size: usize) -> impl Strategy<Value = Vec<usize>> {
        let values = values.collect::<Vec<_>>();
        use proptest::sample::subsequence;
        subsequence(values, size).prop_shuffle()
    }

    fn weights_and_path() -> impl Strategy<Value=(Vec<Vec<i32>>, Vec<usize>)>{
        // So far, this is only used in props that make sense with 2 or more length
        let length = 2..10usize;
        length.prop_flat_map(|num_vertecies| {
            (1..num_vertecies).prop_flat_map(move |path_length| {
                square_vec(Just(num_vertecies)).prop_flat_map(move |weights| {
                    shuffled_subsequence(0..num_vertecies, path_length).prop_map(move |path| ((weights.clone(), path)))
                })
            })
        })
    }

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

        ballots.concat()
    }

    #[test]
    fn wiki_schults_method() {
        let ballots = wiki_ballots();
        let winner = schulze_method_single(ballots);
        assert_eq!(winner, ELSA);
    }

    #[test]
    #[should_panic]
    fn invalid_ballot() {
        let ballot = vec![(ALICE, 1), (ALICE, 2)];
        schulze_method(vec![ballot]);
    }

    #[test]
    fn wiki_floyd_warshall() {
        let ballots = wiki_ballots();
        let count = PairwisePreferences::from_ballots(&ballots);
        let widest_paths = floyd_warshall_widest_paths(&count.counts);
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
