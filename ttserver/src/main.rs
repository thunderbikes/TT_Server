use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct Entry {
    description: String,
    urgency: i32,
    area: String,
}

impl Entry {
    fn new(description: String, urgency: i32, area: String) -> Self {
        Self {
            description,
            urgency,
            area,
        }
    }
}

struct Dictionary {
    entries: HashMap<i32, Entry>,
}

impl Dictionary {
    fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    fn add_entry(&mut self, code: i32, entry: Entry) {
        self.entries.insert(code, entry);
    }

    fn get_entry(&self, code: i32) -> Option<&Entry> {
        self.entries.get(&code)
    }

    fn remove_entry(&mut self, code: i32) -> Option<Entry> {
        self.entries.remove(&code)
    }
}

#[derive(Debug)]
struct Error {
    description: String,
    time: i64,
    urgency: i32,
    area: String,
}

impl Error {
    fn new(description: String, urgency: i32, area: String) -> Self {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        Self {
            description,
            time,
            urgency,
            area,
        }
    }
    fn from_entry(input: Entry) -> Self {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let description = input.description;
        let urgency = input.urgency;
        let area = input.area;
        Self {
            description,
            time,
            urgency,
            area,
        }
    }
}

struct Data {
    errors: HashMap<i32, Error>,
}

impl Data {
    fn new() -> Self {
        Self {
            errors: HashMap::new(),
        }
    }

    fn add_error(&mut self, error: Error) {
        self.errors.insert(self.errors.len() as i32 + 1, error);
    }

    fn remove_error(&mut self, number: i32) -> Option<Error> {
        self.errors.remove(&number)
    }

    fn make_json(&self) -> String {
        let mut json_str = String::from("{\n");
        for (number, error) in &self.errors {
            json_str.push_str(&format!("    \"{}\": {{\n", number));
            json_str.push_str(&format!("        \"description\": \"{}\",\n", error.description));
            json_str.push_str(&format!("        \"time\": \"{}\",\n", error.time));
            json_str.push_str(&format!("        \"urgency\": \"{}\",\n", error.urgency));
            json_str.push_str(&format!("        \"area\": \"{}\"\n", error.area));
            json_str.push_str("    },\n");
        }
        if self.errors.is_empty() {
            json_str.push_str("}\n");
        } else {
            json_str.pop();
            json_str.pop();
            json_str.push_str("\n}\n");
        }
        json_str
    }
}

fn main() {
    let mut error_dictionary = Dictionary::new();
    let error_1 = Entry::new(String::from("overcurrent"), 0, String::from("BMS"));
    error_dictionary.add_entry(1, error_1);
    let error_2 = Entry::new(String::from("overtemp"), 0, String::from("Motor"));
    error_dictionary.add_entry(2, error_2);
    let error_3 = Entry::new(String::from("overvoltage"), 0, String::from("Battery"));
    error_dictionary.add_entry(3, error_3);

    let data = Arc::new(Mutex::new(Data::new()));
    let dictionary = Arc::new(Mutex::new(error_dictionary));

    
    println!("{}",data.lock().unwrap().make_json());
    
    let test_error = dictionary.lock().unwrap().remove_entry(1).unwrap();
    data.lock().unwrap().add_error(Error::from_entry(test_error));
    println!("{}",data.lock().unwrap().make_json());
    
    let test_error = dictionary.lock().unwrap().remove_entry(2).unwrap();
    data.lock().unwrap().add_error(Error::from_entry(test_error));
    println!("{}",data.lock().unwrap().make_json());

    data.lock().unwrap().remove_error(1);
    println!("{}",data.lock().unwrap().make_json());
    }

