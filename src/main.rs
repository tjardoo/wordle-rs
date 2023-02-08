use wordle::{Wordle, Guesser};
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
    Naive,
    Allocs,
    Vecrem,
    Once,
}

impl std::str::FromStr for Implementation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "naive" => Ok(Self::Naive),
            "allocs" => Ok(Self::Allocs),
            "vecrem" => Ok(Self::Vecrem),
            "once" => Ok(Self::Once),
            _ => Err(format!("Unknown implementation '{}'", s)),
        }
    }
}

fn main() {
    let args = Args::parse();

    match args.implementation {
        Implementation::Naive => {
            play(wordle::algorithms::Naive::new, args.max)
        },
        Implementation::Allocs => {
            play(wordle::algorithms::Allocs::new, args.max)
        },
        Implementation::Vecrem => {
            play(wordle::algorithms::Vecrem::new, args.max)
        },
        Implementation::Once => {
            play(wordle::algorithms::OnceInit::new, args.max)
        },
    }
}

fn play<G>(mut mk: impl FnMut() -> G, max: Option<usize>) where G: Guesser {
    let w = Wordle::new();

    for answer in GAMES.split_whitespace().take(max.unwrap_or(usize::MAX)) {
        let guesser = (mk)();

        if let Some(score) = w.play(answer, guesser) {
            println!("guessed '{}' in {}", answer, score);
        } else {
            eprintln!("Failed to guess");
        }
    }
}
