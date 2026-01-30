//! This module is responsible for saving image data to the disc. It does so by
//! running the file saving on a different thread. As this thread will be mostly i/o bound
//! it should not interfere with the rayon library

use crate::{PICTURE_HEIGHT, PICTURE_WIDTH};
use png::Encoder;
use std::fs::File;
use std::io::BufWriter;
use std::sync::mpsc::{Receiver, SyncSender, sync_channel};

/// The amount of images we buffer we can save. This is limited to avoid an overrun of data.
const BUFFER_SIZE: usize = 1024;

/// The different commands, that can be sent to the worker thread.
enum Command {
    /// Save the new image consisting of data and the indicated serial number.
    SaveImage { data: Vec<u8>, serial_number: u32 },
    /// In this case no more data is coming and we can terminate the thread.
    Terminate,
}

/// This struct is mainly an entry point to a thread and a communication channel.
pub struct ImageSaver {
    thread_handle: Option<std::thread::JoinHandle<()>>,
    sender: SyncSender<Command>,
}

/// The default spawns a thread and creates the communication channel.
impl Default for ImageSaver {
    fn default() -> Self {
        let (sender, receiver) = sync_channel::<Command>(BUFFER_SIZE);
        Self {
            thread_handle: Some(std::thread::spawn(move || handle_work(receiver))),
            sender,
        }
    }
}

/// In the drop we sent a termination command and wait for termination.
impl Drop for ImageSaver {
    fn drop(&mut self) {
        self.sender
            .send(Command::Terminate)
            .expect("Failed to send terminate command");
        self.thread_handle.take().unwrap().join().unwrap();
    }
}

impl ImageSaver {
    /// Schedules the image saving command to the thread.
    pub fn save_image(&self, color_vec: Vec<u8>, serial_number: u32) {
        self.sender
            .send(Command::SaveImage {
                data: color_vec,
                serial_number,
            })
            .expect("Failed to send file save data.");
    }
}

/// The real worker, that read in the commands and saves the png files.
fn handle_work(receiver: Receiver<Command>) {
    loop {
        let command = receiver.recv().unwrap();
        match command {
            Command::SaveImage {
                data,
                serial_number,
            } => {
                let path = format!("Image_{:06}.png", serial_number);
                let file = File::create(path).expect("Failed to create image file");
                let writer = BufWriter::new(file);

                let mut encoder = Encoder::new(writer, PICTURE_WIDTH as u32, PICTURE_HEIGHT as u32);
                encoder.set_color(png::ColorType::Rgb);
                encoder.set_depth(png::BitDepth::Eight);

                let mut writer = encoder
                    .write_header()
                    .expect("Failed to write image header");
                writer
                    .write_image_data(&data)
                    .expect("Failed to write image data");
            }
            Command::Terminate => {
                break;
            }
        }
    }
}
