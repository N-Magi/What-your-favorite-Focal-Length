use exif::{self, In, Tag};
use glob::glob;
use plotters::{
    self,
    prelude::{BitMapBackend, ChartBuilder, IntoDrawingArea, IntoLinspace, IntoSegmentedCoord},
    series::Histogram,
    style::{Color, RED, WHITE, BLACK},
};
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

    let plotFile = "output.png";
    let root = BitMapBackend::new(plotFile, (800, 500)).into_drawing_area();
    root.fill(&WHITE);
    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5)
        .caption("Histogram of Photo Focal-Length", ("Times New Roman", 35.0))
        .build_cartesian_2d((0u32..220u32).step(10).into_segmented(), (0u32..1750u32))
        .unwrap();
    chart
        .configure_mesh()
        .disable_x_mesh()
        .x_labels(22)
        .bold_line_style(&WHITE.mix(0.3))
        .y_desc("Count")
        .x_desc("35mm film Focal-Length[mm]")
        .axis_desc_style(("Times New Roman", 15))
        .draw()
        .unwrap();

    let pattern = args[1].clone() + "/**/*.JPG";
    let mut image_files = list_image_files(pattern);
    let pattern = args[1].clone() + "/**/*.jpg";
    let mut jpgs = list_image_files(pattern);
    image_files.append(&mut jpgs);

    let mut writer = BufWriter::new(File::create("output.csv").unwrap());

    let mut cnt = 0;
    let files_cnt = image_files.len();
    let mut data: Vec<u32> = Vec::new();
    for image_file in image_files {
        let mut length = get_exif_data(image_file).unwrap_or(0);
        if length > 0 {
            let mod10 = length % 10;
            if mod10 < 5 {
                length -= mod10;
            } else {
                length += 10 - mod10;
            }

            data.push(length);
        }
        writeln!(writer, "{},", length).unwrap();
        cnt += 1;
        print!("\r {} / {}", cnt, files_cnt);
    }

    chart
        .draw_series(
            Histogram::vertical(&chart)
                .style(BLACK.mix(0.8).filled())
                .data(data.iter().map(|x| (*x, 1))),
        )
        .unwrap();

    root.present().expect("failed");
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
