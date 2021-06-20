use core::num;
mod kmp;
mod binary_search;
fn main() {
    // let rs = sum_base(25, 3);
    // println!("{}", rs);

    let mut v = vec![1, 1, 2];
    let x = remove_duplicates(&mut v);

    let haystack = String::from("测试123测试");
    let needle = "测".to_string();
    let res = kmp::Solution::str_str(haystack, needle);
    println!("{:?}", res);
}

/*
给你一个整数 n（10 进制）和一个基数 k ，请你将 n 从 10 进制表示转换为 k 进制表示，计算并返回转换后各位数字的 总和 。

转换后，各位数字应当视作是 10 进制数字，且它们的总和也应当按 10 进制表示返回。

来源：力扣（LeetCode）
链接：https://leetcode-cn.com/problems/sum-of-digits-in-base-k
 */
pub fn sum_base(mut n: i32, k: i32) -> i32 {
    //短除法
    let mut res: i32 = 0;
    while n > 0 {
        res += n % k;
        n /= k;
    }
    res
}

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    match nums.is_empty() {
        true => 0,
        _ => {
            let mut i = 0;
            for j in 1..nums.len() {
                if nums[i] != nums[j] {
                    nums[i + 1] = nums[j];
                    i += 1;
                }
            }
            (i + 1) as i32
        }
    }
}
