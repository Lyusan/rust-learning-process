fn main() {
    another_function(5);
    print_labeled_measurement(5, 'h');
    println!("The value of five() is: {}", five());
    println!("The value of plus_one(5) is: {}", plus_one(5));
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// function with return value
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}