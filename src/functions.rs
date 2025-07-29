pub fn integer_arithmetic() {
    let num1: f32 = 41.0;
    let num2: f32 = 20.0;

    println!("Sum {}", num1 + num2);
    println!("Subtract {}", num1 - num2);
    println!("Multiply {}", num1 * num2);
    println!("Divide {}", num1 / num2);
    println!("Reminder {}", num1 % num2);
}

pub fn safe_overflow_demo() {
    let number: i8 = 127;

    let result = number.checked_mul(6);

    match result {
        Some(val) => {
            println!("value is this: {}", val);
        }
        None => {
            println!("overflow")
        }
    }
}

pub fn temperature_converter() {
    let celsius = 32.59898;
    let fahrenheit = celsius * 9.0 / 5.0 + 32.0;
    let kelvin = celsius + 273.15;

    println!("-------------------------------");
    println!("Calsius {:.3}", celsius);
    println!("Fehr {:.6}", fahrenheit);
    println!("Kel {:.6}", kelvin);
}
