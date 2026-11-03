//! 第 1 课作业 —— 把三个 `todo!()` 换成你的实现，然后运行 `cargo test`。
//!
//! 全对的样子是：`test result: ok. 3 passed`
//! 卡住了就把 `todo!()` 里的字读一遍，那里写着提示。

// 练习 1：自己比较大小，不许用 `std::cmp::max`。
// 提示：`if` 是表达式，可以直接当返回值，末尾不写分号。
fn max(a: i32, b: i32) -> i32 {
    todo!("提示：if a > b {{ a }} else {{ b }} —— 注意分支里不要加分号")
}

// 练习 2：数元音字母个数（只算小写 a e i o u）。
// 提示：`for c in s.chars()` 逐个字符遍历，再 `match c { ... }`。
// 加分项：让大写也算 —— 分支写成 `'a' | 'A' => ...` 试试。
fn count_vowels(s: &str) -> u32 {
    todo!("提示：let mut n = 0; 然后循环里 match，其它字符走 `_ => {{}}`")
}

// 练习 3：除零时返回 None，而不是 panic。
// 提示：`if b == 0 { None } else { Some(a / b) }`。
fn safe_divide(a: i32, b: i32) -> Option<i32> {
    todo!("提示：返回 Option<i32>，所以是 Some(a / b) 而不是 a / b")
}

fn main() {
    println!("max(3, 7) = {}", max(3, 7));
    println!(
        "count_vowels(\"hello world\") = {}",
        count_vowels("hello world")
    );
    println!("safe_divide(10, 2) = {:?}", safe_divide(10, 2));
    println!("safe_divide(1, 0) = {:?}", safe_divide(1, 0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exercise1_max() {
        assert_eq!(max(3, 7), 7);
        assert_eq!(max(7, 3), 7);
        assert_eq!(max(-1, -5), -1);
        assert_eq!(max(2, 2), 2);
    }

    #[test]
    fn exercise2_count_vowels() {
        assert_eq!(count_vowels("hello world"), 3);
        assert_eq!(count_vowels(""), 0);
        assert_eq!(count_vowels("xyz"), 0);
        assert_eq!(count_vowels("aeiou"), 5);
    }

    #[test]
    fn exercise3_safe_divide() {
        assert_eq!(safe_divide(10, 2), Some(5));
        assert_eq!(safe_divide(10, 0), None);
        assert_eq!(safe_divide(-9, 3), Some(-3));
    }
}
