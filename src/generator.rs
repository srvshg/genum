use crate::config::{Config, NumberType};
use rand::{seq::IteratorRandom, Rng};

pub fn generate_random_number(config: &Config) -> Result<String, &'static str> {
    if config.min >= config.max {
        return Err("Minimum value must be less than maximum value.");
    }

    let mut rng = rand::thread_rng();

    match config.number_type {
        NumberType::Any => generate_any_number(config, &mut rng),
        NumberType::Odd => generate_odd_number(config, &mut rng),
        NumberType::Even => generate_even_number(config, &mut rng),
        NumberType::Prime => generate_prime_number(config, &mut rng),
    }
}

fn generate_any_number(config: &Config, rng: &mut impl Rng) -> Result<String, &'static str> {
    if config.is_float {
        let number = rng.gen_range(config.min..config.max);
        Ok(format!("{:.2}", number))
    } else {
        let number = rng.gen_range(config.min as i64..config.max as i64);
        Ok(number.to_string())
    }
}

fn generate_odd_number(config: &Config, rng: &mut impl Rng) -> Result<String, &'static str> {
    if config.is_float {
        return Err("Odd/Even selection is not applicable for floating-point numbers.");
    }

    let min = if config.min as i64 % 2 == 0 {
        config.min as i64 + 1
    } else {
        config.min as i64
    };
    let max = config.max as i64;

    // Creating a range iterator for odd numbers
    let odd_numbers = (min..max).filter(|x| x % 2 != 0);

    // Choose a random odd number from the iterator
    match odd_numbers.choose(rng) {
        Some(number) => Ok(number.to_string()),
        None => Err("No odd numbers in the specified range."),
    }
}

fn generate_even_number(config: &Config, rng: &mut impl Rng) -> Result<String, &'static str> {
    if config.is_float {
        return Err("Odd/Even selection is not applicable for floating-point numbers.");
    }

    let min = if config.min as i64 % 2 != 0 {
        config.min as i64 + 1
    } else {
        config.min as i64
    };
    let max = config.max as i64;

    // Creating a range iterator for even numbers
    let even_numbers = (min..max).filter(|x| x % 2 == 0);

    // Choose a random even number from the iterator
    match even_numbers.choose(rng) {
        Some(number) => Ok(number.to_string()),
        None => Err("No even numbers in the specified range."),
    }
}

fn generate_prime_number(config: &Config, rng: &mut impl Rng) -> Result<String, &'static str> {
    let min = config.min as u64;
    let max = config.max as u64;

    let prime_number = (min..max).filter(|&x| is_prime(x)).choose(rng);

    match prime_number {
        Some(number) => Ok(number.to_string()),
        None => Err("No prime numbers in the specified range."),
    }
}

fn is_prime(n: u64) -> bool {
    match n {
        0 | 1 => false,
        2 | 3 => true,
        _ if n % 2 == 0 => false,
        _ => is_prime_miller_rabin(n, 5),
    }
}

fn is_prime_miller_rabin(n: u64, k: u64) -> bool {
    let mut rng = rand::thread_rng();
    let (s, d) = decompose(n - 1);
    'outer: for _ in 0..k {
        let a = rng.gen_range(2..n - 1);
        let mut x = mod_exp(a, d, n);
        if x == 1 || x == n - 1 {
            continue 'outer;
        }
        for _ in 0..s {
            x = mod_exp(x, 2, n);
            if x == n - 1 {
                continue 'outer;
            }
        }
        return false;
    }
    true
}

fn decompose(mut n: u64) -> (u64, u64) {
    let mut s = 0;
    while n % 2 == 0 {
        n /= 2;
        s += 1;
    }
    (s, n)
}

fn mod_exp(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp >>= 1;
        base = (base * base) % modulus;
    }
    result
}
