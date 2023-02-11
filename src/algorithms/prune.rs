use std::{ops::Neg, borrow::Cow};
use once_cell::sync::OnceCell;

use crate::{Guesser, Guess, DICTIONARY, Correctness, Word};

static INITIAL: OnceCell<Vec<(Word, usize)>> = OnceCell::new();
static PATTERNS: OnceCell<Vec<[Correctness; 5]>> = OnceCell::new();

pub struct Prune {
    remaining: Cow<'static, Vec<(Word, usize)>>,
    patterns: Cow<'static, Vec<[Correctness; 5]>>,
}

impl Prune {
    pub fn new() -> Self {
        Prune {
            remaining: Cow::Borrowed(INITIAL.get_or_init(|| {
                Vec::from_iter(DICTIONARY.lines().map(|line| {
                    let (word, count) = line
                        .split_once(" ")
                        .expect("Every line is word + space + occurances");

                    let word = word.as_bytes().try_into().expect("every dictionary word is exactly 5 characters");

                    let count = count.parse().expect("Every count is a number");

                    (word, count)
                }))
            })),
            patterns: Cow::Borrowed(PATTERNS.get_or_init(|| {
                Correctness::patterns().collect()
            })),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Candidate {
    word: Word,
    goodness: f64,
}

impl Guesser for Prune {
    fn guess(&mut self, history: &[Guess]) -> Word {
        if let Some(last) = history.last() {
            if matches!(self.remaining, Cow::Owned(_)) {
                self.remaining
                    .to_mut()
                    .retain(|&(word, _)| last.matches(word));
            } else {
                self.remaining = Cow::Owned(
                    self.remaining
                        .iter()
                        .filter(|&&(word, _)| last.matches(word))
                        .copied()
                        .collect(),
                );
            }
        }

        if history.is_empty() {
            self.patterns = Cow::Borrowed(PATTERNS.get().unwrap());

            return *b"tares";
        } else {
            assert!(!self.patterns.is_empty());
        }

        let remaining_count: usize = self.remaining.iter().map(|&(_, c)| c).sum();

        let mut best: Option<Candidate> = None;

        for &(word, count) in &*self.remaining {
            let mut sum = 0.0;

            let check_pattern = |pattern: &[Correctness; 5]| {
                let mut in_pattern_total = 0;

                for &(candidate, count) in &*self.remaining {
                    let g = Guess {
                        word,
                        mask: *pattern,
                    };

                    if g.matches(candidate) {
                        in_pattern_total += count;
                    }
                }

                if in_pattern_total == 0 {
                    return false;
                }

                let p_of_this_pattern = in_pattern_total as f64 / remaining_count as f64;
                sum += p_of_this_pattern * p_of_this_pattern.log2();

                return true;
            };

            if matches!(self.patterns, Cow::Owned(_)) {
                self.patterns.to_mut().retain(check_pattern);
            } else {
                self.patterns = Cow::Owned(
                    self.patterns
                        .iter()
                        .copied()
                        .filter(check_pattern)
                        .collect()
                    );
            }

            let p_word = count as f64 / remaining_count as f64;

            let goodness = p_word * sum.neg();

            if let Some(c) = best {
                if goodness > c.goodness {
                    // eprintln!("{:?} is better than {:?} ({} > {})",
                    //     std::str::from_utf8(&word).unwrap(),
                    //     std::str::from_utf8(&c.word).unwrap(),
                    //     goodness,
                    //     c.goodness
                    // );

                    best = Some(Candidate {
                        word,
                        goodness,
                    })
                }
            } else {
                best = Some(Candidate {
                    word,
                    goodness,
                });
            }
        }

        best.unwrap().word
    }
}
