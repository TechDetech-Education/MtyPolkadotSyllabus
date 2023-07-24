use num_integer;
use primes::{self, PrimeSet};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

pub const MAX_KEY: u32 = u32::MAX - (char::MAX as u32);

pub fn gen_number(p: u32) -> u32 {
    todo!()
}

pub fn gen_prime() -> u64 {
    todo!()
}

pub fn get_primitive_roots(n: u64) -> Vec<u64> {
    todo!()
}

pub fn get_random_pr(p: u32) -> Option<u32> {
    todo!()
}

pub fn is_primitive_root(n: u32) -> bool {
    todo!()
}

fn mod_exp(base: u64, exponent: u64, modulus: u64) -> u64 {
    todo!()
}

// fn gen_prime(min: u64, max: u64) -> u32 {
//     let mut rng = rand::thread_rng();
//     let idx = rng.gen_range(min..=max) as usize;
//     let mut ps = primes::Sieve::new();
//     ps.iter().skip(idx).take(1).collect::<Vec<u64>>()[0]
//         .try_into()
//         .unwrap()
// }
//
// fn mod_exp(base: u32, exponent: u32, modulus: u32) -> u32 {
//     let mut result = 1;
//     let mut base = base % modulus;
//
//     for _ in 0..exponent {
//         result = (result * base) % modulus;
//     }
//
//     result
// }
//
// fn get_primitive_roots(p: u32) -> Vec<u32> {
//     let mut roots = Vec::new();
//     if p % 2 == 0 {
//         return roots;
//     }
//     let phi = p - 1;
//     for q in 2..p {
//         let mut is_primitive_root = true;
//         if num_integer::gcd(q, p) != 1 {
//             continue;
//         }
//         for i in 1..phi {
//             if mod_exp(q, i, p) == 1 {
//                 is_primitive_root = false;
//                 break;
//             }
//         }
//         if is_primitive_root {
//             roots.push(q);
//         }
//     }
//     roots
// }
