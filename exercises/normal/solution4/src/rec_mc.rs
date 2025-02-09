pub fn dp_rec_mc(amount: u32) -> u32 {
    min_num_of_notes(amount)
}

fn min_num_of_notes(amount: u32) -> u32 {
    // 定义纸币面额，按从大到小排序
    let denominations = [100, 50, 30, 20, 10, 5, 2, 1];
    let mut remaining_amount = amount;
    let mut note_count = 0;

    for &denomination in denominations.iter() {
        if remaining_amount == 0 {
            break;
        }
        // 计算当前面额纸币可使用的数量
        let num_notes = remaining_amount / denomination;
        note_count += num_notes;
        // 更新剩余需要找零的金额
        remaining_amount %= denomination;
    }

    note_count
}