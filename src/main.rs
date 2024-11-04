use std::fs::{self, File};
use std::collections::HashMap;
use std::path::Path;
use std::io::{BufRead, BufReader, Write};
use std::thread;
use std::sync::mpsc;

struct TimeSeriesRow {
    index: String,
    timestamp: String,
    value: f64,
}

fn main() {
    let dir_path = "/home/josh/code/output_reformat/test/";
    let file_counts = count_files_by_base_name(dir_path);
    
    let num_threads = thread::available_parallelism().unwrap().get();
    let chunk_size = (file_counts.len() + num_threads - 1) / num_threads;
    let (tx, rx) = mpsc::channel();

    // Spawn threads and divide work
    let handles: Vec<_> = file_counts.into_iter()
        .collect::<Vec<_>>()
        .chunks(chunk_size)
        .map(|chunk| {
            let chunk = chunk.to_vec();
            let dir_path = dir_path.to_string();
            let tx = tx.clone();
            
            thread::spawn(move || {
                for (base_name, count) in chunk {
                    tx.send(format!("{}\t{}", base_name, count)).unwrap();
                    
                    if count == 1 {
                        rename_single_file(&dir_path, &base_name);
                    } else {
                        process_multiple_files(&dir_path, &base_name);
                    }
                }
            })
        })
        .collect();

    // Drop original sender so receiver knows when all threads are done
    drop(tx);

    // Print progress messages as they come in
    for msg in rx {
        println!("{}", msg);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
}

fn count_files_by_base_name(dir_path: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    if let Ok(paths) = fs::read_dir(dir_path) {
        for entry in paths.filter_map(Result::ok) {
            if let Ok(filename) = entry.file_name().into_string() {
                if let Some(base_name) = filename.split("_rank").next() {
                    *counts.entry(base_name.to_string()).or_insert(0) += 1;
                }
            }
        }
    }
    counts
}

fn process_multiple_files(dir_path: &str, base_name: &str) {
    // Get all files for this nexus
    let mut files = Vec::new();
    if let Ok(paths) = fs::read_dir(dir_path) {
        for entry in paths.filter_map(Result::ok) {
            if let Ok(filename) = entry.file_name().into_string() {
                if filename.starts_with(base_name) {
                    files.push(entry.path());
                }
            }
        }
    }

    if files.is_empty() {
        return;
    }

    // Read and sum the files
    let mut summed_data = Vec::new();
    let mut first_file = true;

    for file_path in &files {
        if let Ok(file) = File::open(file_path) {
            let reader = BufReader::new(file);
            for (i, line) in reader.lines().enumerate() {
                if let Ok(line) = line {
                    let parts: Vec<&str> = line.split(',').map(str::trim).collect();
                    if parts.len() != 3 {
                        continue;
                    }

                    let value: f64 = match parts[2].parse() {
                        Ok(val) => val,
                        Err(_) => continue,
                    };

                    if first_file {
                        summed_data.push(TimeSeriesRow {
                            index: parts[0].to_string(),
                            timestamp: parts[1].to_string(),
                            value,
                        });
                    } else if i < summed_data.len() {
                        summed_data[i].value += value;
                    }
                }
            }
            first_file = false;
        }
    }

    // Write summed output
    if !summed_data.is_empty() {
        let output_path = Path::new(dir_path).join(format!("{}.csv", base_name));
        if let Ok(output) = File::create(&output_path) {
            let mut writer = std::io::BufWriter::new(output);
            for row in &summed_data {
                let _ = writeln!(writer, "{}, {}, {}", row.index, row.timestamp, row.value);
            }
        }
    }

    // Remove input files
    for file_path in files {
        let _ = fs::remove_file(file_path);
    }
}

fn rename_single_file(dir_path: &str, base_name: &str) {
    if let Ok(paths) = fs::read_dir(dir_path) {
        for entry in paths.filter_map(Result::ok) {
            if let Ok(filename) = entry.file_name().into_string() {
                if filename.starts_with(base_name) {
                    let new_path = Path::new(dir_path).join(format!("{}.csv", base_name));
                    let _ = fs::rename(entry.path(), new_path);
                    break;
                }
            }
        }
    }
}