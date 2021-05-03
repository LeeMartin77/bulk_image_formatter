use std::io::{self, BufRead};
use std::env;
use image::io::Reader as ImageReader;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut i = 0;
    for line in stdin.lock().lines() {
        let lineres = line.unwrap();
        println!("{}", lineres);
        i = i + 1;
        match ImageReader::open(lineres)?.decode() {
            Ok(okimg) => {
                match okimg.save(format!("{}/target/output/{}.png", env::current_dir().unwrap().to_string_lossy(), i)) {
                    Ok(_) => (println!("Success: {}", i)),
                    Err(_) => (println!("Error Save: {}", i))
                }
            },
            Err(_) => (println!("Error read: {}", i))
        }
    }
    Ok(())
}
