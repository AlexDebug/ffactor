use std::io::{self, IsTerminal, Write};
use calc::prime;
use clap::{Parser, Subcommand};

#[macro_use] extern crate prettytable;
use prettytable::{Table, Row, Cell};


mod calc {
    use std::collections::HashMap;

    pub fn is_even(n: u128) -> bool {
        n % 2 == 0
    }

    pub fn is_square(n: u128) -> bool {
        let r: f64 = (n as f64).sqrt();
        r.fract() == 0.0
    }

    fn fermat(n: u128) -> Vec<u128>{
        let mut vec = Vec::new();

        let mut a = (n as f64).sqrt().ceil() as u128;
            while !is_square(a*a - n) {
                a += 1;
            }
            let b = ((a*a-n) as f64).sqrt() as u128;
        if (a-b) == 1 {
            vec.push(a+b);
            vec
        } else {
            for i in fermat(a+b) {
                vec.push(i);
            }
            for i in fermat(a-b) {
                vec.push(i);
            }
            vec
        }


    }

    fn insert_or_sum(n: u128, map: &mut HashMap<u128,u128>) {
        if map.contains_key(&n) {
            *map.get_mut(&n).unwrap() += 1;
        } else {
            map.insert(n, 1);
        }
    } 

    pub fn factorize(n: u128) -> HashMap<u128, u128> {
        let mut factors: HashMap<u128, u128> = HashMap::new();
        let mut n = n;
        while is_even(n) {
            insert_or_sum(2, &mut factors);
            n = n/2
        }
        let mut ff = fermat(n);
        ff.sort();
        for k in fermat(n) {
            insert_or_sum(k, &mut factors);
        }
            
        factors
    }

    pub fn prime(n: u128) -> bool {
        factorize(n).len() == 1 
    }
}

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
    /// Shows the prime factor descomposition of n
    Factor {
        /// Number to decompose
        n: u128
    },
    /// Tells if a number is prime or not
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
        Commands::Factor { n } => {
            let mut r = calc::factorize(n);

            if is_terminal {
                let mut table = Table::new();
                table.add_row(row!["Primes", "Times"]);
                for (k, v) in r.iter() {
                    table.add_row(row![k, v]);
                }
                table.to_string()
            } else {
                let mut output = String::new();
                for (k, v) in r.iter() {
                    output.push_str(&format!("{} {} \n", k, v));
                }
                output
            }
        }
        Commands::Prime { n } => {
            let prime = prime(n);
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
mod test {
    use super::*;

    #[test]
    fn test_even() {
        let even: [u128; 5] = [2, 4, 8, 6, 14];
        let odd: [u128; 5] = [3, 5, 7, 9, 19];
        for i in 0..5 {
            assert_eq!(calc::is_even(even[i]), true);
            assert_eq!(calc::is_even(odd[i]), false)
        }
    }
    #[test]
    fn test_square() {
        let square: [u128; 3] = [4, 9, 16];
        let no_square: [u128; 3] = [6, 8, 15];
        for i in 0..3 {
            assert_eq!(calc::is_square(square[i]), true);
            assert_eq!(calc::is_square(no_square[i]), false)
        }
    }
}