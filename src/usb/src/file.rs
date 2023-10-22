use std::error::Error;
use std::fs::OpenOptions;

use serde::{Deserialize, Serialize};

pub fn save_mount_info_to_file(device_partition: &String, mounted_path: &String) {
    let csv_file_path_name = "/tmp/usb.csv".to_string();
    log::debug!("Init write mount info to {}", csv_file_path_name);
    let record = vec![device_partition, mounted_path];
    append_to_file(&csv_file_path_name, record).unwrap();
}


// TODO save headers in save_mount_info_to_file
pub fn delete_mount_info_in_file(device_partition: &String) {
    let csv_file_path_name = "/tmp/usb.csv".to_string(); // TODO duplicated
    log::debug!("Init delete mount info of {}", device_partition);
    let mut rdr = csv::Reader::from_path(&csv_file_path_name).unwrap();
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
    write_to_new_file(&csv_file_path_name, &new_file_content_vector).unwrap();
    println!("{:?}", new_file_content_vector);
}

fn append_to_file(file_path: &String, record: Vec<&String>) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .write(true)
        .open(file_path)
        .unwrap();
    let mut wtr = csv::Writer::from_writer(file);
    wtr.write_record(&record)?;
    wtr.flush()?;
    Ok(())
}

pub fn write_to_new_file(file_path: &String, records: &Vec<MountInfo>) -> Result<(), Box<dyn Error>> {
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
