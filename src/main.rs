use std::env;

use nanoserde::SerJson;
use users::{get_current_uid, get_user_by_uid};

#[derive(Debug, SerJson)]
struct Note {
    pub title: String,
    pub description: String,
    pub updated_by: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let note = args.get(1).unwrap();
    dbg!(note);

    let client = reqwest::blocking::Client::new();

    // Thomas:Hydrogen
    let user = get_user_by_uid(get_current_uid()).unwrap();
    let mut username = user.name().to_string_lossy().to_string();
    println!("logged in user: {username}");

    let hostname = hostname::get().unwrap();
    let hostname = hostname.to_str().unwrap();
    println!("hostname: {hostname}");

    username.push(':');
    username.push_str(hostname);

    let note = Note {
        title: note.clone(),
        description: String::from(""),
        updated_by: username,
    };

    let body = SerJson::serialize_json(&note);

    dbg!(body.clone());

    let response = client
        .post("http://localhost:3000/notes/documents/1669928535/groups/1669928538")
        .body(body)
        .send()
        .unwrap();

    dbg!(response);
}
