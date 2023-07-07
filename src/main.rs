use clap::{Parser, Subcommand, ValueEnum};
use primes::{is_prime, PrimeSet, Sieve};

/// Various prime number related utilities in a single tool.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Clone, ValueEnum)]
#[clap(rename_all = "kebab_case")]
enum Bias {
    Next,
    Previous,
}

#[derive(Subcommand)]
enum Commands {
    /// Extract information from a number (factors, prev and next primes.)
    Info {
        /// The number to check
        number: u64,
    },
    /// Check if a number is prime
    Is {
        /// The numbers to check
        numbers: Vec<u64>,
    },
    /// Get the factors of a number
    Factors {
        /// The numbers to get the factors of
        numbers: Vec<u64>,
    },
    /// List the first N prime numbers
    List {
        /// The number of primes to list
        n: usize,
    },

    /// Get the nth prime number
    Nth {
        /// The index of the prime number to get
        n: usize,

        /// The amount of primes to list after the nth prime
        #[arg(short, long, default_value = "1")]
        amount: usize,
    },

    /// Get the next prime number after a given number
    Next {
        /// The number to start searching from
        number: u64,
    },

    /// Get the previous prime number before a given number
    Previous {
        /// The number to start searching from
        number: u64,
    },

    /// Get the closest prime number to a given number
    Closest {
        /// The number to start searching from
        number: u64,

        /// When two choices are provided (e.g. to decide 3 or 5 when given 4)
        /// decide if this should be biased towards choosing the next or previous prime
        #[arg(value_enum, short, long, default_value = "next")]
        bias: Bias,
    },
}

fn previous(value: u64, set: &mut Sieve) -> (usize, u64) {
    for (ix, n) in set.iter().enumerate() {
        if n >= value {
            return (ix - 1, set.get(ix - 1));
        }
    }

    unreachable!();
}

fn main() {
    let cli = Cli::parse();

    let mut prime_set = Sieve::new();

    match &cli.command {
        Commands::Info { number } => {
            println!("Number: {}", *number);
            println!("Prime: {}", is_prime(*number));
            println!("Factors: {:?}", prime_set.prime_factors(*number));
            println!("Previous: {}", previous(*number, &mut prime_set).1);
            println!("Next: {}", prime_set.find(*number).1);
        }
        Commands::Is { numbers } => {
            for n in numbers {
                println!("{}: {}", n, is_prime(*n));
            }
        }
        Commands::Factors { numbers } => {
            for n in numbers {
                println!(
                    "{}: {}",
                    n,
                    prime_set
                        .prime_factors(*n)
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(", ")
                );
            }
        }
        Commands::List { n } => {
            for (ix, n) in prime_set.iter().enumerate().take(*n) {
                println!("{}: {}", ix + 1, n);
            }
        }
        Commands::Nth { n, amount } => {
            for (ix, n) in prime_set
                .iter()
                .enumerate()
                .skip(*n - 1)
                .take(*amount)
            {
                println!("{}: {}", ix + 1, n);
            }
        }
        Commands::Next { number } => {
            let (idx, num) = prime_set.find(*number);
            println!("{}: {}", idx, num);
        }
        Commands::Previous { number } => {
            let (idx, num) = previous(*number, &mut prime_set);
            println!("{}: {}", idx, num);
        }
        Commands::Closest { number, bias } => {
            let (prev_idx, prev_num) = previous(*number, &mut prime_set);
            let (next_idx, next_num) = prime_set.find(*number);

            match bias {
                Bias::Next => {
                    if number - prev_num >= next_num - number {
                        println!("{}: {}", next_idx, next_num);
                    } else {
                        println!("{}: {}", prev_idx, prev_num);
                    }
                }
                Bias::Previous => {
                    if number - prev_num <= next_num - number {
                        println!("{}: {}", prev_idx, prev_num);
                    } else {
                        println!("{}: {}", next_idx, next_num);
                    }
                }
            }
        }
    }
}
