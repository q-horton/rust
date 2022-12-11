use std::io;

mod centers;
mod pig_latin;
mod modelling;

fn program_selector() -> u8 {
    println!("Programs with collections.");

    loop {
        let mut choice = String::new();

        println!("Please select a program:");
        println!("A) Mean, Median and Mode");
        println!("B) Pig Latin");
        println!("C) Employee Modelling");

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line.");
        
        match choice.trim().to_uppercase().as_str() {
            "A" => return 0,
            "B" => return 1,
            "C" => return 2,
            value => {
                println!("'{value}' is not an option, please try again.");
            }
        }
    }
}

fn main() {
    let program = program_selector();

    match program {
        0 => centers::main(),
        1 => pig_latin::main(),
        2 => modelling::main(),
        _ => println!("To be built.")
    }
}
