use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::{Arc, Mutex};

use hyper::{Body, Method, Request, Response, Server, StatusCode};
use hyper::rt::Future;
use hyper::service::service_fn;

use hyper::rt::run;

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

    fn add_error(&mut self, number: i32, error: Error) {
        self.errors.insert(number, error);
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

    let addr = ([127, 0, 0, 1], 3000).into();

    let server = Server::bind(&addr)
        .serve(move || {
            let data = data.clone();
            let dictionary = dictionary.clone();
            service_fn(move |req| {
                let method = req.method().clone();
                let data = data.clone();
                let dictionary = dictionary.clone();
                let mut response = Response::new(Body::empty());
                match (method, req.uri().path()) {
                    (Method::POST, "/errors") => {
                        let mut data = data.lock().unwrap();
                        let mut dictionary = dictionary.lock().unwrap();
                        let body = hyper::body::to_bytes(req.into_body())
                            .wait()
                            .unwrap();
                        let body_str = String::from_utf8(body.to_vec()).unwrap();
                        let error_number: i32 = body_str.trim().parse().unwrap();
                        let entry = dictionary.get_entry(error_number).unwrap();
                        let error = Error::new(entry.description.clone(), entry.urgency
                            , entry.area.clone());
                        data.add_error(error_number, error);
                        *response.status_mut() = StatusCode::OK;
                    },
                    (Method::DELETE, "/errors") => {
                        let mut data = data.lock().unwrap();
                        let body = hyper::body::to_bytes(req.into_body())
                            .wait()
                            .unwrap();
                        let body_str = String::from_utf8(body.to_vec()).unwrap();
                        let error_number: i32 = body_str.trim().parse().unwrap();
                        if let Some(error) = data.remove_error(error_number) {
                            let mut dictionary = dictionary.lock().unwrap();
                            dictionary.remove_entry(error_number);
                            *response.status_mut() = StatusCode::OK;
                        } else {
                            *response.status_mut() = StatusCode::NOT_FOUND;
                        }
                    },
                    (Method::GET, "/errors") => {
                        let data = data.lock().unwrap();
                        let json_str = data.make_json();
                        *response.body_mut() = Body::from(json_str);
                        *response.status_mut() = StatusCode::OK;
                    },
                    _ => {
                        *response.status_mut() = StatusCode::NOT_FOUND;
                    },
                }
                future::ok(response)
            })
        })
        .map_err(|e| eprintln!("server error: {}", e));
    }

