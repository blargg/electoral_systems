fn main() {
    println!("Hello, world!");
}

pub struct Candidate {
    id: usize,
}

type Rank = i32;

// Multiple candidates with the same rank is allowed
// Should allow for Candidates with no rank (eg. give them the lowest rank to tie)
type Ballot = Vec<(Candidate, Rank)>;

pub fn schulze_election(votes: Vec<Ballot>) -> Candidate {
    for ballot in votes.iter() {
        assert!(valid_ballot(ballot));
    }

    let pref = count_preferences(&votes);
    todo!()
}

fn count_candidates(votes: &Vec<Ballot>) -> usize {
    let mut num_candidates = 0;
    for ballot in votes {
        for (candidate, _) in ballot {
            num_candidates = std::cmp::max(num_candidates, candidate.id);
        }
    }

    return num_candidates;
}

// Pairwise preferences.
// pref[x][y] is the number of voters who prefer candidate x to candidate y.
fn count_preferences(votes: &Vec<Ballot>) -> Vec<Vec<i32>> {
    let num_candidates = count_candidates(votes);
    let mut pref = vec![vec![0; num_candidates]; num_candidates];

    return pref;
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
}
