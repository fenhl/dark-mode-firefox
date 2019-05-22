use which::which;

use std::{
    env,
    fs::File,
    io::{
        self,
        prelude::*
    },
    path::Path,
    process::{
        Command,
        exit
    },
    thread,
    time::Duration
};

const DARK_MODE_PATH: &str = "/usr/local/bin/dark-mode";

/// returns `true` if dark mode is currently enabled
fn get_status() -> Result<bool, io::Error> {
    let dark_mode_path = if Path::new(DARK_MODE_PATH).exists() {
        DARK_MODE_PATH
    } else if which("dark-mode").is_ok() {
        "dark-mode"
    } else {
        return Err(io::ErrorKind::NotFound.into());
    };
    let cmd_output = Command::new(dark_mode_path)
        .arg("status")
        .output()?;
    if !cmd_output.status.success() { return Err(io::ErrorKind::Other.into()); }
    match String::from_utf8_lossy(&cmd_output.stdout).trim() {
        "on" => Ok(true),
        "off" => Ok(false),
        _ => Err(io::ErrorKind::Other.into())
    }
}

fn main() -> Result<(), io::Error> {
    let mut args = env::args();
    let _ = args.next(); // ignore executable name
    let subcommand = args.next().expect("missing subcommand");
    match &subcommand[..] {
        "install" => {
            let mut native_manifest = File::create(dirs::home_dir().expect("failed to determine your home directory").join("Library/Application Support/Mozilla/NativeMessagingHosts/dark_mode.json"))?;
            native_manifest.write_all(include_bytes!("../assets/native-manifest.json"))?;
            //TODO install Firefox extension
        }
        _ => {
            if args.next() == Some("dark_mode@fenhl.net".into()) {
                // assume called from Firefox
                let mut previous = get_status()?;
                webextension_protocol::write_stdout(&previous)?;
                loop {
                    thread::sleep(Duration::from_secs(10));
                    let current = get_status()?;
                    if previous != current {
                        webextension_protocol::write_stdout(&current)?;
                        previous = current;
                    }
                }
            } else {
                eprintln!("dark-mode-firefox: unknown subcommand: {}", subcommand);
                exit(1);
            }
        }
    }
    Ok(())
}
