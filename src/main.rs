// Control statements excersises
pub fn check_sign(num: i32) -> &'static str {
    if num == 0 {
        return "zero";
    } else if num > 0 {
        return "positive";
    } else {
        return "negative";
    }
}

pub fn is_in_range(n: i32, low: i32, high: i32) -> bool {
    if n >= low && n <= high { true } else { false }
}

pub fn max(a: i32, b: i32) -> i32 {
    if a > b {
        return a;
    }
    b
}

pub fn countdown(n: u32) {
    let mut i = n;
    while i >= 1 {
        println!("{}", i);
        i -= 1;
    }
    println!("Liftoff!")
}

pub fn print_even(u: u32) {
    for i in 1..=u {
        if i % 2 == 0 {
            println!("{} is even", i);
        }
    }
}

pub fn print_words(words: &[&str]) {
    for word in words.iter() {
        println!("{}", word)
    }
}

pub fn find_first_negative(numbers: &[i32]) -> Option<i32> {
    let mut index = 0;
    loop {
        if index >= numbers.len() {
            break None;
        }
        if numbers[index] < 0 {
            break Some(numbers[index]);
        }
        index += 1;
    }
}

pub fn drain_queue(queue: &mut Vec<i32>) {
    while let Some(val) = queue.pop() {
        println!("{}", val);
    }
}

pub fn print_first_char(s: &str) {
    if let Some(char) = s.chars().next() {
        println!("{}", char)
    } else {
        println!("Empty")
    }
}

pub fn day_name(n: u32) -> &'static str {
    match n {
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 => "Saturday",
        7 => "Sunday",
        _ => "Invalid",
    }
}

pub fn grade(score: u8) -> char {
    match score {
        90..=100 => 'A',
        80..=89 => 'B',
        _ => 'F',
    }
}

pub enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
}

pub fn area(obj: Shape) -> f64 {
    match obj {
        Shape::Circle(val) => 3.14 * val * val,
        Shape::Rectangle(w, h) => w * h,
    }
}

pub fn fizz_buzz(n: u32) {
    match n {
        n if n % 3 == 0 && n % 5 == 0 => println!("FizzBuzz"),
        n if n % 3 == 0 => println!("Fizz"),
        n if n % 5 == 0 => println!("Buzz"),
        _ => println!("sometime"),
    }
}

pub fn classify_point(input: &(u32, u32)) {
    match input {
        (0, 0) => print!("(0, 0)"),
        (0, 1) => print!("(0, 1)"),
        (1, 0) => print!("(1, 0)"),
        (1, 1) => print!("(1, 1)"),
        _ => println!("other"),
    }
}

pub fn first_even(values: &[i32]) -> Option<i32> {
    for val in values.iter() {
        if val % 2 == 0 {
            return Some(*val);
        }
    }
    None
}

pub fn sum_until_limit(limit: u32) {
    let mut sum = 0;
    while sum <= limit {
        println!("Sum: {}", sum);
        sum += 1;
    }
}

pub fn filter_and_sum(nums: &[i32]) -> i32 {
    let mut sum = 0;
    for i in nums.iter() {
        if i % 2 == 0 {
            sum += i
        }
    }
    sum
}

pub fn collatz(num: u128) {
    let mut num2 = num;
    while num2 != 1 {
        if num2 % 2 == 0 {
            num2 = num2 / 2;
            println!("Even {}", num2);
        } else {
            num2 = 3 * num2 + 1;
            println!("Odd {}", num2);
        }
    }
    println!("ended {}", num2)
}

pub fn filter_and_sum2(nums: &[i32]) -> i32 {
    nums.iter()
        .filter(|num| if *num % 2 == 0 { true } else { false })
        .sum()
}

fn main() {
    
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_check_sign() {
        assert_eq!(check_sign(-1), "negative");
        assert_eq!(check_sign(1), "positive");
        assert_eq!(check_sign(-0), "zero");
    }

    #[test]
    fn test_in_range() {
        assert_eq!(is_in_range(5, 2, 6), true);
        assert_eq!(is_in_range(5, 5, 6), true);
        assert_eq!(is_in_range(5, 5, 5), true);
        assert_eq!(is_in_range(1, 5, 6), false);
        assert_eq!(is_in_range(7, 5, 6), false);
    }

    #[test]
    fn test_max() {
        assert_eq!(max(5, 4), 5);
        assert_eq!(max(6, 5), 6);
    }

    #[test]
    fn test_find_first_negative() {
        assert_eq!(find_first_negative(&[1, 2, -1, 3]), Some(-1));
        assert_eq!(find_first_negative(&[-5, 1, 2]), Some(-5));
        assert_eq!(find_first_negative(&[1, 2, 3]), None);
        assert_eq!(find_first_negative(&[]), None);
    }

    #[test]
    fn test_drain_queue() {
        let mut queue = vec![1, 2, 3];
        drain_queue(&mut queue);
        assert!(queue.is_empty());
    }

    #[test]
    fn test_day_name() {
        assert_eq!(day_name(1), "Monday");
        assert_eq!(day_name(7), "Sunday");
        assert_eq!(day_name(0), "Invalid");
        assert_eq!(day_name(8), "Invalid");
    }

    #[test]
    fn test_grade() {
        assert_eq!(grade(95), 'A');
        assert_eq!(grade(100), 'A');
        assert_eq!(grade(90), 'A');
        assert_eq!(grade(85), 'B');
        assert_eq!(grade(89), 'B');
        assert_eq!(grade(80), 'B');
        assert_eq!(grade(79), 'F');
        assert_eq!(grade(0), 'F');
    }

    #[test]
    fn test_area() {
        let circle = Shape::Circle(1.0);
        assert_eq!(area(circle), 3.14);
        let rectangle = Shape::Rectangle(2.0, 3.0);
        assert_eq!(area(rectangle), 6.0);
    }

    #[test]
    fn test_first_even() {
        assert_eq!(first_even(&[1, 3, 5, 2, 4]), Some(2));
        assert_eq!(first_even(&[1, 3, 5]), None);
        assert_eq!(first_even(&[]), None);
        assert_eq!(first_even(&[2, 4, 6]), Some(2));
    }

    #[test]
    fn test_filter_and_sum() {
        assert_eq!(filter_and_sum(&[1, 2, 3, 4, 5, 6]), 12);
        assert_eq!(filter_and_sum(&[1, 3, 5]), 0);
        assert_eq!(filter_and_sum(&[-2, -4, 2, 4]), 0);
        assert_eq!(filter_and_sum(&[]), 0);
    }

    #[test]
    fn test_filter_and_sum2() {
        assert_eq!(filter_and_sum2(&[1, 2, 3, 4, 5, 6]), 12);
        assert_eq!(filter_and_sum2(&[1, 3, 5]), 0);
        assert_eq!(filter_and_sum2(&[-2, -4, 2, 4]), 0);
        assert_eq!(filter_and_sum2(&[]), 0);
    }
}
