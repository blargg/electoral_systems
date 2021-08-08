use crate::{Ballot, Candidate, unique_candidates};
use std::collections::HashMap;

/// Instant Runoff Vote. If there are more than one candidates, remove the one with the fewest first
/// choice votes and repeat.
///
/// Requires that each ballot has unique candidates (no repeats).
pub fn instant_runoff_vote(mut ballots: Vec<Ballot>) -> Candidate {
    for ballot in ballots.iter() {
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

fn first_choice_tally(ballots: &Vec<Ballot>) -> HashMap<Candidate, usize> {
    let mut counts = HashMap::new();
    for ballot in ballots {
        *counts.entry(first_choice_candidate(ballot)).or_insert(0) += 1;
    }

    return counts;
}

fn first_choice_candidate(ballot: &Ballot) -> Candidate {
    ballot.iter()
        .max_by_key(|(_, rank)| rank)
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
