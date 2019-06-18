use std::fs;
use std::io::Read;
use std::io::Write;

use clap::{App, Arg, SubCommand};

#[macro_use]
extern crate log;
extern crate env_logger;

const NAME: &'static str = env!("CARGO_PKG_NAME");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");

const CMD_GET_USED_ACCOUNT: &'static str = "get_used_account";
const CMD_RETURN_USED_ACCOUNT: &'static str = "return_used_account";

const BASE_DIR: &'static str = "/home/granary/.granary/";
const LOCK_FILE_NAME: &'static str = "lock";
const KEY_FILE_NAME: &'static str = "key";
const TS_FILE_NAME: &'static str = "ts";

fn main() {
    env_logger::init();
    debug!("Hello, world!");
    let granary_app = App::new(NAME)
        .version(VERSION)
        .author(AUTHORS)
        .about(DESCRIPTION)
        .args(&vec![Arg::from_usage(
            // TODO: Remove or use args
            "-i --interactive 'Enter interactive mode'",
        )])
        .subcommand(SubCommand::with_name(CMD_GET_USED_ACCOUNT))
        .subcommand(SubCommand::with_name(CMD_RETURN_USED_ACCOUNT).args(&vec![
            Arg::from_usage("-p --pub <PUB_KEY> 'The public address of the key'"),
            Arg::from_usage("-P --priv <PRIV_KEY> 'The private key'"),
            Arg::from_usage(
                "-t --transactions <TRANSACTIONS_DATA> 'The transaction data for this key'",
            ),
        ]));

    let matches = granary_app.get_matches();

    if let Some(_matches) = matches.subcommand_matches(CMD_GET_USED_ACCOUNT) {
        debug!("Used account requested");
        _request_used_account().expect("Failed to retreive account");
    } else if let Some(matches) = matches.subcommand_matches(CMD_RETURN_USED_ACCOUNT) {
        debug!("Returning an used account");
        let pub_key = matches.value_of("pub").unwrap().to_string();
        let priv_key = matches.value_of("priv").unwrap().to_string();
        let transactions = matches.value_of("transactions").unwrap().to_string();
        _return_used_account(pub_key, priv_key, transactions).expect("Failed to return account");
    }
}

fn _request_used_account() -> std::io::Result<()> {
    debug!("_request_used_account");
    fs::create_dir_all(BASE_DIR)?;
    debug!("dir created");

    // Loop over existing keys
    for entry in fs::read_dir(&BASE_DIR)? {
        if let Ok(entry) = entry {
            if !entry.path().is_dir() {
                continue;
            }
            // continue if locked
            let mut lock_file = entry.path();
            lock_file.push(&LOCK_FILE_NAME);
            debug!("checking lock {:?}", lock_file);
            if lock_file.exists() {
                debug!("file locked, skipping..");
                continue;
            }
            debug!("file NOT locked");

            // Lock keys
            let _lock = fs::File::create(lock_file)?;
            debug!("lock created");

            // return data
            let mut key_file = entry.path();
            key_file.push(KEY_FILE_NAME);

            let mut key = fs::File::open(key_file)?;
            let mut contents = String::new();
            key.read_to_string(&mut contents)?;
            println!("{}", contents);

            let mut ts_file = entry.path();
            ts_file.push(TS_FILE_NAME);

            let mut ts = fs::File::open(ts_file)?;
            let mut contents = String::new();
            ts.read_to_string(&mut contents)?;
            println!("{}", contents);

            debug!("returned key data");
            break;
        }
    }
    //Err(std::io::Error)
    Ok(())
}

fn _return_used_account(
    pub_key: String,
    priv_key: String,
    transactions: String,
) -> std::io::Result<()> {
    debug!("_return_used_account");
    debug!("{}, {}, {}", pub_key, priv_key, transactions);
    let base_folder = std::path::Path::new(BASE_DIR);
    let key_folder = base_folder.join(&pub_key);
    // Check if key exists
    let key_exists = key_folder.exists();

    // ensure key folder exists
    fs::create_dir_all(&key_folder)?;
    debug!("key folder created");

    let key_file = key_folder.join(KEY_FILE_NAME);
    let ts_file = key_folder.join(TS_FILE_NAME);
    // if existed, compare private key data
    if key_exists {
        debug!("updating key...");
        let mut key = fs::File::open(key_file)?;
        let mut contents = String::new();
        key.read_to_string(&mut contents)?;
        assert!(priv_key == contents);

        let mut ts = fs::File::open(ts_file)?;
        ts.write_all(transactions.as_bytes())?;

        let lock_file = key_folder.join(LOCK_FILE_NAME);
        fs::remove_file(lock_file)?;
    }
    // if new, write private key data
    else {
        debug!("creating key...");
        let mut key = fs::File::create(key_file)?;
        key.write_all(&priv_key.into_bytes())?;

        let mut ts = fs::File::create(ts_file)?;
        ts.write_all(&transactions.into_bytes())?;
    }

    // if existed, remove lock

    // return "OK"
    Ok(())
}
