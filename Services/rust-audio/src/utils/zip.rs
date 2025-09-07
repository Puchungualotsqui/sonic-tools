use std::io::Write;
use zip::write::FileOptions;

pub fn make_zip(files: Vec<(String, Vec<u8>)>) -> Result<Vec<u8>, String> {
    println!("make zip called");
    let mut buf = Vec::new();
    {
        let mut zip = zip::ZipWriter::new(std::io::Cursor::new(&mut buf));
        let options: FileOptions<()> =
            FileOptions::default().compression_method(zip::CompressionMethod::Stored);

        println!("loop started");
        for (name, data) in files {
            zip.start_file(name, options).map_err(|e| e.to_string())?;
            zip.write_all(&data).map_err(|e| e.to_string())?;
        }

        zip.finish().map_err(|e| e.to_string())?;
    }
    Ok(buf)
}
