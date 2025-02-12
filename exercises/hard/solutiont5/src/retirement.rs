pub fn retire_time(time: &str, tp: &str) -> String {
    let parts: Vec<&str> = time.split('-').collect();
    let year = parts[0].parse::<i32>().unwrap();
    let month = parts[1].parse::<i32>().unwrap();
    
    let res = match tp {
        "男职工" => { get_retire_age_m(year, month) },
        "原法定退休年龄55周岁女职工" => { get_retire_age_w_55(year, month) },
        "原法定退休年龄50周岁女职工" => { get_retire_age_w_50(year, month) },
        _ => { (0, 0, 0.0, 0) }
    };

    let decimal_part = (res.2 - res.2.trunc()).abs();
    if decimal_part < 1e-9 {
        format!("{}-{:02},{:.0},{}", res.0, res.1, res.2, res.3)
    } else {
        format!("{}-{:02},{:.2},{}", res.0, res.1, res.2, res.3)
    }
}

fn get_retire_age_w_50(year: i32, month: i32) -> (i32, i32, f32, i32) {
    let start_year = 1975;
    let start_month = 1;
    let mut retire_age_year = 50;
    let mut retire_age_month = 1;
    let mut retire_year = year;
    let mut retire_month = month;

    if year < 1970 {
        (year + 50, month, 50.0, 0)
    } else if year > 1984 {
        (year + 55, month, 55.0, 60)
    } else {
        let months = (year - start_year) * 12 + month - start_month;
        let num = months / 4 + 1;

        retire_age_month = num;
        retire_age_year += retire_month / 12;
        retire_age_month %= 12;

        let retire_age: f32 = retire_age_year as f32 + (retire_age_month as f32) / 12.0;

        retire_year += retire_age_year;
        retire_month += retire_age_month;
        retire_year += retire_month / 12;
        retire_month %= 12;


        (retire_year, retire_month, retire_age, num)
    }

}
fn get_retire_age_w_55(year: i32, month: i32) -> (i32, i32, f32, i32) {
    let start_year = 1970;
    let start_month = 1;
    let mut retire_age_year = 55;
    let mut retire_age_month = 1;
    let mut retire_year = year;
    let mut retire_month = month;

    if year < 1970 {
        (year + 55, month, 55.0, 0)
    } else if year > 1981 {
        (year + 58, month, 58.0, 36)
    } else {
        let months = (year - start_year) * 12 + month - start_month;
        let num = months / 4 + 1;

        retire_age_month = num;
        retire_age_year += retire_month / 12;
        retire_age_month %= 12;

        let retire_age: f32 = retire_age_year as f32 + (retire_age_month as f32) / 12.0;

        retire_year += retire_age_year;
        retire_month += retire_age_month;
        retire_year += retire_month / 12;
        retire_month %= 12;


        (retire_year, retire_month, retire_age, num)
    }

}

fn get_retire_age_m(year: i32, month: i32) -> (i32, i32, f32, i32) {
    let start_year = 1965;
    let start_month = 1;
    let mut retire_age_year = 60;
    let mut retire_age_month = 1;
    let mut retire_year = year;
    let mut retire_month = month;

    if year < 1965 {
        (year + 60, month, 60.0, 0)
    } else if year > 1976 {
        (year + 63, month, 63.0, 36)
    } else {
        let months = (year - start_year) * 12 + month - start_month;
        let num = months / 4 + 1;

        retire_age_month = num;
        retire_age_year += retire_age_month / 12;
        retire_age_month %= 12;

        let retire_age: f32 = retire_age_year as f32 + (retire_age_month as f32) / 12.0;

        retire_year += retire_age_year;
        retire_month += retire_age_month;
        retire_year += retire_month / 12;
        retire_month %= 12;

        (retire_year, retire_month, retire_age, num)
    }
}