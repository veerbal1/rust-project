// fn main() {
//     let s = String::from("hello");
//     let t = &s;
//     println!("{}", s);
//     println!("{}", t);
// }

// fn takes_ownership(s: &String) { /* print s’s length */ }
// fn main() {
//     let s = String::from("rust");
//     takes_ownership(&s);
//     println!("{}", s);
// }

// fn give_ownership() -> String {
//     String::from("yours")
// }

// fn take_and_give_back(s: String) -> String {
//     s
// }
// fn main() {
//     let s = give_ownership();
//     let x = take_and_give_back(s);
//     println!("{}", x);
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// fn change(s: &mut String) {
//     s.push_str(" world");
// }

// fn print_both(s1: &String, s2: &String) {
//     println!("{}", s1);
//     println!("{}", s2);
// }

// fn test_ownership_borrow(s: &String, s2: String) {
//     println!("{}", s);
//     println!("{}", s2);
// }

// pub fn first_word(s: &String) -> &str {
//     for (index, char) in s.char_indices().enumerate() {
//         if char.1 == ' ' {
//             return &s[..index];
//         }
//     }
//     &s
// }

// pub struct Person { name: String }
// impl Person {
//     fn new(name: String) -> Person { Person { name } }
//     fn name(&self) -> &String { &self.name }
// }

// #[derive(Debug, Copy, Clone)]
// struct Point {
//     x: i32,
//     y: i32,
// }

// struct Excerpt<'a> {
//     part: &'a str,
// }

// fn add_element(mut v: Vec<i32>) -> Vec<i32> {
//     v.push(42);
//     v
//  }

// fn sum(nums: &Vec<i32>) -> i32 {
//     nums.iter().sum()
// }

fn split_words(s: String) -> Vec<String> {
    s.split_whitespace().map(|word| word.to_string()).collect()
 }

fn main() {
    let string = String::from("Hello World");
    let splited_values = split_words(string);
    println!("{:?}", splited_values);
}
