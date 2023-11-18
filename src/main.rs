use std::panic::panic_any;

fn main() {
    println!("{}", get_primes(100).into_iter()
        .fold(
            String::new(),
            |a: String, b: String | a.clone() // i don't yet understand why, but sure
                + (if !a.is_empty() && !b.is_empty() { ", " } else { "" })
                + &b
        )
    )
}

fn get_primes(nums: usize) -> Vec<String> {

    let mut board = vec![true; nums];
    let mut primes = Vec::new();

    for i in 2..=nums {
        match board.get(i) {
            Some(prime) if !prime => continue,
            Some(prime) if *prime => {
                primes.insert(primes.len(), i.to_string());

                for not_prime in (i + 1..nums).step_by(i)  {
                    board.insert(not_prime, false);
                }
            }
            _ => panic_any("Not possible"),
        }
    }

    primes
}
