use std::collections::HashMap;
use std::io::Write;
use zip::write::FileOptions;

pub fn make_zip(files: Vec<(String, Vec<u8>)>) -> Result<Vec<u8>, String> {
    let mut buf = Vec::new();
    {
        let mut zip = zip::ZipWriter::new(std::io::Cursor::new(&mut buf));
        let options: FileOptions<()> =
            FileOptions::default().compression_method(zip::CompressionMethod::Stored);

        let mut seen: HashMap<String, usize> = HashMap::new();

        for (name, data) in files {
            // avoid paths; keep the visible name
            let clean = name.replace('\\', "_").replace('/', "_");

            let entry = seen.entry(clean.clone()).or_insert(0);
            let final_name = if *entry == 0 {
                clean.clone()
            } else {
                // append (n) before the extension
                let (base, ext) = clean
                    .rsplit_once('.')
                    .map(|(b, e)| (b.to_string(), format!(".{e}")))
                    .unwrap_or((clean.clone(), String::new()));
                format!("{base}_({}){ext}", *entry) // no extra space
            };
            *entry += 1;

            println!("Adding {} ({} bytes)", final_name, data.len());
            zip.start_file(&final_name, options)
                .map_err(|e| e.to_string())?;
            zip.write_all(&data).map_err(|e| e.to_string())?;
        }

        zip.finish().map_err(|e| e.to_string())?;
    }
    Ok(buf)
}
