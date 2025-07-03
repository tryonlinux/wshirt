pub mod models;
pub mod storage;

/// Returns the version string for the program.
pub fn get_version() -> &'static str {
    option_env!("WSHIRT_VERSION").unwrap_or("0.0.1")
}

/// Sanitizes a shirt name by removing commas and newlines.
pub fn sanitize_shirt_name(name: &str) -> String {
    name.trim().replace([',', '\n', '\r'], "")
}

/// Prints an error message to stderr.
pub fn print_error(msg: &str) {
    eprintln!("{}", msg);
}

/// Returns the path to the shirt database file in the user's home directory.
pub fn get_db_path() -> std::path::PathBuf {
    let mut home = dirs::home_dir().expect("Could not find home directory");
    home.push(".wshirt");
    home
}

/// Checks if the required number of arguments is present, prints usage if not.
pub fn require_args(args: &[String], n: usize, usage: &str) -> bool {
    if args.len() < n {
        print_error(usage);
        return false;
    }
    true
}

pub fn handle_add(collection: &mut models::ShirtCollection, name: &str) -> Result<String, String> {
    let shirt_name = sanitize_shirt_name(name);
    if shirt_name.is_empty() {
        return Err("Shirt name cannot be empty.".to_string());
    }
    if collection.list_shirts().iter().any(|s| s.name == shirt_name) {
        return Err(format!("A shirt with the name '{}' already exists.", shirt_name));
    }
    let shirt = models::Shirt {
        name: shirt_name.clone(),
        last_worn: chrono::NaiveDate::from_ymd_opt(1901, 1, 1).unwrap(),
    };
    collection.add_shirt(shirt);
    storage::save_shirts(collection);
    Ok(format!("Shirt '{}' added.", shirt_name))
}

pub fn handle_wear(collection: &mut models::ShirtCollection, arg: &str) -> Result<String, String> {
    let today = chrono::Local::now().date_naive();
    let result = if let Ok(idx) = arg.parse::<usize>() {
        collection.wear_shirt_by_index(idx - 1, today)
    } else {
        collection.wear_shirt_by_name(arg, today)
    };
    if result.is_some() {
        storage::save_shirts(collection);
        Ok(format!("Wore shirt '{}'.", arg))
    } else {
        Err("Shirt not found.".to_string())
    }
}

pub fn handle_delete(collection: &mut models::ShirtCollection, arg: &str) -> Result<String, String> {
    let result = if let Ok(idx) = arg.parse::<usize>() {
        collection.remove_shirt_by_index(idx - 1)
    } else {
        collection.remove_shirt_by_name(arg)
    };
    if result.is_some() {
        storage::save_shirts(collection);
        Ok(format!("Deleted shirt '{}'.", arg))
    } else {
        Err("Shirt not found.".to_string())
    }
}

pub fn handle_reset() -> Result<String, String> {
    let db_path = get_db_path();
    if std::fs::write(&db_path, b"").is_ok() {
        Ok(format!("Shirt database reset ({} cleared).", db_path.display()))
    } else {
        Err("No shirt database found to reset.".to_string())
    }
}
