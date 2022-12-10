use rayon::prelude::*;
use std::io::Write;

const MAX_VAL: u32 = u32::MAX;

fn main() -> std::io::Result<()> {
    let mut primes: Vec<u32> = Vec::new();

    let mut curr = 2;

    while curr < MAX_VAL {
        let next = curr.saturating_mul(curr).min(MAX_VAL);
        let mut new_primes: Vec<u32> = (curr..next).into_par_iter().filter(|i| primes.iter().all(|p| i.rem_euclid(*p) > 0)).collect();

        primes.append(&mut new_primes);

        curr = next;
    }

    let stdout_lock = std::io::stdout().lock();
    let mut stdout_writer = std::io::BufWriter::new(stdout_lock);
    for prime in primes.into_iter() {
        stdout_writer.write_fmt(format_args!("{}\n", prime))?;
    }

    Ok(())
}
