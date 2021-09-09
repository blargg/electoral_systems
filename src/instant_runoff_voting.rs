use crate::{unique_candidates, Ballot, BallotSlice, Candidate};
use std::collections::HashMap;

/// Instant Runoff Vote. If there are more than one candidates, remove the one with the fewest first
/// choice votes and repeat.
///
/// Requires that each ballot has unique candidates (no repeats).
pub fn instant_runoff_vote(mut ballots: Vec<Ballot>) -> Candidate {
    for ballot in ballots.iter() {
        // TODO, there may be stricter requirements here.
        assert!(unique_candidates(ballot));
    }

    loop {
        let tally = first_choice_tally(&ballots);

        if tally.len() == 1 {
            // Safe to unwrap, there must be at least 1 key.
            return *tally.keys().next().unwrap();
        }

        let (weakest_candidate, _) = tally.iter().min_by_key(|(_, count)| *count).unwrap();
        remove_candidate(&mut ballots, *weakest_candidate);
    }
}

fn first_choice_tally(ballots: &[Ballot]) -> HashMap<Candidate, usize> {
    let mut counts = HashMap::new();
    for ballot in ballots {
        *counts.entry(first_choice_candidate(ballot)).or_insert(0) += 1;
    }

    counts
}

fn first_choice_candidate(ballot: &BallotSlice) -> Candidate {
    ballot
        .iter()
        .min_by_key(|(_, rank)| rank)
        .unwrap() // should always be called on ballots with candidates still left.
        .0
}

/// Removes the candidate from the ballots.
fn remove_candidate(ballots: &mut Vec<Ballot>, candidate: Candidate) {
    for ballot in ballots {
        let mut i = 0;
        while i < ballot.len() {
            if ballot[i].0 == candidate {
                ballot.swap_remove(i);
            } else {
                // if we didn't remove an element, iterate
                i += 1;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const ALICE: Candidate = Candidate { id: 0 };
    const BOB: Candidate = Candidate { id: 1 };
    const CHAD: Candidate = Candidate { id: 2 };

    #[test]
    fn simple_instant_runoff_vote() {
        let ballots = vec![
            vec![(ALICE, 1), (BOB, 2), (CHAD, 3)],
            vec![(ALICE, 1), (BOB, 2), (CHAD, 3)],
            vec![(CHAD, 1), (BOB, 2), (ALICE, 3)],
            vec![(BOB, 1), (CHAD, 2), (ALICE, 3)],
            vec![(BOB, 1), (CHAD, 2), (ALICE, 3)],
        ];

        let winner = instant_runoff_vote(ballots);
        assert_eq!(winner, BOB);
    }
}
