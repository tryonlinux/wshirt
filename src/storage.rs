use crate::models::{Shirt, ShirtCollection};
use chrono::NaiveDate;
use std::fs;
use std::io::{BufRead, BufReader, Write};

pub fn save_shirts(collection: &ShirtCollection) {
    let path = crate::get_db_path();
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    let mut file = fs::File::create(path).expect("Could not create file");
    for (i, shirt) in collection.shirts.iter().enumerate() {
        writeln!(
            file,
            "{},\"{}\",{}",
            i + 1,
            shirt.name,
            shirt.last_worn.format("%Y-%m-%d")
        ).unwrap();
    }
}

pub fn load_shirts() -> ShirtCollection {
    let path = crate::get_db_path();
    if !path.exists() {
        return ShirtCollection::new();
    }
    let file = fs::File::open(path).expect("Could not open file");
    let reader = BufReader::new(file);
    let mut shirts = Vec::new();
    for line in reader.lines() {
        if let Ok(l) = line {
            let parts: Vec<&str> = l.splitn(3, ',').collect();
            if parts.len() != 3 {
                continue;
            }
            let name = parts[1].trim_matches('"').to_string();
            let date = NaiveDate::parse_from_str(parts[2], "%Y-%m-%d");
            match date {
                Ok(date) => {
                    shirts.push(Shirt { name, last_worn: date });
                },
                Err(_e) => {}
            }
        }
    }
    let mut collection = ShirtCollection { shirts };
    collection.shirts.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    collection
}
