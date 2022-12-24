use exif::{self, In, Tag};
use glob::glob;
use std::{
    env,
    fs::File,
    io::{BufReader, BufWriter, Write},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("引数がありません");
        return;
    }

    let pattern = args[1].clone() + "/**/*.JPG";
    let mut image_files = list_image_files(pattern);
    let pattern = args[1].clone() + "/**/*.jpg";
    let mut jpgs = list_image_files(pattern);
    image_files.append(&mut jpgs);

    let mut writer = BufWriter::new(File::create("output.csv").unwrap());

    let mut cnt = 0;
    let files_cnt = image_files.len();
    for image_file in image_files {
        let length = get_exif_data(image_file).unwrap_or(0);

        writeln!(writer, "{},", length).unwrap();
        cnt += 1;
        print!("\r {} / {}", cnt, files_cnt);
    }
}

fn list_image_files(search_pattern: String) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    println!("{}", search_pattern);
    for entry in glob(search_pattern.as_str()).unwrap() {
        if let Ok(file) = entry {
            let file_path = file.to_str().expect("filed to conver to &str").to_string();
            result.push(file_path);
            continue;
        }
        continue;
    }
    result
}

fn get_exif_data(file_path: String) -> Result<u32, ()> {
    let file = File::open(file_path).unwrap();
    let mut bufreader = BufReader::new(&file);
    let exifreader = exif::Reader::new();
    let exif = exifreader.read_from_container(&mut bufreader);
    if exif.is_err() {
        return Err(());
    };
    let exif = exif.unwrap();

    if let Some(length35) = exif.get_field(Tag::FocalLengthIn35mmFilm, In::PRIMARY) {
        let val = length35.value.get_uint(0).unwrap();
        return Ok(val);
    }
    ();
    return Err(());
}
