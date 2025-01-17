use figlet_rs::FIGfont;
use colored::*;

use std::io;
use std::io::Write;

fn main() {
  splash();

  loop {
    let input = get_input();
    match input.trim() {
      "test" => test_command(),
      "help" => help(),
      "exit" | "quit" => break,
      _ => println!("\nCommand '{}' not found", input.trim().red().bold())
    }
  }
}

fn help() {
  let commands = ["help - Display this message", "quit - Exit the program"];
  println!("\nCommands:");
  for command in commands.iter() {
    println!("    {}", command);
  }
}

fn test_command() {
  println!("{}", "test command works".green());
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
  let figure = match FIGfont::from_file("resources/univers.flf") {
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
  print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Clear screen and move cursor to 1,1 so splash is displayed at the top
}