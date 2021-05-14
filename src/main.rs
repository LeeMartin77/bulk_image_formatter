use image::GenericImageView;
use std::str::FromStr;
use std::path::Path;
use std::io::{self, BufRead, Write };
use image::io::Reader as ImageReader;
use image::imageops::FilterType;
use clap::{Arg, App};

struct ProcessingArguments {
    format: String,
    square: bool,
    resize: Option<u32>
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
                    .arg(Arg::with_name("square")
                            .short("sq")
                            .long("square")
                            .takes_value(true)
                            .help("Crop image to a square")
                            .default_value("false")
                            .possible_values(&["true", "false"]))
                    .arg(Arg::with_name("resize")
                            .short("rs")
                            .long("resize")
                            .takes_value(true)
                            .help("Set max dimension of image in pixels"))
                    .get_matches();
    
    let resize_arg: Option<u32>;
    if let Some(arg) = matches.value_of("resize") {
        if let Ok(num) = arg.to_string().parse::<u32>() {
            resize_arg = Some(num);
        } else {
            resize_arg = None;
        }
    } else {
        resize_arg = None;
    }

    let arguments = ProcessingArguments{
        format: matches.value_of("format").unwrap_or("jpg").to_string(),
        square: FromStr::from_str(matches.value_of("square").unwrap_or("false")).unwrap_or(false),
        resize: resize_arg
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
            .and_then(|img| crop_image(img, arguments))
            .and_then(|img| resize_image(img, arguments))
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

fn crop_image(img: image::DynamicImage, arguments: &ProcessingArguments) -> Result<image::DynamicImage, BulkImageFormatError> {
    if !arguments.square || img.width() == img.height() {
        return Ok(img);
    }
    if img.width() > img.height() {
        let x: u32 = (img.width() - img.height()) / 2 ;
        let y: u32 = 0;
        let width: u32 = img.height();
        let height: u32 = img.height();
        return Ok(img.crop_imm(x, y, width, height))
    }
    else
    {
        let x: u32 = 0;
        let y: u32 = (img.height() - img.width()) / 2 ;
        let width: u32 = img.width();
        let height: u32 = img.width();
        return Ok(img.crop_imm(x, y, width, height))
    }
}

fn resize_image(img: image::DynamicImage, arguments: &ProcessingArguments) -> Result<image::DynamicImage, BulkImageFormatError> {
    if arguments.resize == None {
        return Ok(img);
    }
    Ok(img.resize(arguments.resize.unwrap(), arguments.resize.unwrap(), FilterType::Gaussian))
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