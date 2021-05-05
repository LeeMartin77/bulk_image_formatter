use std::io::{self, BufRead, Write };
use image::io::Reader as ImageReader;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let lineres = line.unwrap();
        let mut parts = lineres.split(":");
        let source = parts.next().unwrap();
        let target = parts.next().unwrap();
        match ImageReader::open(&source)?.decode() {
            Ok(okimg) => save_image_to_output(okimg, format!("{}", target)),
            Err(_) => write_error_to_log(format!("Error read: {}", source))
        }
    }
    Ok(())
}

fn save_image_to_output(img: image::DynamicImage, target: String) -> () {
    match img.save(&target) {
        Ok(_) => write_path_to_stdout(target),
        Err(_) => write_error_to_log(format!("saving {}", target))
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