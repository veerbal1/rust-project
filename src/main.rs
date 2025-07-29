mod functions;
fn main() {
    functions::integer_arithmetic();
    functions::safe_overflow_demo();
    functions::temperature_converter();

    functions::mini_stat_calculator();

    let swap = functions::tuple_swapper((5, 3));
    println!("{:?}", swap);

    functions::rgb_to_grayscale();
}
