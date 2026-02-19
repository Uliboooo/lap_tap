use rodio::{Decoder, Source};
use std::{
    fs::{self},
    io::{BufRead, BufReader, Cursor, Write},
    path::{Path, PathBuf},
    thread::sleep,
};

pub fn tap(stream_handle: &rodio::OutputStream, audio_data: &[u8]) {
    let cursor = Cursor::new(audio_data.to_owned());
    let decoder = Decoder::new(cursor).unwrap();
    let duration = decoder.total_duration().unwrap();

    stream_handle.mixer().add(decoder);
    println!("sound!");
    std::io::stdout().flush().unwrap();

    sleep(duration);
}

fn load_audio_files<P: AsRef<Path>>(p: P) -> Vec<(PathBuf, Vec<u8>)> {
    fs::read_dir(p)
        .unwrap()
        .filter_map(|f| f.ok())
        .map(|f| {
            let path = f.path();
            (fs::read(&path), path)
        })
        .filter(|f| f.0.is_ok())
        .map(|f| (f.1, f.0.unwrap()))
        .collect::<Vec<_>>()
}

fn run_audio_loop<P: AsRef<Path>>(folder: P) {
    let loaded = load_audio_files(folder);
    let stream_handle = rodio::OutputStreamBuilder::open_default_stream().unwrap();

    let reader = BufReader::new(std::io::stdin());

    for l in reader.lines() {
        match l {
            Ok(_v) => {
                let rand_index = rand::random_range(0..loaded.len());
                let a_buf = loaded.get(rand_index).unwrap();
                tap(&stream_handle, &a_buf.1);
            }
            Err(e) => {
                eprintln!("{e}");
                continue;
            }
        }
    }
}

fn main() {
    let mut args = std::env::args();
    let folder = args.nth(1).unwrap();
    run_audio_loop(folder);
}

#[cfg(test)]
mod tests {
    use crate::{load_audio_files, run_audio_loop, tap};
    use std::thread;
    use std::time::Duration;

    #[test]
    fn audio_test() {
        let c = std::env::current_dir()
            .unwrap()
            .parent()
            .unwrap()
            .join("resources")
            .join("audio-effects");
        println!("{:?}", c);
        println!("{:?}", c.exists());
        let loaded = load_audio_files(c);
        let max = loaded.len();

        let stream_handle = rodio::OutputStreamBuilder::open_default_stream().unwrap();
        let rand_index = rand::random_range(0..max);
        let a_buf = loaded.get(rand_index).unwrap();
        tap(&stream_handle, &a_buf.1);

        thread::sleep(Duration::from_millis(500));
    }

    #[test]
    fn foo1() {
        run_audio_loop("/home/coyuki/Develop/lap-tap/lap_tap/resources/audio-effects/");
    }
}
