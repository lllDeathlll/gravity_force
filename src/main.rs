use gravity_force_rust::calc_gravity_force;
use std::io;

fn main() {
    loop {
        println!("Gravitational Force Calculator");

        // Gets variables checks them for errors
        println!("Enter value of m1:");
        let m1 = get_f64();

        println!("Enter value of m2:");
        let m2 = get_f64();

        println!("Enter value of r:");
        let r = get_f64();

        let f = calc_gravity_force(m1, m2, r);

        println!("Gravity force is equal to: {}\n", f)
    }
}

fn get_f64() -> f64 {
    let mut variable = String::new();

    io::stdin()
        .read_line(&mut variable)
        .expect("Failed to read line");
    let variable: f64 = match variable.trim().parse() {
        Ok(num) => num,
        Err(err) => errhandle(err.to_string()),
    };

    variable
}

fn errhandle(err: String) -> f64 {
    // If string is empty sets value to none
    if err == "cannot parse float from empty string" {
        0.0
    }
    // If string is not a number sets value to none
    else if err == "invalid float literal" {
        eprintln!("Value must be a number");
        0.0
    }
    // Panics with error message on other error
    else {
        panic!("Error: {}", err);
    }
}
