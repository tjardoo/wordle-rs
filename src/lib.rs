use std::collections::HashSet;
use colored::Colorize;

pub mod algorithms;

const DICTIONARY: &str = include_str!("../dictionary.txt");

pub type Word = [u8; 5];

pub struct Wordle {
    dictionary: HashSet<&'static Word>,
}

impl Wordle {
    pub fn new() -> Self {
        Wordle {
            dictionary: HashSet::from_iter(
                DICTIONARY
                    .lines()
                    .map(|line| {
                        line.split_once(" ")
                            .expect("every line is word + space + occurances")
                        .0
                        .as_bytes()
                        .try_into()
                        .expect("every word should be exactly 5 characters")
                    })),
        }
    }

    pub fn play<G: Guesser>(&self, answer: Word, mut guesser: G) -> Option<usize> {
        let mut history = Vec::new();

        for i in 1..=32 {
            let guess = guesser.guess(&history);

            if guess == answer {
                let letters = std::str::from_utf8(&guess).unwrap();

                println!("{}", letters.bold().white().on_green());

                return Some(i);
            }

            assert!(
                self.dictionary.contains(&guess),
                "guess {} is not in the dictionary",
                std::str::from_utf8(&guess).unwrap()
            );

            let correctness = Correctness::compute(answer, guess);

            let letters = std::str::from_utf8(&guess).unwrap();

            for (i, letter) in letters.chars().enumerate() {
                let letter = letter.to_string();

                match correctness[i] {
                    Correctness::Correct => {
                        print!("{}", letter.bold().white().on_green());
                    },
                    Correctness::Misplaced => {
                        print!("{}", letter.bold().white().on_yellow());
                    },
                    Correctness::Wrong => {
                        print!("{}", letter.bold().white().on_bright_black());
                    },
                }
            }
            println!("");

            history.push(Guess {
                word: guess,
                mask: correctness,
            })
        }

        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Correctness {
    Correct,
    Misplaced,
    Wrong,
}

impl Correctness {
    fn compute(answer: Word, guess: Word) -> [Self; 5] {
        assert_eq!(answer.len(), 5);
        assert_eq!(guess.len(), 5);

        let mut c = [Correctness::Wrong; 5];

        for (i, (a, g)) in answer.iter().zip(guess.iter()).enumerate() {
            if a == g {
                c[i] = Correctness::Correct;
            }
        }

        let mut used = [false; 5];

        for (i, &c) in c.iter().enumerate() {
            if c == Correctness::Correct {
                used[i] = true;
            }
        }

        for (i, g) in guess.iter().enumerate() {
            if c[i] == Correctness::Correct {
                continue;
            }

            if answer.iter().enumerate().any(|(i, a) | {
                if a == g && !used[i] {
                    used[i] = true;

                    return true;
                }

                false
            }) {
                c[i] = Correctness::Misplaced;
            }
        }

        c
    }

    pub fn patterns() -> impl Iterator<Item = [Correctness; 5]> {
        itertools::iproduct!(
            [Self::Correct, Self::Misplaced, Self::Wrong],
            [Self::Correct, Self::Misplaced, Self::Wrong],
            [Self::Correct, Self::Misplaced, Self::Wrong],
            [Self::Correct, Self::Misplaced, Self::Wrong],
            [Self::Correct, Self::Misplaced, Self::Wrong]
        ).map(|(a, b, c, d, e)| [a, b, c, d, e])
    }
}

pub struct Guess {
    pub word: Word,
    pub mask: [Correctness; 5],
}

impl Guess {
    pub fn matches(&self, target_word: Word) -> bool {
        // if guess G gives mask C against answer A,
        // then guess A should also give mask C against answer G
        Correctness::compute(target_word, self.word) == self.mask
    }
}

pub trait Guesser {
    fn guess(&mut self, history: &[Guess]) -> Word;
}

impl Guesser for fn(history: &[Guess]) -> Word {
    fn guess(&mut self, history: &[Guess]) -> Word {
        (*self)(history)
    }
}

#[cfg(test)]
macro_rules! guesser {
    (|$history:ident| $impl:block) => {{
        struct G;
        impl $crate::Guesser for G {
            fn guess(&mut self, $history: &[$crate::Guess]) -> $crate::Word {
                $impl
            }
        }
        G
    }};
}

#[cfg(test)]
macro_rules! mask {
    (C) => {$crate::Correctness::Correct};
    (M) => {$crate::Correctness::Misplaced};
    (W) => {$crate::Correctness::Wrong};
    ($($c:tt)+) => {[
        $(mask!($c)),+
    ]}
}

#[cfg(test)]
mod tests {
    mod guess_matcher {
        use crate::Guess;

        macro_rules! check {
            ($prev:literal + [$($mask:tt)+] allows $next:literal) => {
                assert!(Guess {
                    word: *$prev,
                    mask: mask![$($mask )+]
                }
                .matches(*$next));

                // assert_eq!($crate::Correctness::compute($next, $prev), mask![$($mask )+]);
            };

            ($prev:literal + [$($mask:tt)+] disallows $next:literal) => {
                assert!(!Guess {
                    word: *$prev,
                    mask: mask![$($mask )+]
                }
                .matches(*$next));

                // assert_ne!($crate::Correctness::compute($next, $prev), mask![$($mask )+]);
            };
        }

        #[test]
        fn matches() {
            check!(b"abcde" + [C C C C C] allows b"abcde");
            check!(b"abcdf" + [C C C C C] disallows b"abcde");
            check!(b"abcde" + [W W W W W] allows b"fghij");
            check!(b"abcde" + [M M M M M] allows b"bcdea");
            check!(b"aaabb" + [C M W W W] disallows b"accaa");
            check!(b"baaaa" + [W C M W W] allows b"aaccc");
            check!(b"baaaa" + [W C M W W] disallows b"caacc");
            check!(b"baaaa" + [W C M W W] disallows b"caacc");
            check!(b"tares" + [W M M W W] disallows b"brink");
        }
    }

    mod game {
        use crate::Wordle;

        #[test]
        fn genius() {
            let w = Wordle::new();
            let guesser = guesser!(|_history| { *b"right" });

            assert_eq!(w.play(*b"right", guesser), Some(1));
        }

        #[test]
        fn magnificent() {
            let w = Wordle::new();
            let guesser = guesser!(|history| {
                if history.len() == 1 {
                    return *b"right";
                }

                return *b"wrong";
            });

            assert_eq!(w.play(*b"right", guesser), Some(2));
        }

        #[test]
        fn impressive() {
            let w = Wordle::new();
            let guesser = guesser!(|history| {
                if history.len() == 2 {
                    return *b"right";
                }

                return *b"wrong";
            });

            assert_eq!(w.play(*b"right", guesser), Some(3));
        }

        #[test]
        fn splendid() {
            let w = Wordle::new();
            let guesser = guesser!(|history| {
                if history.len() == 3 {
                    return *b"right";
                }

                return *b"wrong";
            });

            assert_eq!(w.play(*b"right", guesser), Some(4));
        }

        #[test]
        fn great() {
            let w = Wordle::new();
            let guesser = guesser!(|history| {
                if history.len() == 4 {
                    return *b"right";
                }

                return *b"wrong";
            });

            assert_eq!(w.play(*b"right", guesser), Some(5));
        }

        #[test]
        fn phew() {
            let w = Wordle::new();
            let guesser = guesser!(|history| {
                if history.len() == 5 {
                    return *b"right";
                }

                return *b"wrong";
            });

            assert_eq!(w.play(*b"right", guesser), Some(6));
        }

        #[test]
        fn oops() {
            let w = Wordle::new();
            let guesser = guesser!(|_history| {
                return *b"wrong";
            });

            assert_eq!(w.play(*b"right", guesser), None);
        }
    }
    mod compute {
        use crate::Correctness;

        #[test]
        fn all_green() {
            assert_eq!(
                Correctness::compute(*b"abcde", *b"abcde"),
                mask![C C C C C],
            );
        }

        #[test]
        fn all_gray() {
            assert_eq!(
                Correctness::compute(*b"abcde", *b"fghij"),
                mask![W W W W W],
            );
        }

        #[test]
        fn all_yellow() {
            assert_eq!(
                Correctness::compute(*b"abcde", *b"eabcd"),
                mask![M M M M M],
            );
        }

        #[test]
        fn repeat_green() {
            assert_eq!(
                Correctness::compute(*b"aabbb", *b"aaccc"),
                mask![C C W W W],
            );
        }

        #[test]
        fn repeat_yellow() {
            assert_eq!(
                Correctness::compute(*b"aabbb", *b"ccaac"),
                mask![W W M M W],
            );
        }

        #[test]
        fn repeat_some_green() {
            assert_eq!(
                Correctness::compute(*b"aabbb", *b"caacc"),
                mask![W C M W W],
            );
        }

        #[test]
        fn repeat_some_yellow() {
            assert_eq!(
                Correctness::compute(*b"azzaz", *b"aaabb"),
                mask![C M W W W],
            );
        }

        #[test]
        fn assert_some_red() {
            assert_eq!(
                Correctness::compute(*b"abcde", *b"aacde"),
                mask![C W C C C],
            );
        }
    }
}
