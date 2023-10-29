use std::error::Error;
use std::fs::OpenOptions;
use std::io::Seek;
use std::path::Path;

use serde::{Deserialize, Serialize};

pub const CSV_FILE_PATH_NAME: &str = "/tmp/usb.csv";

pub fn save_mount_info_to_file(file_path: &str, device_partition: &String, mounted_path: &String) {
    log::debug!("Init write mount info to {}", file_path);
    let record = vec![device_partition, mounted_path];
    append_to_file(file_path, record).unwrap();
}

// TODO save headers in save_mount_info_to_file
pub fn delete_mount_info_in_file(file_path: &str, device_partition: &String) {
    log::debug!("Init delete mount info of {}", device_partition);
    if Path::new(file_path).exists() {
        let mut rdr = csv::Reader::from_path(file_path).unwrap();
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
        write_to_new_file(file_path, &new_file_content_vector).unwrap();
    } else {
        log::debug!("No file to modify. The file does not exist: {}", file_path);
    }
}

fn append_to_file(file_path: &str, record: Vec<&String>) -> Result<(), Box<dyn Error>> {
    let must_create_new_file = !Path::new(file_path).exists();
    let mut file = OpenOptions::new()
        .create(must_create_new_file)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_file_methods_runs_ok() {
        let file_path = "/tmp/test-usb.csv";
        if Path::new(file_path).exists() {
            std::fs::remove_file(file_path).unwrap();
        };
        let mut expected_file_content = "device_partition,mounted_path
/dev/foo1,/mount/foo
"
        .to_string();
        save_mount_info_to_file(
            file_path,
            &"/dev/foo1".to_string(),
            &"/mount/foo".to_string(),
        );
        let contents = std::fs::read_to_string(file_path).unwrap();
        assert_eq!(expected_file_content, contents);
        expected_file_content.push_str("/dev/bar1,/mount/bar\n");
        save_mount_info_to_file(
            file_path,
            &"/dev/bar1".to_string(),
            &"/mount/bar".to_string(),
        );
        let contents = std::fs::read_to_string(file_path).unwrap();
        assert_eq!(expected_file_content, contents);
        let expected_file_content = "device_partition,mounted_path
/dev/bar1,/mount/bar
"
        .to_string();
        delete_mount_info_in_file(file_path, &"/dev/foo1".to_string());
        let contents = std::fs::read_to_string(file_path).unwrap();
        assert_eq!(expected_file_content, contents);
        delete_mount_info_in_file(file_path, &"/dev/bar1".to_string());
        let contents = std::fs::read_to_string(file_path).unwrap();
        assert_eq!(true, contents.is_empty());
    }
}
