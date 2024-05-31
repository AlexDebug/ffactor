use std::collections::HashMap;
use std::thread::{self, spawn};
use std::sync::{Mutex, Arc};

pub fn is_even(n: u128) -> bool {
    n % 2 == 0
}

fn is_integer(f: f64) -> bool {
    f.fract() == 0.0
}

pub fn is_square(n: u128) -> bool {
    let r: f64 = (n as f64).sqrt();
    is_integer(r)
}

pub fn is_triangle(n: u128) -> bool {
    is_integer((8.0*(n as f64)+1.0).sqrt()/2.0-0.5)
}

fn fermat(n: u128) -> Vec<u128>{
    let mut vec =Vec::new();

    let mut a = (n as f64).sqrt().ceil() as u128;
        while !is_square(a*a - n) {
            a += 1;
        }
        let b = ((a*a-n) as f64).sqrt() as u128;
    if (a-b) == 1 {
        vec.push(a+b);
        vec
    } else {
        let ar = spawn(move || { return fermat(a+b); });
        let sr = spawn(move || { return fermat(a-b); });
        
        for i in ar.join().unwrap() {
            vec.push(i);
        }
        for i in sr.join().unwrap() {
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

    for k in fermat(n) {
        insert_or_sum(k, &mut factors);
    }
        
    factors
}

pub fn prime(n: u128) -> bool {
    factorize(n).len() == 1 
}