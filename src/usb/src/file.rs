use std::error::Error;
use std::fs::OpenOptions;
use std::io::Seek;

use serde::{Deserialize, Serialize};

const CSV_FILE_PATH_NAME: &str = "/tmp/usb.csv";

pub fn save_mount_info_to_file(device_partition: &String, mounted_path: &String) {
    log::debug!("Init write mount info to {}", CSV_FILE_PATH_NAME);
    let record = vec![device_partition, mounted_path];
    append_to_file(CSV_FILE_PATH_NAME, record).unwrap();
}


// TODO save headers in save_mount_info_to_file
pub fn delete_mount_info_in_file(device_partition: &String) {
    log::debug!("Init delete mount info of {}", device_partition);
    let mut rdr = csv::Reader::from_path(CSV_FILE_PATH_NAME).unwrap();
    let mut file_content_vector: Vec<MountInfo> = Vec::new();
    for result in rdr.records() {
        let record = result.unwrap();
        let mount_info = MountInfo::new(&record[0], &record[1]);
        file_content_vector.push(mount_info);
    }
    let new_file_content_vector: Vec<MountInfo> = file_content_vector
        .into_iter()
        .filter(|mount_info| &mount_info.device_partition != device_partition)
        .collect();
    write_to_new_file(CSV_FILE_PATH_NAME, &new_file_content_vector).unwrap();
    println!("{:?}", new_file_content_vector);
}

fn append_to_file(file_path: &str, record: Vec<&String>) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .write(true)
        .open(file_path)
        .unwrap();
    // https://stackoverflow.com/questions/76688593/header-appending-for-every-record-rust-csv-writer
    let needs_headers = file.seek(std::io::SeekFrom::End(0))? == 0;
    let mut wtr = csv::WriterBuilder::new()
        .has_headers(needs_headers)
        .from_writer(file);
    wtr.serialize(&MountInfo::new(&record[0], &record[1]))?;
    wtr.flush()?;
    Ok(())
}

pub fn write_to_new_file(file_path: &str, records: &Vec<MountInfo>) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path(file_path)?;
    for record in records {
        wtr.serialize(record)?;
    }
    wtr.flush()?;
    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MountInfo {
    device_partition: String,
    mounted_path: String,
}

impl MountInfo {
    fn new(device_partition: &str, mounted_path: &str) -> Self {
        MountInfo {
            device_partition: device_partition.to_string(),
            mounted_path: mounted_path.to_string(),
        }
    }
}
