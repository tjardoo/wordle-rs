use wordle::{Wordle, Guesser, Word};
use clap::Parser;

const GAMES: &str = include_str!("../answers.txt");

#[derive(Parser)]
struct Args {
    #[arg(long)]
    implementation: Implementation,

    #[arg(long)]
    max: Option<usize>,
}

#[derive(Clone)]
enum Implementation {
    Allocs,
    Vecrem,
}

impl std::str::FromStr for Implementation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "allocs" => Ok(Self::Allocs),
            "vecrem" => Ok(Self::Vecrem),
            _ => Err(format!("Unknown implementation '{}'", s)),
        }
    }
}

fn main() {
    let args = Args::parse();

    match args.implementation {
        Implementation::Allocs => {
            play(wordle::algorithms::Allocs::new, args.max)
        },
        Implementation::Vecrem => {
            play(wordle::algorithms::Vecrem::new, args.max)
        },
    }
}

fn play<G>(mut mk: impl FnMut() -> G, max: Option<usize>) where G: Guesser {
    let w = Wordle::new();

    for answer in GAMES.split_whitespace().take(max.unwrap_or(usize::MAX)) {
        let answer_b: Word = answer.as_bytes().try_into().expect("all answers are 5 characters");

        let guesser = (mk)();

        if let Some(score) = w.play(answer_b, guesser) {
            println!("guessed '{}' in {}", answer, score);
        } else {
            eprintln!("Failed to guess");
        }
    }
}
