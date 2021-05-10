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
    ReadFailure,
    SourceError,
    TargetError,
    LineError
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
        if let Ok(line) = stdline {
            if let Ok(saveloc) = process_line(line, &arguments) {
                let _ = io::stdout().write((saveloc + "\n").as_bytes());
            }
        }
    }
    Ok(())
}

fn process_line(line: String, arguments: &ProcessingArguments) -> Result<String, BulkImageFormatError> {
    if let Ok(bulksourcetarget) = turn_line_into_source_and_target(line.split(":")) {
        return format_from_source_to_target(bulksourcetarget.source.as_str())
            .and_then(|img| save_image_to_output(img, bulksourcetarget.target, arguments))
    }
    Err(BulkImageFormatError::LineError)
}

fn turn_line_into_source_and_target(mut parts: std::str::Split<&str>) -> Result<BulkImageSourceTarget, BulkImageFormatError> {
    if let Some(source) = parts.next() {
        if let Some(target) = parts.next() {
            return Ok(BulkImageSourceTarget{source: source.to_string(), target: target.to_string()});
        }
        return Err(BulkImageFormatError::TargetError);
    }
    return Err(BulkImageFormatError::SourceError);
}

fn format_from_source_to_target(source: &str) -> Result<image::DynamicImage, BulkImageFormatError> {
    if let Ok(imgreader) = ImageReader::open(&source) {
        if let Ok(okimg) = imgreader.decode() {
            return Ok(okimg);
        }
        
    }
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
    Err(BulkImageFormatError::SaveError)
}