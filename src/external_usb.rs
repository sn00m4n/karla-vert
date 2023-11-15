use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::Deserialize;

/// Struct for deserializing VIC entries
#[derive(Debug, Deserialize)]
struct VicEntry {
    timestamp: DateTime<Utc>,
    drive_name: String,
    vol_label: String,
    drive_type: String,
}

#[derive(Debug, Deserialize)]
struct Device {
    time_stamp: DateTime<Utc>,
    vendorname: String,
    productname: String,
    version: String,
    serialnumber: String,
    friendly_name: String,
}

#[derive(Debug, Deserialize)]
struct HdiEntry {
    vendor_id: String,
    product_id: String,
    vendorname: String,
    productname: String,
    serialnumber: String,
    first_connected: DateTime<Utc>,
    last_connected: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct MountedDevice {
    device_name: String, // key value name
    vendorname: String,
    productname: String,
    revision: String,
    serial: String,
}

#[derive(Debug, Deserialize)]
struct ScsiEntry {
    manufacturer: String,
    title: String,
    device_name: String,
    first_connected: DateTime<Utc>,
    last_connected: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
struct UsbEntry {
    vid: String,
    pid: String,
    vendorname: String,
    productname: String,
    serial_number: String,
    friendly_name: String,
    location_information: String,
    time_stamp: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
struct UsbStorEntry {
    manufacturer: String,
    title: String,
    version: String,
    serial_number: String,
    device_name: String,
    first_connected: DateTime<Utc>,
    last_connected: DateTime<Utc>,
    last_removed: DateTime<Utc>,
}

/// new json output for vic entries
pub fn vic_karla_vert(input: &str, output: &str) -> Result<()> {
    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);

    let mut entries = vec![];
    for line in reader.lines() {
        let line = line?;

        let entry: VicEntry = serde_json::from_str(&line)?;
        entries.push(vec![
            entry.drive_name,
            entry.drive_type,
            entry.vol_label,
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

pub fn device_karla_vert(input: &str, output: &str) -> Result<()> {
    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);

    let mut entries = vec![];
    for line in reader.lines() {
        let line = line?;

        let entry: Device = serde_json::from_str(&line)?;
        entries.push(vec![
            entry.friendly_name,
            entry.vendorname,
            entry.productname,
            entry.version,
            entry.serialnumber,
            entry.time_stamp.to_string(),
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

pub fn hid_karla_vert(input: &str, output: &str) -> Result<()> {
    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);

    let mut entries = vec![];
    for line in reader.lines() {
        let line = line?;

        let entry: HdiEntry = serde_json::from_str(&line)?;
        entries.push(vec![
            entry.vendor_id,
            entry.vendorname,
            entry.product_id,
            entry.productname,
            entry.serialnumber,
            entry.first_connected.to_string(),
            entry.last_connected.to_string(),
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

pub fn mounted_dev_karla_vert(input: &str, output: &str) -> Result<()> {
    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);

    let mut entries = vec![];
    for line in reader.lines() {
        let line = line?;

        let entry: MountedDevice = serde_json::from_str(&line)?;
        entries.push(vec![
            entry.device_name,
            entry.vendorname,
            entry.productname,
            entry.revision,
            entry.serial,
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

pub fn scsi_karla_vert(input: &str, output: &str) -> Result<()> {
    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);

    let mut entries = vec![];
    for line in reader.lines() {
        let line = line?;

        let entry: ScsiEntry = serde_json::from_str(&line)?;
        entries.push(vec![
            entry.title,
            entry.device_name,
            entry.manufacturer,
            entry.first_connected.to_string(),
            entry.last_connected.to_string(),
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

pub fn usb_entry_karla_vert(input: &str, output: &str) -> Result<()> {
    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);

    let mut entries = vec![];
    for line in reader.lines() {
        let line = line?;

        let entry: UsbEntry = serde_json::from_str(&line)?;
        entries.push(vec![
            entry.vid,
            entry.vendorname,
            entry.pid,
            entry.productname,
            entry.friendly_name,
            entry.serial_number,
            entry.location_information,
            entry.time_stamp.to_string(),
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

pub fn usbstor_entry_karla_vert(input: &str, output: &str) -> Result<()> {
    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);

    let mut entries = vec![];
    for line in reader.lines() {
        let line = line?;

        let entry: UsbStorEntry = serde_json::from_str(&line)?;
        entries.push(vec![
            entry.title,
            entry.manufacturer,
            entry.device_name,
            entry.serial_number,
            entry.version,
            entry.first_connected.to_string(),
            entry.last_connected.to_string(),
            entry.last_removed.to_string(),
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
