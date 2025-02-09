use std::collections::HashMap;

// 定义一个字符到数字的映射表
fn char_to_digit_map() -> HashMap<char, u32> {
    let mut map = HashMap::new();
    for (i, c) in "0123456789abcdef".chars().enumerate() {
        map.insert(c, i as u32);
    }
    map
}
// 定义一个数字到字符的映射表
fn digit_to_char_map() -> HashMap<u32, char> {
    let mut map = HashMap::new();
    for (i, c) in "0123456789abcdef".chars().enumerate() {
        map.insert(i as u32, c);
    }
    map
}
// 将任意进制的字符串转换为十进制数
fn to_decimal(num_str: &str, from_base: u32) -> Result<u32, &'static str> {
    if from_base < 2 || from_base > 16 {
        return Err("Invalid base, must be between 2 and 16");
    }
    let char_to_digit = char_to_digit_map();
    let mut decimal_num = 0;
    for (i, c) in num_str.chars().rev().enumerate() {
        if let Some(digit) = char_to_digit.get(&c) {
            if *digit >= from_base {
                return Err("Invalid digit for the given base");
            }
            decimal_num += digit * from_base.pow(i as u32);
        } else {
            return Err("Invalid character in the number string");
        }
    }
    Ok(decimal_num)
}
// 将十进制数转换为任意进制的字符串
fn from_decimal(decimal_num: u32, to_base: u32) -> Result<String, &'static str> {
    if to_base < 2 || to_base > 16 {
        return Err("Invalid base, must be between 2 and 16");
    }
    let digit_to_char = digit_to_char_map();
    if decimal_num == 0 {
        return Ok("0".to_string());
    }
    let mut num_str = String::new();
    let mut num = decimal_num;
    while num > 0 {
        let remainder = num % to_base;
        if let Some(c) = digit_to_char.get(&remainder) {
            num_str.insert(0, *c);
        }
        num /= to_base;
    }
    Ok(num_str)
}
// 实现任意 2 - 16 进制数之间的转换
fn convert_base_(num_str: &str, from_base: u32, to_base: u32) -> Result<String, &'static str> {
    let decimal_num = to_decimal(num_str, from_base)?;
    from_decimal(decimal_num, to_base)
}

fn extract_parts(s: &str) -> Option<(&str, &str)> {
    // 查找 '(' 的位置
    if let Some(start_index) = s.find('(') {
        // 查找 ')' 的位置
        if let Some(end_index) = s.find(')') {
            if end_index > start_index {
                // 提取 x 部分
                let x = &s[0..start_index];
                // 提取 y 部分
                let y = &s[start_index + 1..end_index];
                return Some((x, y));
            }
        }
    }
    None
}

pub fn convert_base(num_str: &str, to_base: u32) -> String {
    if let Some(parts) = extract_parts(num_str) {
        match parts.1.parse::<u32>() {
            Ok(num) => convert_base_(parts.0, num, to_base).unwrap(),
            Err(_) => return "".to_string(),
        }
    } else {
        return "".to_string();
    }
}
