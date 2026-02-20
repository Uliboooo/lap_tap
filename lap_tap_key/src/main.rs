use clap::Parser;
use std::{
    io::{BufRead, BufReader, Write},
    path::PathBuf,
    process::Stdio,
    sync::{Arc, atomic::AtomicBool},
};

use evdev::EventSummary;

#[derive(Debug, Parser)]
struct Cli {
    #[arg(short = 'a', long = "audio_p_path", help = "sounds binaly path")]
    audio_process_bin_path: Option<String>,
}

impl Cli {
    fn resolve_sound_bin_path(&self) -> Result<PathBuf, std::io::Error> {
        let res = match &self.audio_process_bin_path {
            Some(v) => PathBuf::from(v),
            None => std::env::home_dir()
                .unwrap()
                .join(".local")
                .join("bin")
                .join("lap_tap")
                .join("lap_tap_audio"),
        };
        Ok(res)
    }
    fn resolve_sound_src_path(&self) -> Result<PathBuf, std::io::Error> {
        let res = match &self.audio_process_bin_path {
            Some(v) => PathBuf::from(v),
            None => std::env::home_dir()
                .unwrap()
                .join(".local")
                .join("bin")
                .join("lap_tap")
                .join("resources"),
        };
        Ok(res)
    }
}

fn main() {
    let cli = Cli::parse();

    let device_path = "/dev/input/event4";
    let mut device = evdev::Device::open(device_path).unwrap();
    println!("Watching device: {}", device.name().unwrap_or("Unknown"));

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, std::sync::atomic::Ordering::SeqCst);
    })
    .unwrap();

    let sound_process_bin_path = cli
        .resolve_sound_bin_path()
        .unwrap()
        .to_string_lossy()
        .to_string();

    let resouce_path = cli
        .resolve_sound_src_path()
        .unwrap()
        .to_string_lossy()
        .to_string();

    let mut child = std::process::Command::new("sudo")
        // use 1000 for using audio server
        .args([
            "-u",
            "#1000",
            "env",
            "XDG_RUNTIME_DIR=/run/user/1000",
            &sound_process_bin_path,
            &resouce_path,
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let mut stdin = child.stdin.take().unwrap();
    let stdout = child.stdout.take().unwrap();
    let mut out_reader = BufReader::new(stdout);

    while running.load(std::sync::atomic::Ordering::SeqCst) {
        for ev in device.fetch_events().unwrap() {
            match ev.destructure() {
                EventSummary::Key(_, key, value) => {
                    let state = match value {
                        0 => "Released".to_string(),
                        1 => {
                            stdin.write_all(b"a\n").unwrap();
                            stdin.flush().unwrap();
                            let mut res = String::new();
                            out_reader.read_line(&mut res).unwrap();
                            res.trim().to_string()
                        }
                        2 => "Repeated".to_string(),
                        _ => "Unknown".to_string(),
                    };
                    println!("Key: {:?} | State: {}", key, state);
                }
                _ => continue,
            }
        }
    }
    child.kill().unwrap();
    child.wait().unwrap();
}
