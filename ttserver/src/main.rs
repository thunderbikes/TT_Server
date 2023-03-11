use std::sync::{Arc, Mutex};
use tokio::{self, task};
use warp::{self, Filter};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, serde::Serialize)]
struct Error {
    description: String,
    time: i64,
    urgency: i32,
    area: String,
}

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
    async fn initialize_state(state: Arc<Mutex<String>>, dictionary: Arc<Mutex<Dictionary>>, data: Arc<Mutex<Data>>) { 
        task::spawn_blocking(move || {
            let error_1 = Entry::new(String::from("overcurrent"), 0, String::from("BMS"));
            error_dictionary.add_entry(1, error_1);
            let error_2 = Entry::new(String::from("overtemp"), 0, String::from("Motor"));
            error_dictionary.add_entry(2, error_2);
            let error_3 = Entry::new(String::from("overvoltage"), 0, String::from("Battery"));
            error_dictionary.add_entry(3, error_3);
    
        let data = Arc::new(Mutex::new(Data::new()));
        let dictionary = Arc::new(Mutex::new(error_dictionary));
    
        //part above stays
        //part below is for testing
        println!("{}",data.lock().unwrap().make_json());
        
        let test_error = dictionary.lock().unwrap().remove_entry(1).unwrap();
        data.lock().unwrap().add_error(Error::from_entry(test_error));
        println!("{}",data.lock().unwrap().make_json());
        
        let test_error = dictionary.lock().unwrap().remove_entry(2).unwrap();
        data.lock().unwrap().add_error(Error::from_entry(test_error));
        println!("{}",data.lock().unwrap().make_json());
    
        data.lock().unwrap().remove_error(1);
        println!("{}",data.lock().unwrap().make_json());
        let mut s = state.lock().unwrap();
        *s = String::from("Initialized state!");
    })
    .await
    .unwrap();
}


#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(String::from("Hello, world!")));
    initialize_state(state.clone()).await;

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let add = warp::path!("add" / i32)
        .map(|add_code| {
            let test_error = dictionary.lock().unwrap().remove_entry(add_code).unwrap();
            data.lock().unwrap().add_error(Error::from_entry(test_error));
            format!("{}", add_code)
        }
    );
    let version = warp::path!("version")
        .map(|| format!("Version: 0.1.0"));
    let goodbye = warp::path!("goodbye" / String)
        .map(|name: String| format!("bye {}!", name));
    warp::serve(add.or(version).or(goodbye))
        .run(([127, 0, 0, 1], 3030))
        .await;
    
}





