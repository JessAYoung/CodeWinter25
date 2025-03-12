use std::fs;
use std::collections::VecDeque;
use std::path::Path;
use std::time::Duration;
use std::thread;
use std::io::{self, BufRead};
use std::fs::File;

/// Structure to represent a 3D print job
struct PrintJob {
    filename: String,
    file_size: u64,
    estimated_time: Option<Duration>,
}

/// Print queue with size-based prioritization
struct PrintQueue {
    queue: VecDeque<PrintJob>,
    prioritize_small_files: bool, 
}

impl PrintQueue {
    /// Create a new print queue
    fn new(prioritize_small_files: bool) -> Self {
        Self {
            queue: VecDeque::new(),
            prioritize_small_files,
        }
    }

    /// Add a new job to the queue
    fn add_job(&mut self, job: PrintJob) {
        if self.prioritize_small_files {
            self.queue.push_front(job); // Smaller files first
        } else {
            self.queue.push_back(job); // Larger files first
        }
    }

    /// Process the next print job
    fn process_next_job(&mut self) {
        if let Some(job) = self.queue.pop_front() {
            println!(
                "Processing print job: {} (Size: {} bytes, Estimated time: {:?})",
                job.filename,
                job.file_size,
                job.estimated_time.unwrap_or(Duration::new(0, 0))
            );

            // Simulating sending a notification (replace with actual email/push logic)
            send_notification(&job.filename);
        } else {
            println!("No more jobs in the queue.");
        }
    }
}

/// Function to scan a directory and categorize files
fn scan_directory(directory: &str) -> (Vec<PrintJob>, Vec<PrintJob>) {
    let mut stl_files = Vec::new();
    let mut gcode_files = Vec::new();

    if let Ok(entries) = fs::read_dir(directory) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(extension) = path.extension() {
                let file_size = entry.metadata().map(|m| m.len()).unwrap_or(0);
                let estimated_time = if extension == "gcode" {
                    extract_print_time(&path)
                } else {
                    None
                };

                let job = PrintJob {
                    filename: path.file_name().unwrap().to_string_lossy().to_string(),
                    file_size,
                    estimated_time,
                };

                if extension == "stl" {
                    stl_files.push(job);
                } else if extension == "gcode" {
                    gcode_files.push(job);
                }
            }
        }
    }
    (stl_files, gcode_files)
}
fn main() {
    let directory = "./print_files"; 
    let prioritize_small_files = true; 

    println!("Scanning directory: {}", directory);

    let (_stl_files, mut gcode_files) = scan_directory(directory);
    
    println!("Found {} G-code files.", gcode_files.len());

    let mut queue = PrintQueue::new(prioritize_small_files);

    // Sort and add jobs to queue
    gcode_files.sort_by_key(|job| job.file_size);
    for job in &gcode_files {
        println!("Adding job to queue: {} ({} bytes)", job.filename, job.file_size);
        queue.add_job(job.clone());
    }

    // Process one job
    println!("Processing the next print job...");
    queue.process_next_job();
}

/// Function to extract estimated print time from a G-code file
fn extract_print_time(path: &Path) -> Option<Duration> {
    if let Ok(file) = File::open(path) {
        let reader = io::BufReader::new(file);
        for line in reader.lines().flatten() {
            if line.contains("TIME:") {
                if let Some(time_str) = line.split(':').nth(1) {
                    if let Ok(seconds) = time_str.trim().parse::<u64>() {
                        return Some(Duration::from_secs(seconds));
                    }
                }
            }
        }
    }
    None
}

/// Function to simulate sending a notification (email or push)
fn send_notification(job_name: &str) {
    println!("Notification sent: Print job '{}' completed!", job_name);
}

fn main() {
    let directory = "./print_files"; 
    let prioritize_small_files = true; 

    println!("Scanning directory: {}", directory);
    let (_stl_files, mut gcode_files) = scan_directory(directory);

    let mut queue = PrintQueue::new(prioritize_small_files);

    // Sort and add jobs to queue
    gcode_files.sort_by_key(|job| job.file_size); 
    for job in gcode_files {
        queue.add_job(job);
    }

    // Process one job
    queue.process_next_job();
}
 