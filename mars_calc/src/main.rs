use std::io;

fn main() {
    println!("Please enter your weight in Kgs: ");
    let mut input_weight = String::new();
    io::stdin()
        .read_line(&mut input_weight)
        .expect("Error: Unable to process the input");
    let parsed_weight: f32 = input_weight.trim().parse().expect("Error: Unable to convert input into float");
    println!(
        "Weight on mars is {}kg",
        calculate_weight_on_mars(parsed_weight)
    );
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    return (weight / 9.81) * 3.711;
}
