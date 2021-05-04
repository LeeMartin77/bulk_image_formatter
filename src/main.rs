use std::io::{self, BufRead, Write };
use std::env;
use image::io::Reader as ImageReader;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut i = 0;
    for line in stdin.lock().lines() {
        let lineres = line.unwrap();
        i = i + 1;
        match ImageReader::open(lineres)?.decode() {
            Ok(okimg) => {
                let savelocation = format!("{}/target/output/{}.jpg", env::current_dir().unwrap().to_string_lossy(), i);
                match okimg.save(&savelocation) {
                    Ok(_) => {
                        match io::stdout().write((savelocation + "\n").as_bytes()) {
                            Ok(_) => (),
                            Err(_) => ()
                        }
                    },
                    Err(_) => (println!("Error Save: {}", i))
                }
            },
            Err(_) => (println!("Error read: {}", i))
        }
    }
    Ok(())
}
