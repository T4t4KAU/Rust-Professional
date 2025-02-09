pub fn goldbach_conjecture() -> String {
    let mut res = Vec::new();
    let mut num = 9;

    while res.len() < 2 {
        if !is_prime(num) && num % 2 != 0 {
            if !check(num) {
                res.push(num);
            }
        }
        num += 2;
    }

    let string_numbers = res.iter().map(|&num| num.to_string()).collect::<Vec<String>>();
    let string_res = string_numbers.join(",");

    return string_res;
}

fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }

    for i in 2..=(n as f64).sqrt() as u32 {
        if n % i == 0 {
            return false;
        }
    }

    true
}

fn check(n: u32) -> bool {
    for i in 1.. {
        let square = 2 * i * i;
        if square >= n {
            break;
        }

        let prime = n - square;
        if is_prime(prime) {
            return true;
        }
    }

    false
}