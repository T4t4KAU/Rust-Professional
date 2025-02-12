pub fn time_info(time: &str) -> String {
    calculate_time(time)
}

// 判断是否为闰年
fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}
// 获取每个月的天数
fn days_in_month(year: i32, month: i32) -> i32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => panic!("Invalid month"),
    }
}
// 计算当年的第几天
fn day_of_year(year: i32, month: i32, day: i32) -> i32 {
    let mut days = 0;
    for m in 1..month {
        days += days_in_month(year, m);
    }
    days + day
}
// 计算当年剩余天数
fn days_left_in_year(year: i32, month: i32, day: i32) -> i32 {
    let total_days = if is_leap_year(year) { 366 } else { 365 };
    total_days - day_of_year(year, month, day)
}
// 计算星期几，使用蔡氏公式
fn day_of_week(year: i32, month: i32, day: i32) -> i32 {
    let y = if month < 3 { year - 1 } else { year };
    let m = if month < 3 { month + 12 } else { month };
    let c = y / 100;
    let y = y % 100;
    let w = (y + y / 4 + c / 4 - 2 * c + 26 * (m + 1) / 10 + day - 1) % 7;
    if w <= 0 {
        w + 7
    } else {
        w
    }
}
// 计算 ISO 8601 标准的周数
fn week_of_year(year: i32, month: i32, day: i32) -> i32 {
    let first_weekday = day_of_week(year, 1, 1);
    let day_of_year = day_of_year(year, month, day);
    let week = (day_of_year + first_weekday - 1) / 7;
    if first_weekday > 4 {
        week + 1
    } else {
        week
    }
}
// 简单模拟计算距离过年（正月初一）的天数
fn days_to_chinese_new_year(year: i32, month: i32, day: i32) -> i32 {
    let current_day = day_of_year(year, month, day);
    let new_year_day = day_of_year(year, 1, 22);
    if current_day < new_year_day {
        new_year_day - current_day
    } else {
        let next_year = year + 1;
        let next_new_year_day = day_of_year(next_year, 1, 29);
        let days_in_current_year = if is_leap_year(year) { 366 } else { 365 };
        days_in_current_year - current_day + next_new_year_day
    }
}
// 简单模拟计算距离下一次 A 股开盘的天数，假设周一到周五开盘
fn days_to_next_a_share_opening(year: i32, month: i32, day: i32) -> i32 {
    let weekday = day_of_week(year, month, day);
    match weekday {
        7 => 1, // 周日，距离周一开盘 1 天
        5 => 3, // 周五，距离下周一开盘 3 天
        6 => 2, // 周六，距离周一开盘 2 天
        _ => 1, // 周一到周四，距离下一天开盘 1 天
    }
}
fn calculate_time(date_str: &str) -> String {
    let parts: Vec<&str> = date_str.split('-').collect();
    if parts.len() != 3 {
        panic!("Invalid date format, expected YYYY-MM-DD");
    }
    let year: i32 = parts[0].parse().expect("Invalid year");
    let month: i32 = parts[1].parse().expect("Invalid month");
    let day: i32 = parts[2].parse().expect("Invalid day");
    let week_num = week_of_year(year, month, day);
    let weekday = day_of_week(year, month, day);
    let day_of_year = day_of_year(year, month, day);
    let days_left = days_left_in_year(year, month, day);
    let days_to_cny = days_to_chinese_new_year(year, month, day);
    let days_to_a_share = days_to_next_a_share_opening(year, month, day);
    format!(
        "{},{},{},{},{},{}",
        week_num, weekday, day_of_year, days_left, days_to_cny, days_to_a_share
    )
}