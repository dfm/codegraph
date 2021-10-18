use flate2::read::GzDecoder;
use serde_json;
use serde_json::Value;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn read_file<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn Error>> {
    let gz = File::open(path)?;
    let mut reader = BufReader::new(GzDecoder::new(gz));
    read_buffer(&mut reader)
}

pub fn read_buffer<R: BufRead>(reader: &mut R) -> Result<(), Box<dyn Error>> {
    println!("Started reading file");
    let lines: Vec<(u64, u64)> = reader
        .lines()
        .map(|l| l.expect("Couldn't read line"))
        .filter_map(parse_event)
        .collect();
    println!("Finished reading file");
    Ok(())
}

fn parse_number(id: &Value) -> Option<u64> {
    match id {
        Value::Number(num) => num.as_u64(),
        _ => None,
    }
}

fn parse_event(data: String) -> Option<(u64, u64)> {
    let event: Value = {
        if let Ok(parsed) = serde_json::from_str(&data) {
            parsed
        } else {
            return None;
        }
    };
    let actor_id = parse_number(&event["actor"]["id"]);
    let repo_id = parse_number(&event["repo"]["id"]);
    match (actor_id, repo_id) {
        (Some(actor_id), Some(repo_id)) => Some((actor_id, repo_id)),
        _ => None,
    }
}
