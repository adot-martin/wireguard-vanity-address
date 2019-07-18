use rayon::prelude::*;
use std::env;
use wireguard_vanity_lib::trial;

fn main() {
    let prefix = env::args().nth(1).unwrap().to_ascii_lowercase();
    let len = prefix.len() as u64;
    const WITHIN: usize = 10;
    let offsets: u64 = (WITHIN as u64) - len;
    let expected: u64 = 2u64.pow(5).pow(len as u32) / offsets;
    println!(
        "prefix: {}, expect {} trials, Ctrl-C to stop",
        prefix, expected
    );

    // 1M trials takes about 10s on my laptop, so let it run for 1000s
    let _: Vec<bool> = (0..100_000_000)
        .into_par_iter()
        .map(|_| trial(&prefix, WITHIN))
        .filter(|good| *good)
        .collect();
}
