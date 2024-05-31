use std::io::{self, IsTerminal, Write};
use clap::{Parser, Subcommand};

#[macro_use] extern crate prettytable;
use prettytable::{Table, Row, Cell};


mod calc;

/// A small tool for number theory
#[derive(Debug, Parser)]
#[command(name = "ffactor")]
#[command(about = "A small tool for number theory", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    
    /// Test if n is even
    #[command(arg_required_else_help = true)]
    Even { 
        /// Number to test
        n: u128
    },
    /// Test if n is square
    Square {
         /// Number to test
        n: u128
    },
    /// Test if n is trinagle
    Triangle {
        /// Number to test
        n: u128
    },
    /// Shows the prime factor descomposition of n
    Factor {
        /// Number to decompose
        n: u128
    },
    /// Tells if n is prime
    Prime {
        n: u128
    },
}


fn main() {

    let args: Cli = Cli::parse();
    let mut stdout = io::stdout().lock();
    let is_terminal = stdout.is_terminal();

    let to_print = match args.command {
        Commands::Even { n } => {
            let r = calc::is_even(n);

            if  r && is_terminal {
                format!("The number {} is even \n", n)
            } else if is_terminal {
                format!("The number {} is odd \n", n)
            } else {
                if r { String::from("1 \n") } else { String::from("0 \n") }
            }
        },
        Commands::Square { n } => {
            let r = calc::is_square(n);

            if  r && is_terminal {
                format!("The number {} is square \n", n)
            } else if is_terminal {
                format!("The number {} is not square \n", n)
            } else {
                if r { String::from("1 \n") } else { String::from("0 \n") }
            }
        }
        Commands::Triangle { n } => {
            let r = calc::is_triangle(n);

            if  r && is_terminal {
                format!("The number {} is triangle \n", n)
            } else if is_terminal {
                format!("The number {} is not triangle \n", n)
            } else {
                if r { String::from("1 \n") } else { String::from("0 \n") }
            }
        }
        Commands::Factor { n } => {
            let mut r = calc::factorize(n);
            let mut keys = r.keys().collect::<Vec<_>>();
            keys.sort();

            if is_terminal {
                let mut table = Table::new();
                table.add_row(row!["Primes", "Times"]);
                for k in keys {
                    table.add_row(row![k, r[k]]);
                }
                table.to_string()
            } else {
                let mut output = String::new();
                for k in keys {
                    output.push_str(&format!("{} {} \n", k, r[k]));
                }
                output
            }
        }
        Commands::Prime { n } => {
            let prime = calc::prime(n) || n == 2;
            if prime && is_terminal {
                format!("The number {} is prime \n", n)
            } else if is_terminal {
                format!("The number {} is not prime \n", n)
            } else if prime {
                "1 \n".to_string()
            } else {
                "0 \n".to_string()
            }
        }
    };
    
    stdout.write(to_print.as_bytes());
    stdout.flush();
}

#[cfg(test)]
mod test;