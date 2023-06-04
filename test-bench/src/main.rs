use std::cmp::Reverse;

pub fn is_armstrong_number(num: u32) -> bool {
    let mut num_copy = num;
    let num_of_digits = (num as f64).log10() as u32 + 1;
    let mut armstrong_sum: u64 = 0;

    while num_copy > 0 {
        // println!("{}", num_copy);
        let single_digit = num_copy % 10;
        armstrong_sum += (single_digit as u64).pow(num_of_digits);
        num_copy /= 10;
    }

    num as u64 == armstrong_sum
    // let num_string = num.to_string();
    // let calculated_sum: u32 = num_string
    //     .chars()
    //     .map(|c| {
    //         c.to_digit(10)
    //             .unwrap()
    //             .pow(num_string.len().try_into().unwrap())
    //     })
    //     .sum();
    // num == calculated_sum
}

pub fn square_of_sum(n: u32) -> u32 {
    ((n * (n + 1)) / 2).pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    println!("{}", (n * (n + 1) * (2 * n + 1)) / 6);
    (n * (n + 1) * (2 * n + 1)) / 6
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}

pub fn verse(n: u32) -> String {
    // 99 bottles of beer on the wall, 99 bottles of beer.
    // Take one down and pass it around, 98 bottles of beer on the wall.

    match n {
        0 => String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n"),
        1 => String::from("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n"),
        2 => String::from("2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n"),
        _ => format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", n, n, n - 1)
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut output = String::from("");

    for n in (end..=start).rev() {
        output.push_str(&verse(n));
        if n != end {
            output.push_str("\n");
        }
    }
    output
}

pub fn is_leap_year(year: u64) -> bool {
    if year % 4 == 0 {
        if year % 100 == 0 && year % 400 != 0 {
            return false;
        }
        return true;
    }
    false
}

pub fn plants(_diagram: &str, _student: &str) -> Vec<&'static str> {
    _diagram.lines().for_each(f)
    unimplemented!("Solve kindergarten-garden exercise");
}

fn main() {
    // println!("is_armstrong_number {}", is_armstrong_number(153));
    // println!("{}", difference(5))
    // println!("{}", sing(3, 0));
    println!("{}", is_leap_year(400));

    let diagram = "VC
    RC";
    let student = "Alice";
    let expected = vec!["violets", "clover", "radishes", "clover"];
    assert_eq!(plants(diagram, student), expected);
}
