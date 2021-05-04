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
            Ok(okimg) => save_image_to_output(okimg, format!("{}", i)),
            Err(_) => write_error_to_log(format!("Error read: {}", i))
        }
    }
    Ok(())
}

fn save_image_to_output(img: image::DynamicImage, filename: String) -> () {
    let savelocation = format!("{}/target/output/{}.jpg", env::current_dir().unwrap().to_string_lossy(), filename);
    match img.save(&savelocation) {
        Ok(_) => write_path_to_stdout(savelocation),
        Err(_) => write_error_to_log(format!("saving {}", filename))
    }
}

fn write_path_to_stdout(savelocation: String) -> (){
    match io::stdout().write((savelocation + "\n").as_bytes()) {
        Ok(_) => (),
        Err(_) => ()
    }
}

fn write_error_to_log(error: String) -> (){
    println!("Error: {}", error);
}