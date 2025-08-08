// fn lit_to_owned() -> String {
//     "veerbal".to_owned()
// }

// fn takes_str_returns_string(s: &str) -> String {
//     s.to_owned()
// }

// fn owned_to_str(s: &String) -> &str {
//     s
// }

// 5
// fn accept_vec_str(a: Vec<&str>) -> Vec<String> {
//     a.iter().map(|val| val.to_string()).collect()
// }

// 6
// fn accept_vec_string(s: Vec<String>) -> Vec<&str> {
//     // Can't return ref. from owned collection in fn as item will be dropped on scope end, and there will be dangling pointer
// }

// use std::collections::HashSet;

// 7
// fn string_pool(s: &[&str]) -> HashSet<String> {
//     let mut pool = HashSet::new();
//     for slice in s {
//         pool.insert(slice.to_string());
//     }
//     pool
// }

// const LIT: &str = "🚀";
// let own = String::from("🚀");

// fn iter(s: &str) -> Vec<char> {
//     s.chars().collect()
// }

// fn describe_str(s: &str) -> String {
//     if s.len() == 0 {
//         return "Empty string".to_string();
//     } else {
//         return format!("String length is {}", s.len());
//     }
// }

// fn first_and_last_sum(numbers: &[i32]) -> Option<i32> {
//     if numbers.len() > 0 {
//         Some(numbers[0] + numbers[numbers.len() - 1])
//     } else {
//         None
//     }
// }

// fn truncate_to_max(s: &str, max: usize) -> &str {
//     &s[..=max]
// }

fn main() {
    
}
