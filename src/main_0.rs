fn greet() {
    println!("Hello world!")
}

fn add_2_numebers(a: i32, b: i32) -> i32 {
    a + b
}

fn float_mul(a: f64, b: f64) -> f64 {
    a * b
}

fn is_even(number: i32) -> bool {
    number % 2 == 0
}

fn swap(a: i32, b: i32) -> (i32, i32) {
    (b, a)
}

fn factorial(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return n;
    }
    n * factorial(n - 1)
}

fn increment(n: u32) -> u32 {
    n + 1
}

fn print_length(s: &String) -> usize {
    s.len()
}

fn take_return_len(s: String) -> usize {
    s.len()
}

fn apply(f: fn(i32) -> i32, i: i32) -> i32 {
    f(i)
}

fn add_number(n: i32) -> i32 {
    n + 1
}

fn largest<T: Ord + Clone>(input: &[T]) -> Option<T> {
    if input.len() <= 0 {
        None
    } else {
        let max_val = input.iter().max();
        match max_val {
            Some(val) => Some(val.clone()),
            None => None,
        }
    }
}

fn fib(n: u32) -> u32 {
    if n <= 1 { n } else { fib(n - 2) + fib(n - 1) }
}

fn division(x: u32, y: u32) -> Result<u32, String> {
    if y == 0 {
        Err(String::from("got 0"))
    } else {
        Ok(x / y)
    }
}

fn sum(slice: &[u32]) -> u32 {
    slice.iter().sum()
}

fn div_mod(x: u32, y: u32) -> (u32, u32) {
    (x / y, x % y)
}

struct Point(u32, u32);

fn distance(p: Point) -> (u32, u32) {
    (p.0 - 0, p.1 - 0)
}

fn add_ten(n: &mut u32) -> u32 {
    *n = *n + 10;
    return *n
}

fn longest(string: &[String]) -> Option<&String> {
    if string.len() == 0 {
        None
    } else {
        let mut lengthy = &string[0];
        for word in string {
            if word.len() > lengthy.len() {
                lengthy = word;
            }
        }
        return Some(lengthy);
    }
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<usize> {
    use std::collections::HashMap;

    let mut seen: HashMap<i32, usize> = HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&idx) = seen.get(&complement) {
            return vec![idx, i];
        }
        // store the index of the current number for future complement lookups
        seen.insert(num, i);
    }
    vec![] // return empty vector if no solution is found
}

fn main() {
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        greet(); // should not panic
    }

    #[test]
    fn test_add_2_numbers() {
        assert_eq!(add_2_numebers(2, 3), 5);
    }

    #[test]
    fn test_float_mul() {
        assert!((float_mul(1.5, 2.0) - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_is_even() {
        assert!(is_even(2));
        assert!(!is_even(3));
    }

    #[test]
    fn test_swap() {
        assert_eq!(swap(1, 2), (2, 1));
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(0), 0);
    }

    #[test]
    fn test_increment() {
        assert_eq!(increment(4), 5);
    }

    #[test]
    fn test_print_length() {
        let s = String::from("hello");
        assert_eq!(print_length(&s), 5);
    }

    #[test]
    fn test_take_return_len() {
        let s = String::from("hello");
        assert_eq!(take_return_len(s), 5);
    }

    #[test]
    fn test_apply() {
        assert_eq!(apply(add_number, 5), 6);
    }

    #[test]
    fn test_largest() {
        let arr = [1, 3, 2];
        assert_eq!(largest(&arr), Some(3));
        let empty: [i32; 0] = [];
        assert_eq!(largest(&empty), None);
    }

    #[test]
    fn test_fib() {
        assert_eq!(fib(8), 21);
    }

    #[test]
    fn test_division() {
        assert_eq!(division(10, 2), Ok(5));
        assert!(division(10, 0).is_err());
    }

    #[test]
    fn test_sum() {
        let arr = [1, 2, 3];
        assert_eq!(sum(&arr), 6);
    }

    #[test]
    fn test_div_mod() {
        assert_eq!(div_mod(7, 3), (2, 1));
    }

    #[test]
    fn test_distance() {
        let p = Point(3, 4);
        assert_eq!(distance(p), (3, 4));
    }

    #[test]
    fn test_add_ten() {
        let mut n = 5;
        assert_eq!(add_ten(&mut n), 15);
        assert_eq!(n, 15);
    }

    #[test]
    fn test_longest() {
        let strings = vec![
            String::from("short"),
            String::from("longer"),
            String::from("longest"),
        ];
        let expected = &strings[2];
        assert_eq!(longest(&strings), Some(expected));
    }

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }
}
