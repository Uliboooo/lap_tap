use std::{
    io::{BufRead, BufReader, Read, Write},
    path::PathBuf,
    process::Stdio,
    sync::{Arc, atomic::AtomicBool},
};

use evdev::EventSummary;

fn main() {
    let device_path = "/dev/input/event4";
    let mut device = evdev::Device::open(device_path).unwrap();
    println!("Watching device: {}", device.name().unwrap_or("Unknown"));

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, std::sync::atomic::Ordering::SeqCst);
    })
    .unwrap();

    let c = PathBuf::from("/home")
        .join("coyuki")
        .join("Develop")
        .join("lap-tap")
        .join("lap_tap")
        .join("target")
        .join("release")
        .join("lap_tap_audio")
        .to_string_lossy()
        .to_string();

    let c2 = PathBuf::from("/home")
        .join("coyuki")
        .join("Develop")
        .join("lap-tap")
        .join("lap_tap")
        .join("resources")
        .join("audio-effects")
        .to_string_lossy()
        .to_string();

    let mut child = std::process::Command::new("sudo")
        // use 1000 for using audio server
        .args([
            "-u",
            "#1000",
            "env",
            "XDG_RUNTIME_DIR=/run/user/1000",
            &c,
            &c2,
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
