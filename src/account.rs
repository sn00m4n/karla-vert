use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::Deserialize;

/// Struct for Deserializing Logons and RDP events
#[derive(Debug, Deserialize)]
struct LogonsRDPDeserialized {
    event_id: u32,
    logon_type: String,
    timestamp: DateTime<Utc>,
    description: String,
}

/// Struct for Deserializing AuthenticationEvents
#[derive(Debug, Deserialize)]
struct AuthenticationServiceDeserialized {
    event_id: u32,
    timestamp: DateTime<Utc>,
    description: String,
}
/// Struct for deserializing Profile List Entries
#[derive(Debug, Deserialize)]
struct ProfileListEntry {
    timestamp: DateTime<Utc>,
    sid: String, //using key name instead of Sid:RegBinary for now
    profile_image_path: String,
}

/// new json output for Logon and RDP events
pub fn logons_rdp_karla_vert(input: &str, output: &str) -> Result<()> {
    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);

    let mut entries = vec![];
    for line in reader.lines() {
        let line = line?;

        let entry: LogonsRDPDeserialized = serde_json::from_str(&line)?;
        entries.push(vec![
            entry.event_id.to_string(),
            entry.logon_type,
            entry.timestamp.to_string(),
            entry.description,
        ]);
    }

    let serialized = serde_json::to_string(&entries)?;
    println!("{serialized}");
    let file = File::create(output)?;
    let mut writer = BufWriter::new(file);
    write!(&mut writer, "{}", &serialized)?;
    writer.flush()?;
    Ok(())
}

/// new json output for authentication events
pub fn authentication_service_karla_vert(input: &str, output: &str) -> Result<()> {
    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);

    let mut entries = vec![];
    for line in reader.lines() {
        let line = line?;

        let entry: AuthenticationServiceDeserialized = serde_json::from_str(&line)?;
        entries.push(vec![
            entry.event_id.to_string(),
            entry.timestamp.to_string(),
            entry.description,
        ]);
    }

    let serialized = serde_json::to_string(&entries)?;
    println!("{serialized}");
    let file = File::create(output)?;
    let mut writer = BufWriter::new(file);
    write!(&mut writer, "{}", &serialized)?;
    writer.flush()?;
    Ok(())
}

pub fn profile_list_karla_vert(input: &str, output: &str) -> Result<()> {
    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);

    let mut entries = vec![];
    for line in reader.lines() {
        let line = line?;

        let entry: ProfileListEntry = serde_json::from_str(&line)?;
        entries.push(vec![
            entry.profile_image_path,
            entry.sid,
            entry.timestamp.to_string(),
        ]);
    }

    let serialized = serde_json::to_string(&entries)?;
    println!("{serialized}");
    let file = File::create(output)?;
    let mut writer = BufWriter::new(file);
    write!(&mut writer, "{}", &serialized)?;
    writer.flush()?;
    Ok(())
}
