mod utils;

fn main() {
    day1();
}

fn day1() {
    let input = utils::read_input(2024, 1).unwrap();
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    for line in input.lines() {
        let nums: Vec<&str> = line.split_whitespace().collect();
        left_list.push(nums[0].parse().unwrap());
        right_list.push(nums[1].parse().unwrap());
    }

    left_list.sort();
    right_list.sort();

    let mut distance = 0;
    for it in left_list.iter().zip(right_list.iter()) {
        let (left, right) = it;
        distance += i32::abs(left - right);
    }
    utils::print_result(2024, 1, 1, distance.to_string());

    let mut similarity = 0;
    for num in left_list {
        let count = right_list.iter().filter(|&n| *n == num).count() as i32;
        similarity += num * count;
    }
    utils::print_result(2024, 1, 2, similarity.to_string());
}
