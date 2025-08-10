fn longest(a: &String, b: &String) -> &String {
    if a.len() > b.len() { a } else { b }
}

fn main() {
    let a = String::from("Veerbal");
    let b = String::from("Singh");

    let longest_val = longest(&a, &b);

    println!("A: {}", a);
    println!("B: {}", b);

    println!("{}", longest_val);
}
