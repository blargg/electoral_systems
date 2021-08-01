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
                }
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
    fn ballot_count() {
        let ballots = vec![
            vec![(ALICE, 1), (BOB, 2), (CHAD, 3)],
        ];
        let count = VoteCount::from_ballots(&ballots);
        assert_eq!(count.count, vec![
            vec![0, 1, 1],
            vec![0, 0, 1],
            vec![0, 0, 0],
        ]);
    }
}
