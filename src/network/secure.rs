use std::error::Error;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::BufWriter;
use std::env;
use std::path::Path;
use colorful::Colorful;
use colorful::Color;
use libp2p::identity;
use libp2p::identity::Keypair;
use libp2p::identity::ed25519::SecretKey;
use rand::rngs::OsRng;
use web3::signing::keccak256;

pub fn new_secret_key() -> Result<(), Box<dyn Error>> {
    let secret_key = secp256k1::SecretKey::new(&mut OsRng);
    println!("{}", "The secret_key is saved in $HOME/.tochat. Please keep it safe.\n".color(Color::LightRed));
    let s = format!("{}", secret_key.display_secret());
    println!("{}", s);

    let home_path = match env::var("HOME") {
        Ok(path) => path,
        Err(_) => env::var("HOMEPATH").unwrap(),
    };

    let tochat_path = Path::new("/.tochat").to_string_lossy();
    std::fs::create_dir_all(format!("{}{}", home_path, tochat_path))?;

    let secret_path = Path::new("/.tochat/secret.json").to_string_lossy();
    let buf = BufWriter::new(
        OpenOptions::new()
        .write(true)
        .create(true)
        .open(format!("{}{}", home_path, secret_path))?);
    serde_json::to_writer_pretty(buf, &s).unwrap();
    Ok(())
}

pub fn generate_ed25519(key: &String) -> identity::Keypair {
    let mut hash = keccak256(key.as_bytes());
    let secret_key = SecretKey::from_bytes(&mut hash).unwrap();
    Keypair::Ed25519(secret_key.into())
}

pub fn get_secret() -> String {
    let home_path = match env::var("HOME") {
        Ok(path) => path,
        Err(_) => env::var("HOMEPATH").unwrap(),
    };

    let secret_path = Path::new("/.tochat/secret.json").to_string_lossy();
    let buf = BufReader::new(
        OpenOptions::new()
        .read(true)
        .open(format!("{}{}", home_path, secret_path))
        .expect("Please create or import a secret key"));
    serde_json::from_reader(buf).unwrap()
}

pub fn import_secret(key: &String) -> Result<(), Box<dyn Error>> {
    let home_path = match env::var("HOME") {
        Ok(path) => path,
        Err(_) => env::var("HOMEPATH").unwrap(),
    };

    let tochat_path = Path::new("/.tochat").to_string_lossy();
    std::fs::create_dir_all(format!("{}{}", home_path, tochat_path))?;

    let secret_path = Path::new("/.tochat/secret.json").to_string_lossy();
    let buf = BufWriter::new(
        OpenOptions::new()
        .write(true)
        .create(true)
        .open(format!("{}{}", home_path, secret_path))?);
    serde_json::to_writer_pretty(buf, key).unwrap();
    Ok(())
}