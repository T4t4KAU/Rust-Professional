pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    sum_of_odd_fibonacci_in_range(1, threshold)
}

fn sum_of_odd_fibonacci_in_range(lower_bound: u32, upper_bound: u32) -> u32 {
    if lower_bound > upper_bound {
        return 0;
    }
    let mut prev = 0;
    let mut current = 1;
    let mut sum = 0;

    while current <= upper_bound {
        if current % 2 != 0 && current >= lower_bound {
            sum += current;
        }
        let next = prev + current;
        prev = current;
        current = next;
    }

    sum
}