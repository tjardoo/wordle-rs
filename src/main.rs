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
    Once,
    Precalc,
    Weight,
    Prune,
    Cutoff,
}

impl std::str::FromStr for Implementation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "allocs" => Ok(Self::Allocs),
            "vecrem" => Ok(Self::Vecrem),
            "once" => Ok(Self::Once),
            "precalc" => Ok(Self::Precalc),
            "weight" => Ok(Self::Weight),
            "prune" => Ok(Self::Prune),
            "cutoff" => Ok(Self::Cutoff),
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
        Implementation::Once => {
            play(wordle::algorithms::OnceInit::new, args.max)
        },
        Implementation::Precalc => {
            play(wordle::algorithms::Precalc::new, args.max)
        },
        Implementation::Weight => {
            play(wordle::algorithms::Weight::new, args.max)
        },
        Implementation::Prune => {
            play(wordle::algorithms::Prune::new, args.max)
        },
        Implementation::Cutoff => {
            play(wordle::algorithms::Cutoff::new, args.max)
        },
    }
}

fn play<G>(mut mk: impl FnMut() -> G, max: Option<usize>) where G: Guesser {
    let w = Wordle::new();

    let mut score = 0;
    let mut games = 0;

    for answer in GAMES.split_whitespace().take(max.unwrap_or(usize::MAX)) {
        let answer_b: Word = answer.as_bytes().try_into().expect("all answers are 5 characters");

        let guesser = (mk)();

        if let Some(s) = w.play(answer_b, guesser) {
            games += 1;
            score += s;

            println!("guessed '{}' in {}", answer, s);
        } else {
            eprintln!("Failed to guess");
        }
    }

    println!("average score: {:.2}", score as f64 / games as f64);
}
