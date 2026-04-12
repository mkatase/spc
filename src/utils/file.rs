// src/utils/file.rs

use eframe::egui;
use std::fs;
use std::io::{BufRead, BufReader};

use crate::error::AppError;

pub fn save_image(image: &egui::ColorImage, path: &str) {
    if let Ok(_) = image::save_buffer(
        path,
        image.as_raw(),
        image.width() as u32,
        image.height() as u32,
        image::ColorType::Rgba8,
    ) {
        println!("Saved screenshot to: {}", path);
    }
}

pub fn read_file(name: String) -> Result<Vec<Vec<String>>, AppError> {
    let mut v: Vec<Vec<String>> = Vec::new();
    println!("--- read file : {}", name);

    let file = fs::File::open(name)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line    = line?;

        let processed_line = line.split('#').next().unwrap_or("");
        let trimmed = processed_line.trim();

        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        let words: Vec<String> = trimmed.split(',')
            .map(|w| w.trim().to_string())
            .filter(|w| !w.is_empty())
            .collect();

        if !words.is_empty() {
            v.push(words);
        }
    }
    Ok(v)
}
