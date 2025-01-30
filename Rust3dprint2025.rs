use std::fs;
use std::path::Path;
use std::collections::VecDeque;

/// Structure to represent a 3D print job
struct PrintJob {
    filename: String,
    file_size: u64,
}
/// Basic print queue
struct PrintQueue {
    queue: VecDeque<PrintJob>,
    prioritize_small_files: bool,  //Smaller files go first, larger files go first be able to switch it?
}
 PrintQueue
    /// Create a new queue with sorting preference
    /// Add a new job to the queue
    fn add_job(self, job: PrintJob) {
        if self small_files {
            self push_front(job); 
        } else {
            self.queue.push_back(job); 
        }
    }

/// Process the next print job
/// Function to scan a directory and categorize files
fn scan_directory(directory: &str) -> (Vec<PrintJob>, Vec<PrintJob>) {
    let mut stl_files
    let mut gcode_files
    // (stl_files, gcode_files)
}

fn main() {
    let directory = "./print_files"; // Replace with actual directory
    let prioritize_small_files = true; 


    let queue = PrintQueue:new(prioritize_small_files);
    // Add G-code files to the queue    
    // Process the print queue (simulate sending to 3D printer)

}
