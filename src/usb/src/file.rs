use std::error::Error;
use std::fs::OpenOptions;

pub fn write_to_file(file_path: &String, record: Vec<&String>) -> Result<(), Box<dyn Error>> {
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
