extern crate indicatif;

use std::fs;
use std::path::PathBuf;
use std::time::Instant;
use std::sync::{Arc, Mutex};

use indicatif::ParallelProgressIterator;
use indicatif::ProgressIterator;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

const FILES_DIR: &str = "../../transcriptions-words";

fn main() {
    let start = Instant::now();
    let files = read_files_parallel(FILES_DIR);
    let elapsed = start.elapsed();
    println!("Loaded {} files in {:?}", files.len(), elapsed);
    println!("Average size per file: {} bytes", calc_average_size(&files));
}

fn calc_average_size(text_files: &Vec<String>) -> usize {

    let mut sum = 0;
    for text in text_files {
        sum += text.len();
    }

    sum / text_files.len()
}

fn dir_to_paths(dir_path: &str) -> Vec<PathBuf> {
    fs::read_dir(dir_path)
        .unwrap()
        .map(|file| file.unwrap().path())
        .collect()
}

fn read_files(dir_path: &str) -> Vec<String> {
    let paths = dir_to_paths(dir_path);
    let mut contents: Vec<String> = Vec::new();
    paths.iter().progress().for_each(|path| {
        let result = fs::read_to_string(path.as_path());
        match result {
            Ok(content) => contents.push(content),
            Err(e) => println!("Unable to read file {}", e),
        }
    });

    contents
}

fn read_files_parallel(dir_path: &str) -> Vec<String> {
    let paths = dir_to_paths(dir_path);
    let contents = Arc::new(Mutex::new(Vec::new()));
    paths.par_iter().progress().for_each(|path| {
        let result = fs::read_to_string(path.as_path());
        match result {
            Ok(content) => contents.lock().unwrap().push(content),
            Err(e) => println!("Unable to read file {}", e),
        }
    });

    let unlocked = contents.lock().unwrap().to_owned();
    return unlocked;
}
