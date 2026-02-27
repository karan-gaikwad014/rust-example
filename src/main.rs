use rust_example::{checked_add, welcome};

enum Command {
    Greet { name: String },
    Add { a: i32, b: i32 },
    Help,
}

fn parse_command(args: &[String]) -> Result<Command, String> {
    match args.get(1).map(String::as_str) {
        Some("greet") => {
            let name = args.get(2).cloned().unwrap_or_else(|| "friend".to_string());
            Ok(Command::Greet { name })
        }
        Some("add") => {
            let a = args
                .get(2)
                .ok_or_else(|| "missing first number".to_string())?
                .parse::<i32>()
                .map_err(|_| "first number must be an i32".to_string())?;
            let b = args
                .get(3)
                .ok_or_else(|| "missing second number".to_string())?
                .parse::<i32>()
                .map_err(|_| "second number must be an i32".to_string())?;
            Ok(Command::Add { a, b })
        }
        Some("help") | None => Ok(Command::Help),
        Some(other) => Err(format!("unknown command: {other}")),
    }
}

fn print_help() {
    println!("rust-example\n");
    println!("Usage:");
    println!("  rust-example greet [name]");
    println!("  rust-example add <a> <b>");
    println!("  rust-example help");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match parse_command(&args) {
        Ok(Command::Greet { name }) => match welcome(&name) {
            Ok(msg) => println!("{msg}"),
            Err(err) => {
                eprintln!("error: {err}");
                std::process::exit(2);
            }
        },
        Ok(Command::Add { a, b }) => match checked_add(a, b) {
            Ok(sum) => println!("{sum}"),
            Err(err) => {
                eprintln!("error: {err}");
                std::process::exit(2);
            }
        },
        Ok(Command::Help) => print_help(),
        Err(err) => {
            eprintln!("error: {err}");
            eprintln!("Run `rust-example help` for usage.");
            std::process::exit(2);
        }
    }
}
