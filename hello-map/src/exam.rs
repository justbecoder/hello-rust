use std::collections::HashMap;

// 寻找列表中的中位数
pub fn find_median(nums: &mut Vec<i32>) -> f64 {
    let len = nums.len();

    // 排序
    nums.sort();

    // 奇数还是偶数
    if len % 2 == 0 {
        let mid_left = nums[len / 2 - 1];
        let mid_right = nums[len / 2];
        return (mid_left + mid_right) as f64 / 2.0;
    } else {
        return nums[len / 2] as f64;
    }
}

// 获取列表的众数
pub fn find_mode(nums: &Vec<i32>) -> Vec<i32> {
    let mut frequency_map: HashMap<i32, i32> = HashMap::new();

    let mut max_freq = 0;

    // 计算每个数字出现的频率
    for &num in nums.iter() {
        let count = frequency_map.entry(num).or_insert(0);
        *count += 1;

        max_freq = max_freq.max(*count);
    }

    let mut modes: Vec<i32> = Vec::new();
    for (&num, &freq) in &frequency_map {
        if freq == max_freq {
            modes.push(num);
        }
    }
    return modes;
}
