use chrono::NaiveDate;

#[derive(Debug, Clone)]
pub struct Shirt {
    pub name: String,
    pub last_worn: NaiveDate,
}

#[derive(Debug, Clone)]
pub struct ShirtCollection {
    pub shirts: Vec<Shirt>,
}

impl ShirtCollection {
    pub fn new() -> Self {
        ShirtCollection { shirts: Vec::new() }
    }

    pub fn add_shirt(&mut self, shirt: Shirt) {
        self.shirts.push(shirt);
        self.shirts.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    }

    pub fn remove_shirt_by_name(&mut self, name: &str) -> Option<Shirt> {
        if let Some(pos) = self.shirts.iter().position(|shirt| shirt.name == name) {
            Some(self.shirts.remove(pos))
        } else {
            None
        }
    }

    pub fn remove_shirt_by_index(&mut self, idx: usize) -> Option<Shirt> {
        if idx < self.shirts.len() {
            Some(self.shirts.remove(idx))
        } else {
            None
        }
    }

    pub fn list_shirts(&self) -> &Vec<Shirt> {
        &self.shirts
    }

    pub fn wear_shirt_by_name(&mut self, name: &str, date: NaiveDate) -> Option<&Shirt> {
        if let Some(shirt) = self.shirts.iter_mut().find(|shirt| shirt.name == name) {
            shirt.last_worn = date;
            Some(shirt)
        } else {
            None
        }
    }

    pub fn wear_shirt_by_index(&mut self, idx: usize, date: NaiveDate) -> Option<&Shirt> {
        if let Some(shirt) = self.shirts.get_mut(idx) {
            shirt.last_worn = date;
            Some(shirt)
        } else {
            None
        }
    }
}