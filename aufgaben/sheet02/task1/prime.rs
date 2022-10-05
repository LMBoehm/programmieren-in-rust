//! Aufgabe 2.1: Primzahltest

pub fn is_prime(n: i64) -> bool {
    if n == 2 {
        return true;
    }
    if n <= 1 || n % 2 == 0 {
        return false;
    }
    let sqr = (n as f64).sqrt() as i64;
    for test in (3..sqr + 1).step_by(2) {
        if n % test == 0 {
            return false;
        };
    }
    return true;
}

fn main() {
    for i in 1..20 {
        println!("{}{}", i, if is_prime(i) { "*" } else { "" })
    }
}

#[test]
fn small_primes() {
    assert!(is_prime(2));
    assert!(is_prime(3));
    assert!(is_prime(5));
    assert!(is_prime(7));
}

#[test]
fn small_composites() {
    assert!(!is_prime(1));
    assert!(!is_prime(4));
    assert!(!is_prime(6));
    assert!(!is_prime(8));
    assert!(!is_prime(9));
}

#[test]
fn large_primes() {
    assert!(is_prime(1_300_769));
    assert!(is_prime(1_300_297));
    assert!(is_prime(7_367_287));
}

#[test]
fn large_composites() {
    assert!(!is_prime(908_209));
    assert!(!is_prime(3_073_009));
    assert!(!is_prime(4_897_369));
}
