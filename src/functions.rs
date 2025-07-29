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

pub fn mini_stat_calculator() {
    println!("------------------");
    let arr: [u32; 5] = [4, 5, 6, 2, 1];

    let min: u32 = arr.iter().copied().min().unwrap();
    let max: u32 = arr.iter().copied().max().unwrap();

    println!("Min is {}, Max is {}", min, max);

    let total_sum = arr.iter().sum::<u32>();
    let total_arr_length = arr.len();
    let mean = total_sum / total_arr_length as u32;

    println!("Mean {} ", mean);
    println!("------------------");
}

pub fn tuple_swapper(input: (u32, u32))->(u32, u32) {
    return (input.1, input.0);
}
