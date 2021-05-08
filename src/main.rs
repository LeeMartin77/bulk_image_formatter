use std::path::Path;
use std::io::{self, BufRead, Write };
use image::io::Reader as ImageReader;
use clap::{Arg, App};

struct ProcessingArguments {
    format: String
}

fn main() -> io::Result<()> {
    let matches = App::new("BIF: Bulk Image Formatter")
                    .version("0.1.0")
                    .author("Lee Martin <lee.martin@pliminus.co.uk>")
                    .about("Formats images from stdin")
                    .arg(Arg::with_name("format")
                            .short("f")
                            .long("format")
                            .takes_value(true)
                            .help("Output image format")
                            .default_value("jpg")
                            .possible_values(&["jpg", "png", "bmp"]))
                    .get_matches();
    let arguments = ProcessingArguments{
        format: matches.value_of("format").unwrap_or("jpg").to_string()
    };
    let stdin = io::stdin();
    for stdline in stdin.lock().lines() {
        process_line(stdline, &arguments)
    }
    Ok(())
}

fn process_line(stdline: Result<std::string::String, std::io::Error>, arguments: &ProcessingArguments) -> () {
    match stdline {
        Ok(line) => turn_line_into_source_and_target(line.split(":"), arguments),
        Err(_) => write_error_to_log("Failed to read stdin line".to_string())
    }
}

fn turn_line_into_source_and_target(mut parts: std::str::Split<&str>, arguments: &ProcessingArguments) -> () {
    if let Some(source) = parts.next() {
        if let Some(target) = parts.next() {
            return format_from_source_to_target(source, target, arguments)
        }
        return write_error_to_log("Failed to read target".to_string())
    }
    return write_error_to_log("Failed to read source".to_string())
}

fn format_from_source_to_target(source: &str, target: &str, arguments: &ProcessingArguments) -> () {
    match ImageReader::open(&source) {
        Ok(imgreader) =>{
            match imgreader.decode() {
                Ok(okimg) => save_image_to_output(okimg, target.to_string(), arguments),
                Err(_) => write_error_to_log(format!("Error read: {}", source))
            }
        },
        Err(_) => write_error_to_log(format!("Error read: {}", source))
    }
}

fn save_image_to_output(img: image::DynamicImage, target: String, arguments: &ProcessingArguments) -> () {
    
    let path_to_target = Path::new(&target);
    let path_to_target = path_to_target.with_extension(&arguments.format);

    if let Some(path) = path_to_target.as_path().to_str() {
        match img.save(&path) {
            Ok(_) => write_path_to_stdout(path.to_string()),
            Err(_) => write_error_to_log(format!("saving {}", target))
        }
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