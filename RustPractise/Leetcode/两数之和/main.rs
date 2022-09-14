fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    struct Solution {}
    impl Solution {
        pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
            use std::collections::HashMap;
            let mut map = HashMap::new();
            let mut result = Vec::new();
            for (i, v) in nums.iter().enumerate() {
                if map.contains_key(&(target - v)) {
                    result.push(map[&(target - v)] as i32);
                    result.push(i as i32);
                    break;
                }
                map.insert(v, i);
            }
            result
        }
    }

    let result = Solution::two_sum(nums, target);
    println!("{}", result);
}
