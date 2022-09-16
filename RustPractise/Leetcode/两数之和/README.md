```rust
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

```

# Map 操作

```rust

let mut map = HashMap::new();
map.insert(key, value);

```

# Vector 

```rust
let mut result = Vec::new();
result.push(i as i32);
```

# Struct impl

impl 是 struct 方法的实现，struct 是数据模型的定义。
impl 与 Struct 同名，表示将一系列方法挂载到指定的 Struct 上。

方法调用的时候，要通过struct来调用。


```rust
struct Solution  {}
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) {
    }
}
Solution::two_sum()
```


