# RustCrypt
RustCrypt is a command-line tool written in Rust that simulates ransomware behavior, including file encryption and decryption.


# Download 
Download the latest release available here (zip file) : https://github.com/R4z1xx/RustCrypt/releases/latest

# Usage
```
PS C:\> .\RustCrypt.exe --help
        ____             __  ______                 __
       / __ \__  _______/ /_/ ____/______  ______  / /_
      / /_/ / / / / ___/ __/ /   / ___/ / / / __ \/ __/
     / _, _/ /_/ (__  ) /_/ /___/ /  / /_/ / /_/ / /_
    /_/ |_|\__,_/____/\__/\____/_/   \__, / .___/\__/
                                    /____/_/

    Simulates ransomware encryption and exfiltration.


Usage: RustCrypt.exe [OPTIONS]

Options:
  -e, --encrypt    Encrypts files using default/custom key
  -d, --decrypt    Decrypts previously encrypted files
  -k, --key <key>  Encryption/Decryption key
  -h, --help       Print help
  -V, --version    Print version
```
## Simple usage examples
Encryption/decryption key can be found in the "README-RustCrypt.txt" written on the user desktop after encryption.
```
# Start encryption using default key
PS C:\> .\RustCrypt.exe -e

# Start encryption using custom key
PS C:\> .\RustCrypt.exe -e -k "my_secret_key"

# Start decryption
PS C:\> .\RustCrypt.exe -d -k "my_secret_key"
```

# Compilation
You might want to compile the source code on your own, to do so you'll need to have Rust installed on your system. If you haven't installed Rust yet, you can do so by following the instructions [here](https://www.rust-lang.org/tools/install).

Clone this repository and build the project:
```bash
git clone https://github.com/R4z1xx/RustCrypt.git
cd RustCrypt
cargo build --release
```
This will create an executable in the target/release directory.

# License
RustCrypt is released under GNU GPL-3.0. See [LICENSE](LICENSE)
