extern crate core;

use core::panicking::panic;

fn main() {
    // celsius_to_fahrenheit();
    // fibonacci(42);
    twelve_days();
}

fn celsius_to_fahrenheit() {
    const TEMP_CONVERTER: f64 = 5.0 / 9.0;
    let value = -40.0;
    let unit = 'F';
    let converted_val =
        if unit == 'C' {
            value * TEMP_CONVERTER + 32.0
        } else {
            (value - 32.0) * TEMP_CONVERTER
        };

    println!("The converted value of {value}°{unit} is: {converted_val}°{}", if unit == 'C' { 'F' } else { 'C' })
}

fn fibonacci(n: u32) {
    let mut f1 = 0;
    let mut f2 = 1;
    let mut current = 0;
    for i in 1..=n {
        if i > 2 {}
        current = f1 + f2;
        f2 = f1;
        f1 = current;
        println!("{f1} | {f2}")
    };
    println!("{n}th number of the Fibonacci sequence is: {current}");
}

fn twelve_days() {
    let nth_day = ["First", "Second", "Third", "Fourth", "Fifth", "Sixth", "Seventh", "Eighth", "Ninth", "Tenth", "Eleventh", "Twelfth"];
    let present = [
        "A partridge in a pear tree",
        "Two turtledoves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings (five golden rings)",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "I sent eleven pipers piping",
        "Twelve drummers drumming",
    ];
    // present.reverse();
    // let present = present;

    for day in 0..12 {
        println!("On the {} day of Christmas, my true love sent to me ", nth_day[day]);
        for present_on_day in (0..day).rev() {
            if day == 0 {
                print!("And ");
                println!("{}", present[present_on_day].to_lowercase());
                continue;
            }
            println!("{}", present[present_on_day]);
        }
        println!();
    }
}
