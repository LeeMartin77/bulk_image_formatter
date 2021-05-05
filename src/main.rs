use std::io::{self, BufRead, Write };
use image::io::Reader as ImageReader;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    for stdline in stdin.lock().lines() {
        match stdline {
            Ok(line) => {
                let mut parts = line.split(":");
                if let Some(source) = parts.next() {
                    if let Some(target) = parts.next() {
                        match ImageReader::open(&source)?.decode() {
                            Ok(okimg) => save_image_to_output(okimg, target.to_string()),
                            Err(_) => write_error_to_log(format!("Error read: {}", source))
                        }
                    }
                }
            },
            Err(_) => write_error_to_log("Failed to read stdin line".to_string())
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