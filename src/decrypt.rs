use aes_gcm::{Aes256Gcm, Nonce};
use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::aead::generic_array::GenericArray;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub fn decrypt_files<P: AsRef<Path>>(start_dir: P, key: &[u8], extensions: &[&str]) -> io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    for entry in fs::read_dir(start_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            match decrypt_files(&path, key, extensions) {
                Ok(_) => {},
                Err(e) => {
                    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_bold(true)).unwrap();
                    writeln!(&mut stdout, "Failed to decrypt directory {:?}: {}", path, e).unwrap();
                    stdout.reset().unwrap();
                }
                
            }
        } else if let Some(ext) = path.extension() {
            if extensions.contains(&ext.to_str().unwrap_or("")) {
                match decrypt_file(&path, key) {
                    Ok(_) => println!("Successfully decrypted: {:?}", path),
                    Err(e) => {
                        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_bold(true)).unwrap();
                        writeln!(&mut stdout, "Failed to decrypt file {:?}: {}", path, e).unwrap();
                        stdout.reset().unwrap();
                    }
                }
            }
        }
    }
    Ok(())
}

fn decrypt_file(path: &Path, key: &[u8]) -> io::Result<()> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let key = GenericArray::clone_from_slice(key);
    let cipher = Aes256Gcm::new(&key);
    let nonce = Nonce::from_slice(b"unique nonce");

    let plaintext = cipher.decrypt(nonce, buffer.as_ref())
        .expect("Decryption failure ! File may be corrupted or key is incorrect");

    let mut file = File::create(path)?;
    file.write_all(&plaintext)?;
    let new_path = path.with_extension("");
    fs::rename(path, new_path)?;
    Ok(())
}