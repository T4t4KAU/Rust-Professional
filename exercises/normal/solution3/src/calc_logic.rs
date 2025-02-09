pub fn new_birthday_probability(n: u32) -> f64 {
    calculate_probability(n)
}

fn calculate_probability(num_people: u32) -> f64 {
    if num_people < 2 {
        return 0.0;
    }
    let mut probability_all_different = 1.0;
    for i in 0..num_people {
        probability_all_different *= (365 - i) as f64 / 365.0;
    }
    1.0 - probability_all_different
}