use figlet_rs::FIGfont;
use colored::*;
use goddard_propulsion::Engine;

use std::io;
use std::io::Write;

fn main() {
    splash();

    loop {
        let input = get_input();
        match input.trim() {
            "test engine" => test_engine(),
            "help" => help(),
            "exit" | "quit" => break,
            _ => println!("\nCommand '{}' not found", input.trim().red().bold())
        }
    }
}

fn help() {
    let commands = [
        "help - Display this message",
        "test engine - Run engine simulation test",
        "quit - Exit the program"
    ];
    println!("\nCommands:");
    for command in commands.iter() {
        println!("    {}", command);
    }
}

fn test_engine() {
    println!("{}", "\nInitializing engine test...".green());
    let test_engine = Engine::new(
        1000.0, // thrust in Newtons
        300.0,  // ISP in seconds
        2.0,    // mass flow rate in kg/s
    );
    println!("\nEngine specifications:");
    println!("Thrust: {} N", test_engine.thrust());
    println!("ISP: {} s", test_engine.isp());
    println!("Mass flow rate: {} kg/s", test_engine.mass_flow_rate());
    println!("Exhaust velocity: {} m/s", test_engine.exhaust_velocity());
}

fn get_input() -> String {
    let mut input = String::new();
    print!("\n{}", ">>>   ".green());
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input)
        .expect("error: unable to read user input");
    input
}

fn splash() {
    clear_screen();
    // Try relative path first, fallback to plain text if font loading fails
    let figure = match FIGfont::from_file("../../resources/univers.flf") {
        Ok(font) => {
            match font.convert("Goddard 1") {
                Some(fig) => fig.to_string(),
                None => "Goddard 1".to_string()
            }
        },
        Err(_) => "Goddard 1".to_string()
    };
    println!("{}", figure);
}

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
} 