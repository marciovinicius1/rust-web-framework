pub mod compression {
     use std::io::{ Read, Write };
     use flate2::Compression;
     use flate2::read::GzDecoder;
     use flate2::write::GzEncoder;

     pub fn compress_string(s: &str) -> Result<Vec<u8>, std::io::Error> {
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(s.as_bytes())?;
        encoder.finish()
    }

    pub fn _decompress_string(compress_data: &[u8]) ->  Result<String, std::io::Error> {
        let mut encoder = GzDecoder::new(compress_data);
        let mut decompressed_data = String::new();
        encoder.read_to_string(&mut decompressed_data)?;
        Ok(decompressed_data)
    }
}
