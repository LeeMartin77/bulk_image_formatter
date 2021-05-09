use std::path::Path;
use std::io::{self, BufRead, Write };
use image::io::Reader as ImageReader;
use clap::{Arg, App};

struct ProcessingArguments {
    format: String
}

struct BulkImageSourceTarget {
    source: String,
    target: String
}

enum BulkImageFormatError {
    SaveError,
    StdOutError,
    StdInError,
    ReadFailure,
    SourceError,
    TargetError
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
    if let Ok(bulksourcetarget) = stdline.map_err(|e| BulkImageFormatError::StdInError)
        .and_then(|line| turn_line_into_source_and_target(line.split(":"))) {

            let _result = format_from_source_to_target(bulksourcetarget.source.as_str())
                .and_then(|img| save_image_to_output(img, bulksourcetarget.target, arguments))
                .and_then(|saveloc| write_path_to_stdout(saveloc));

        }
    
    return ();
}

fn turn_line_into_source_and_target(mut parts: std::str::Split<&str>) -> Result<BulkImageSourceTarget, BulkImageFormatError> {
    if let Some(source) = parts.next() {
        if let Some(target) = parts.next() {
            return Ok(BulkImageSourceTarget{source: source.to_string(), target: target.to_string()});
        }
        write_error_to_log("Failed to read target".to_string());
        return Err(BulkImageFormatError::TargetError);
    }
    write_error_to_log("Failed to read source".to_string());
    return Err(BulkImageFormatError::SourceError);
}

fn format_from_source_to_target(source: &str) -> Result<image::DynamicImage, BulkImageFormatError> {
    if let Ok(imgreader) = ImageReader::open(&source) {
        if let Ok(okimg) = imgreader.decode() {
            return Ok(okimg);
        }
        
    }
    write_error_to_log(format!("Error read: {}", source));
    Err(BulkImageFormatError::ReadFailure)
}

fn save_image_to_output(img: image::DynamicImage, target: String, arguments: &ProcessingArguments) -> Result<String, BulkImageFormatError>  {
    
    let path_to_target = Path::new(&target);
    let path_to_target = path_to_target.with_extension(&arguments.format);

    if let Some(path) = path_to_target.as_path().to_str() {
        if let Ok(_) = img.save(&path) {
            return Ok(path.to_string());
        }
    }
    write_error_to_log(format!("Failed to write to save {}", target));
    Err(BulkImageFormatError::SaveError)
}

fn write_path_to_stdout(savelocation: String) -> Result<(), BulkImageFormatError> {
    if let Ok(_) = io::stdout().write((savelocation + "\n").as_bytes()) {
        return Ok(())
    }
    write_error_to_log(format!("Failed to write to stdout"));
    Err(BulkImageFormatError::StdOutError)
}

fn write_error_to_log(error: String) -> () {
    println!("Error: {}", error);
}