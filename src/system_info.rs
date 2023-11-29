use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

use anyhow::Result;
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ComputerName {
    computer_name: String,
}

#[derive(Debug, Deserialize)]
struct PreviousCurrentOSEntry {
    current_build_number: String,
    edition_id: String,
    installation_type: String,
    install_date: NaiveDateTime,
    install_time: DateTime<Utc>,
    path_name: String,
    product_id: String,
    product_name: String,
    registered_organization: String,
    registered_owner: String,
    software_type: String,
}

#[derive(Debug, Deserialize)]
pub struct ShutdownTime {
    shutdown_time: DateTime<Utc>,
}

pub fn computer_name_karla_vert(input: &str, output: &str) -> Result<()> {
    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);

    let mut entries = vec![];
    for line in reader.lines() {
        let line = line?;

        let entry: ComputerName = serde_json::from_str(&line)?;
        entries.push(entry.computer_name);
    }

    let serialized = serde_json::to_string(&entries)?;

    let file = File::create(output)?;
    let mut writer = BufWriter::new(file);
    write!(&mut writer, "{}", &serialized)?;
    writer.flush()?;
    Ok(())
}

pub fn os_karla_vert(input: &str, output: &str) -> Result<()> {
    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);

    let mut entries = vec![];
    for line in reader.lines() {
        let line = line?;

        let entry: PreviousCurrentOSEntry = serde_json::from_str(&line)?;
        entries.push(vec![
            entry.current_build_number,
            entry.edition_id,
            entry.path_name,
            entry.product_id,
            entry.product_name,
            entry.registered_organization,
            entry.registered_owner,
            entry.software_type,
            entry.installation_type,
            entry.install_date.to_string(),
            entry.install_time.to_string(),
        ]);
    }

    let serialized = serde_json::to_string(&entries)?;

    let file = File::create(output)?;
    let mut writer = BufWriter::new(file);
    write!(&mut writer, "{}", &serialized)?;
    writer.flush()?;
    Ok(())
}

pub fn shutdown_karla_vert(input: &str, output: &str) -> Result<()> {
    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);

    let mut entries = vec![];
    for line in reader.lines() {
        let line = line?;

        let entry: ShutdownTime = serde_json::from_str(&line)?;
        entries.push(vec![entry.shutdown_time.to_string()]);
    }

    let serialized = serde_json::to_string(&entries)?;

    let file = File::create(output)?;
    let mut writer = BufWriter::new(file);
    write!(&mut writer, "{}", &serialized)?;
    writer.flush()?;
    Ok(())
}
