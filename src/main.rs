use chrono::Local;
use wshirt::{get_version, print_error, require_args, handle_add, handle_wear, handle_delete, handle_reset};

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();
    if !require_args(&args, 2, "Usage: wshirt <command> [options]") {
        return;
    }

    let command = &args[1];
    let mut collection = wshirt::storage::load_shirts();

    match command.as_str() {
        "add" => {
            if !require_args(&args, 3, "Usage: wshirt add <shirt_name>") { return; }
            match handle_add(&mut collection, &args[2]) {
                Ok(msg) => println!("{}", msg),
                Err(e) => print_error(&e),
            }
        }
        "wear" => {
            if !require_args(&args, 3, "Usage: wshirt wear <shirt_name|number>") { return; }
            match handle_wear(&mut collection, &args[2]) {
                Ok(msg) => println!("{}", msg),
                Err(e) => print_error(&e),
            }
        }
        "list" => {
            let mut shirts = collection.list_shirts().clone();
            shirts.sort_by(|a, b| a.last_worn.cmp(&b.last_worn));
            println!("+-----+------------------------------+------------+--------------+");
            println!("|  #  | Name                         | Last Worn  | Days Since   |");
            println!("+-----+------------------------------+------------+--------------+");
            let default_date = chrono::NaiveDate::from_ymd_opt(1901, 1, 1).unwrap();
            let today = Local::now().date_naive();
            for (i, shirt) in shirts.iter().enumerate() {
                let days_since = if shirt.last_worn == default_date {
                    "never".to_string()
                } else {
                    (today - shirt.last_worn).num_days().to_string()
                };
                println!(
                    "| {:>3} | {:<28} | {:<10} | {:<12} |",
                    i + 1,
                    shirt.name.chars().take(28).collect::<String>(),
                    shirt.last_worn,
                    days_since
                );
            }
            println!("+-----+------------------------------+------------+--------------+");
        }
        "delete" => {
            if !require_args(&args, 3, "Usage: wshirt delete <shirt_name|number>") { return; }
            match handle_delete(&mut collection, &args[2]) {
                Ok(msg) => println!("{}", msg),
                Err(e) => print_error(&e),
            }
        }
        "reset" => {
            match handle_reset() {
                Ok(msg) => println!("{}", msg),
                Err(e) => print_error(&e),
            }
        }
        "version" | "--version" | "-v" => {
            println!("wshirt version {}", get_version());
        }
        _ => {
            print_error(&format!("Unknown command: {}", command));
            print_error("Available commands: add, wear, list, delete, reset, version");
        }
    }
}