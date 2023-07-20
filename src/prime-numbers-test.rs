use rand::Rng;

fn mod_pow(base: u64, exponent: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }

    let mut result = 1;
    let base = base % modulus;

    for _ in 0..exponent {
        result = (result * base) % modulus;
    }

    result
}

fn fermat_primality_test(n: u64, k: u32) -> bool {
    if n <= 1 {
        return false;
    }

    if n <= 3 {
        return true;
    }

    let mut rng = rand::thread_rng();

    for _ in 0..k {
        let a = rng.gen_range(2..n - 1);
        if mod_pow(a, n - 1, n) != 1 {
            return false;
        }
    }

    true
}

fn main() {
    let number_to_test = 127; 
    let number_of_tests = 5; 

    if fermat_primality_test(number_to_test, number_of_tests) {
        println!("{} is likely to be prime.", number_to_test);
    } else {
        println!("{} is composite.", number_to_test);
    }
}
