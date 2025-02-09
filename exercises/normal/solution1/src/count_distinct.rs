use std::collections::HashSet;

pub fn new_count_distinct(input_str: &str) -> usize {
    let mut result_set: HashSet<&str> = HashSet::new();
    // 按逗号分割字符串并将分割后的子字符串插入到 HashSet 中
    for part in input_str.split(',') {
        result_set.insert(part);
    }

    result_set.len()
}
