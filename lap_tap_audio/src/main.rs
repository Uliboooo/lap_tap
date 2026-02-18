use rodio::{Decoder, Source};
use std::{
    fs::{self, File},
    io::{BufRead, BufReader, Cursor},
    path::{Path, PathBuf},
    thread,
    time::Duration,
};

fn get_dur<P: AsRef<Path>>(path: P) -> u128 {
    let file = File::open(path).unwrap();
    let deoder = Decoder::new(file).unwrap();
    deoder.total_duration().unwrap().as_millis()
}

pub fn tap(audio_data: &[u8], audio_duration_mills: u64) {
    let stream_handle = rodio::OutputStreamBuilder::open_default_stream().unwrap();

    let s = Decoder::try_from(Cursor::new(audio_data.to_owned().clone())).unwrap();
    stream_handle.mixer().add(s);
    println!("sound!");

    thread::sleep(Duration::from_millis(audio_duration_mills));
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

fn main() {
    let mut args = std::env::args();
    let folder = args.nth(1).unwrap();
    let loaded = load_audio_files(folder);
    let max = loaded.len();

    let reader = BufReader::new(std::io::stdin());

    for l in reader.lines() {
        match l {
            Ok(_v) => {
                let rand_index = rand::random_range(0..max);
                let a_buf = loaded.get(rand_index).unwrap();
                let dur = get_dur(&a_buf.0);
                tap(&a_buf.1, dur as u64);
            }
            Err(_) => continue,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{get_dur, load_audio_files, tap};

    #[test]
    fn test() {
        let c = std::env::current_dir()
            .unwrap()
            .parent()
            .unwrap()
            .join("resources")
            .join("audio-effects");
        println!("{:?}", c);
        println!("{:?}", c.exists());
        let loaded = load_audio_files(c);
        // println!("{:?}", loaded);
        let max = loaded.len();

        let rand_index = rand::random_range(0..max);
        let a_buf = loaded.get(rand_index).unwrap();
        let dur = get_dur(&a_buf.0);
        tap(&a_buf.1, dur as u64);
    }
}
