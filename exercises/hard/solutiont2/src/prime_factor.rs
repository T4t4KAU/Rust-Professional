use rand::Rng;

pub fn find_max_prime_factor(number: u128) -> u128 {
    get_max_prime_factor(number)
}

// 快速求模幂
fn mod_exp(mut base: u128, mut exponent: u128, modulus: u128) -> u128 {
    let mut result = 1;
    base %= modulus;
    while exponent > 0 {
        if exponent % 2 == 1 {
            result = mod_mul(result, base, modulus);
        }
        exponent >>= 1;
        base = mod_mul(base, base, modulus);
    }
    result
}

// 防止乘法溢出的模乘法
fn mod_mul(mut a: u128, mut b: u128, modulus: u128) -> u128 {
    let mut result = 0;
    a %= modulus;
    while b > 0 {
        if b % 2 == 1 {
            result = (result + a) % modulus;
        }
        a = (a * 2) % modulus;
        b >>= 1;
    }
    result
}

// 求最大公约数
fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

// Miller-Rabin 素性测试
fn is_prime(n: u128) -> bool {
    if n < 2 {
        return false;
    }
    let small_primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
    for &p in small_primes.iter() {
        if n == p {
            return true;
        }
        if n % p == 0 {
            return false;
        }
    }
    let mut d = n - 1;
    let mut s = 0;
    while d & 1 == 0 {
        d >>= 1;
        s += 1;
    }
    let test_primes = [2, 7, 61];
    for &a in test_primes.iter() {
        if a % n == 0 {
            continue;
        }
        let mut x = mod_exp(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        let mut composite = true;
        for _ in 1..s {
            x = mod_mul(x, x, n);
            if x == n - 1 {
                composite = false;
                break;
            }
        }
        if composite {
            return false;
        }
    }
    true
}

// Pollard Rho 算法求非平凡因子
fn pollard_rho(n: u128) -> u128 {
    if n & 1 == 0 {
        return 2;
    }
    let mut rng = rand::thread_rng();
    let mut x = rng.gen_range(2..n - 1);
    let mut y = x;
    let c = rng.gen_range(1..n);
    let mut d = 1;
    while d == 1 {
        x = (mod_mul(x, x, n) + c) % n;
        y = (mod_mul(y, y, n) + c) % n;
        y = (mod_mul(y, y, n) + c) % n;
        d = gcd(if x > y { x - y } else { y - x }, n);
        if d == n {
            return pollard_rho(n);
        }
    }
    d
}

// 递归求 n 的最大素数因子
fn get_max_prime_factor(n: u128) -> u128 {
    if n == 1 {
        return 1;
    }
    if is_prime(n) {
        return n;
    }
    let factor = pollard_rho(n);
    let f1 = get_max_prime_factor(factor);
    let f2 = get_max_prime_factor(n / factor);
    if f1 > f2 {
        f1
    } else {
        f2
    }
}