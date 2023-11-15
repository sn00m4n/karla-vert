mod account;
mod external_usb;
mod system_info;

use anyhow::Result;
use clap::{Parser, Subcommand};
use log::error;

use crate::account::{
    authentication_service_karla_vert, logons_rdp_karla_vert, profile_list_karla_vert,
};
use crate::external_usb::{
    device_karla_vert, hid_karla_vert, mounted_dev_karla_vert, scsi_karla_vert,
    usb_entry_karla_vert, usbstor_entry_karla_vert, vic_karla_vert,
};
use crate::system_info::{computer_name_karla_vert, os_karla_vert, shutdown_karla_vert};

#[derive(Parser)]
#[command(
    author = "ally",
    version = "0.1",
    about = "
 _  __          _          __     __        _
| |/ /__ _ _ __| | __ _    \\ \\   / /__ _ __| |_ ___ _ __
| ' // _` | '__| |/ _` |____\\ \\ / / _ \\ '__| __/ _ \\ '__|
| . \\ (_| | |  | | (_| |_____\\ V /  __/ |  | ||  __/ |
|_|\\_\\__,_|_|  |_|\\__,_|      \\_/ \\___|_|   \\__\\___|_|

Karla-Verter uses default output-names from truffleyard for now!
"
)]
struct Cli {
    /// path where json output is located
    #[arg(short)]
    file_path: String,
    /// output path with name of resulting file
    #[arg(short)]
    output_path: String,
    /// specifying Subcommands
    #[clap(subcommand)]
    command: Commands,
}

/// The different subcommands implemented so far
#[derive(Subcommand)]
enum Commands {
    // Account Usage
    AuthenticationEvents,
    RdpUsage,
    ServiceEvents,
    Logons,
    UserAccounts,
    // External Devices, UBS
    VolumeInfoCache,
    VolumeName,
    Hid,
    MountedDevices,
    Scsi,
    Usb,
    UsbStor,
    // System Information
    ComputerName,
    CurrentVersion,
    OldOsVersions,
    LastShutdownTime,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::AuthenticationEvents => {
            if let Err(err) = authentication_service_karla_vert(&cli.file_path, &cli.output_path) {
                error!("An error occured while converting: {err}")
            }
            Ok(())
        }
        Commands::RdpUsage => {
            if let Err(err) = logons_rdp_karla_vert(&cli.file_path, &cli.output_path) {
                error!("An error occured while converting: {err}")
            }
            Ok(())
        }
        Commands::ServiceEvents => {
            if let Err(err) = authentication_service_karla_vert(&cli.file_path, &cli.output_path) {
                error!("An error occured while converting: {err}")
            }
            Ok(())
        }
        Commands::Logons => {
            if let Err(err) = logons_rdp_karla_vert(&cli.file_path, &cli.output_path) {
                error!("An error occured while converting: {err}")
            }
            Ok(())
        }
        Commands::UserAccounts => {
            if let Err(err) = profile_list_karla_vert(&cli.file_path, &cli.output_path) {
                error!("An error occured while converting: {err}")
            }
            Ok(())
        }
        Commands::VolumeInfoCache => {
            if let Err(err) = vic_karla_vert(&cli.file_path, &cli.output_path) {
                error!("An error occured while converting: {err}")
            }
            Ok(())
        }
        Commands::VolumeName => {
            if let Err(err) = device_karla_vert(&cli.file_path, &cli.output_path) {
                error!("An error occured while converting: {err}")
            }
            Ok(())
        }
        Commands::Hid => {
            if let Err(err) = hid_karla_vert(&cli.file_path, &cli.output_path) {
                error!("An error occured while converting: {err}")
            }
            Ok(())
        }
        Commands::MountedDevices => {
            if let Err(err) = mounted_dev_karla_vert(&cli.file_path, &cli.output_path) {
                error!("An error occured while converting: {err}")
            }
            Ok(())
        }
        Commands::Scsi => {
            if let Err(err) = scsi_karla_vert(&cli.file_path, &cli.output_path) {
                error!("An error occured while converting: {err}")
            }
            Ok(())
        }
        Commands::Usb => {
            if let Err(err) = usb_entry_karla_vert(&cli.file_path, &cli.output_path) {
                error!("An error occured while converting: {err}")
            }
            Ok(())
        }
        Commands::UsbStor => {
            if let Err(err) = usbstor_entry_karla_vert(&cli.file_path, &cli.output_path) {
                error!("An error occured while converting: {err}")
            }
            Ok(())
        }
        Commands::ComputerName => {
            if let Err(err) = computer_name_karla_vert(&cli.file_path, &cli.output_path) {
                error!("An error occured while converting: {err}")
            }
            Ok(())
        }
        Commands::CurrentVersion => {
            if let Err(err) = os_karla_vert(&cli.file_path, &cli.output_path) {
                error!("An error occured while converting: {err}")
            }
            Ok(())
        }
        Commands::OldOsVersions => {
            if let Err(err) = os_karla_vert(&cli.file_path, &cli.output_path) {
                error!("An error occured while converting: {err}")
            }
            Ok(())
        }
        Commands::LastShutdownTime => {
            if let Err(err) = shutdown_karla_vert(&cli.file_path, &cli.output_path) {
                error!("An error occured while converting: {err}")
            }
            Ok(())
        }
    }
}

/*pub fn make_path(path: String) -> Result<String> {
    if fs::metadata(&path).is_err() {
        fs::create_dir(&path)?;
        println!("Created new directory: {}", path);
        Ok(path)
    } else {
        println!("Directory/path exists! Use existing? (Y/n): ");
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        match line.as_ref() {
            "y\n" | "Y\n" | "\n" => {
                println!("Using existing directory/path: {}", path);
                Ok(path)
            }
            "n\n" => {
                println!("Please enter a new path! (complete path): ");
                let mut line = String::new();
                io::stdin().read_line(&mut line)?;
                let path = line.trim().to_string();
                make_path(path.clone())?;
                Ok(path)
            }
            _ => {
                //println!("Invalid input! Exiting.");
                Err(anyhow!("Invalid input. Aborting."))
            }
        }
    }
}*/
