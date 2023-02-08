use std::{ops::Neg, borrow::Cow};

use crate::{Guesser, Guess, DICTIONARY, Correctness, Word};

pub struct Vecrem {
    remaining: Vec<(&'static Word, usize)>,
}

impl Vecrem {
    pub fn new() -> Self {
        Vecrem {
            remaining: Vec::from_iter(DICTIONARY.lines().map(|line| {
                    let (word, count) = line
                        .split_once(" ")
                        .expect("Every line is word + space + occurances");

                    let word = word.as_bytes().try_into().expect("every dictionary word is exactly 5 characters");

                    let count = count.parse().expect("Every count is a number");

                    (word, count)
                })),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Candidate {
    word: &'static Word,
    goodness: f64,
}

impl Guesser for Vecrem {
    fn guess(&mut self, history: &[Guess]) -> Word {
        if let Some(last) = history.last() {
            self.remaining.retain(|(word, _)| last.matches(word));
        }

        if history.is_empty() {
            return *b"tares";
        }

        let remaining_count: usize = self.remaining.iter().map(|&(_, c)| c).sum();

        let mut best: Option<Candidate> = None;

        for &(word, _) in &self.remaining {
            // - SUM_i p_i * log_2(p_i)

            let mut sum = 0.0;

            for pattern in Correctness::patterns() {
                let mut in_pattern_total = 0;

                for (candidate, count) in &self.remaining {
                    let g = Guess {
                        word: Cow::Borrowed(word),
                        mask: pattern,
                    };

                    if g.matches(candidate) {
                        in_pattern_total += count;
                    }
                }

                if in_pattern_total == 0 {
                    continue;
                }

               let p_of_this_pattern = in_pattern_total as f64 / remaining_count as f64;
               sum += p_of_this_pattern * p_of_this_pattern.log2();
            }

            let goodness = -sum.neg();

            if let Some(c) = best {
                if goodness > c.goodness {
                    eprintln!("{:?} is better than {:?} ({} > {})",
                        std::str::from_utf8(word).unwrap(),
                        std::str::from_utf8(c.word).unwrap(),
                        goodness,
                        c.goodness
                    );

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

        *best.unwrap().word
    }
}
