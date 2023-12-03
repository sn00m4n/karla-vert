# karla-vert

## Tutorial (Linux)

## prerequisites (if needed):
-> Download and install Rust (using rustup is recommended) on www.rust-lang.org/tools/install \
-> Download and install cargo via package manager (e.g. sudo apt install cargo)\
-> Download git via package manager (e.g. sudo apt install git)

## Download and Install:
1. git clone this repository in desired location
```
git clone https://github.com/sn00m4n/karla-vert.git
```
2. cd into cloned KarlaVert folder
3. install using cargo
```
cargo install --path .
```


## Usage: 
karla\_vert -f \<FILE\_PATH\> -o \<OUTPUT\_PATH\> \<COMMAND\>


## Commands:
authentication-events\
rdp-usage\
service-events\
logons\
user-accounts\
volume-info-cache\
volume-name\
hid\
mounted-devices\
scsi\
usb\
usb-stor\
computer-name\
current-version\
old-os-versions\
last-shutdown-time\
help: Print this message or the help of the given subcommand(s)\

## Options:
-f \<FILE\_PATH\>: path where json output is located\
-o \<OUTPUT\_PATH\>: output path with name of resulting file\
-h, --help: Print help\
-V, --version: Print version\
