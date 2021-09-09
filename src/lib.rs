pub mod instant_runoff_voting;
pub mod schulze_method;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct Candidate {
    id: usize,
}

type Rank = i32;

// Multiple candidates with the same rank is allowed
// Should allow for Candidates with no rank (eg. give them the lowest rank to tie)
type Ballot = Vec<(Candidate, Rank)>;
type BallotSlice = [(Candidate, Rank)];

// Checks if all the Candidates on the ballot are unique.
fn unique_candidates(ballot: &BallotSlice) -> bool {
    let mut candates: Vec<usize> = ballot.iter().map(|(candidate, _)| candidate.id).collect();

    if candates.is_empty() {
        // empty ballot is valid
        return true;
    }

    // invalid if there are duplicates
    // sort the list, duplicates will be sequential
    candates.sort_unstable();
    for window in candates.windows(2) {
        if window[0] == window[1] {
            return false;
        }
    }

    true
}

fn highest_id(votes: &[Ballot]) -> usize {
    let mut num_candidates = 0;
    for ballot in votes {
        for (candidate, _) in ballot {
            num_candidates = std::cmp::max(num_candidates, candidate.id);
        }
    }

    num_candidates
}

/// Pairwise Preferences in an election. For some number of candidates, stores the preference of
/// one candidate to another.
///
/// This can be used for counting the number of ballots that
/// prefer candidate x to candidate y.
#[derive(Debug)]
struct PairwisePreferences {
    // count[x][y] is the number of voters who prefer candidate x to candidate y.
    counts: Vec<Vec<i32>>,
}

impl PairwisePreferences {
    fn new(num_candidates: usize) -> PairwisePreferences {
        PairwisePreferences {
            counts: vec![vec![0; num_candidates]; num_candidates],
        }
    }

    fn num_candidates(&self) -> usize {
        self.counts.len()
    }

    fn candidates(&self) -> impl Iterator<Item = Candidate> {
        (0..self.num_candidates()).map(|candidate_id| candidate_id.into())
    }

    fn from_ballots(ballots: &[Ballot]) -> Self {
        let highest_id = highest_id(ballots);
        let mut count = PairwisePreferences::new(highest_id + 1);

        for ballot in ballots {
            count.count_ballot(ballot);
        }

        count
    }

    // Adds a new ballot to the total count.
    fn count_ballot(&mut self, ballot: &BallotSlice) {
        for i in 0..ballot.len() {
            for j in (i + 1)..ballot.len() {
                let (candidate_a, rank_a) = ballot[i];
                let (candidate_b, rank_b) = ballot[j];

                use std::cmp::Ordering;
                match rank_a.cmp(&rank_b) {
                    // candidate_a is preferred to candidate_b
                    Ordering::Less => self.counts[candidate_a.id][candidate_b.id] += 1,
                    // candidate_b is preferred to candidate_a
                    Ordering::Greater => self.counts[candidate_b.id][candidate_a.id] += 1,
                    // otherwise rank_a == rank_b, do not change the count
                    Ordering::Equal => {}
                }
            }
        }
    }
}

impl From<usize> for Candidate {
    fn from(id: usize) -> Self {
        Candidate { id }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const ALICE: Candidate = Candidate { id: 0 };
    const BOB: Candidate = Candidate { id: 1 };
    const CHAD: Candidate = Candidate { id: 2 };

    #[test]
    fn ballot_count_forward() {
        let ballots = vec![vec![(ALICE, 1), (BOB, 2)]];
        let count = PairwisePreferences::from_ballots(&ballots);
        assert_eq!(count.counts, vec![vec![0, 1], vec![0, 0],]);
    }

    #[test]
    fn ballot_count_backward() {
        let ballots = vec![vec![(ALICE, 2), (BOB, 1)]];
        let count = PairwisePreferences::from_ballots(&ballots);
        assert_eq!(count.counts, vec![vec![0, 0], vec![1, 0],]);
    }

    #[test]
    fn ballot_count_equal() {
        let ballots = vec![vec![(ALICE, 1), (BOB, 1)]];
        let count = PairwisePreferences::from_ballots(&ballots);
        assert_eq!(count.counts, vec![vec![0, 0], vec![0, 0],]);
    }

    #[test]
    fn ballot_counts() {
        let ballots = vec![
            vec![(ALICE, 1), (BOB, 2), (CHAD, 3)],
            vec![(ALICE, 1), (BOB, 1), (CHAD, 3)],
            vec![(ALICE, 3), (BOB, 2), (CHAD, 1)],
        ];
        let count = PairwisePreferences::from_ballots(&ballots);
        assert_eq!(
            count.counts,
            vec![vec![0, 1, 2], vec![1, 0, 2], vec![1, 1, 0],]
        );
    }
}
