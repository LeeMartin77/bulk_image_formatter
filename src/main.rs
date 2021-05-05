use std::io::{self, BufRead, Write };
use image::io::Reader as ImageReader;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    for stdline in stdin.lock().lines() {
        process_line(stdline)
    }
    Ok(())
}

fn process_line(stdline: Result<std::string::String, std::io::Error>) -> () {
    match stdline {
        Ok(line) => turn_line_into_source_and_target(line.split(":")),
        Err(_) => write_error_to_log("Failed to read stdin line".to_string())
    }
}

fn turn_line_into_source_and_target(mut parts: std::str::Split<&str>) -> () {
    if let Some(source) = parts.next() {
        if let Some(target) = parts.next() {
            return format_from_source_to_target(source, target)
        }
        return write_error_to_log("Failed to read target".to_string())
    }
    return write_error_to_log("Failed to read source".to_string())
}

fn format_from_source_to_target(source: &str, target: &str) -> () {
    match ImageReader::open(&source) {
        Ok(imgreader) =>{
            match imgreader.decode() {
                Ok(okimg) => save_image_to_output(okimg, target.to_string()),
                Err(_) => write_error_to_log(format!("Error read: {}", source))
            }
        },
        Err(_) => write_error_to_log(format!("Error read: {}", source))
    }
}

fn save_image_to_output(img: image::DynamicImage, target: String) -> () {
    match img.save(&target) {
        Ok(_) => write_path_to_stdout(target),
        Err(_) => write_error_to_log(format!("saving {}", target))
    }
}

fn write_path_to_stdout(savelocation: String) -> () {
    match io::stdout().write((savelocation + "\n").as_bytes()) {
        Ok(_) => (),
        Err(_) => write_error_to_log(format!("Failed to write to stdout"))
    }
}

fn write_error_to_log(error: String) -> () {
    println!("Error: {}", error);
}