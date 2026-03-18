use std::io::Read;

pub fn parse_response(data: &[u8]) -> Result<(u16, String, Vec<u8>), String> {
    let sep = data
        .windows(4)
        .position(|w| w == b"\r\n\r\n")
        .ok_or("Invalid HTTP response")?;
    let header = String::from_utf8_lossy(&data[..sep]).to_string();

    let status: u16 = header
        .lines()
        .next()
        .and_then(|l| l.split_whitespace().nth(1))
        .and_then(|s| s.parse().ok())
        .unwrap_or(200);

    let body = if header.to_lowercase().contains("transfer-encoding: chunked") {
        decode_chunked(&data[sep + 4..])
    } else {
        data[sep + 4..].to_vec()
    };

    Ok((status, header, body))
}

pub fn extract_header(headers: &str, name: &str) -> Option<String> {
    let search = format!("{}:", name);
    headers
        .lines()
        .find(|l| l.to_lowercase().starts_with(&search))
        .map(|l| l.split_once(':').map(|(_, v)| v).unwrap_or("").trim().to_string())
}

pub fn decompress(data: &[u8], encoding: &str) -> Result<Vec<u8>, String> {
    if data.is_empty() {
        return Ok(Vec::new());
    }

    let enc = encoding.to_lowercase();
    if enc.contains("gzip") {
        let mut decoder = flate2::read::GzDecoder::new(data);
        let mut out = Vec::new();
        decoder
            .read_to_end(&mut out)
            .map_err(|e| format!("gzip: {}", e))?;
        Ok(out)
    } else if enc.contains("deflate") {
        let mut decoder = flate2::read::DeflateDecoder::new(data);
        let mut out = Vec::new();
        decoder
            .read_to_end(&mut out)
            .map_err(|e| format!("deflate: {}", e))?;
        Ok(out)
    } else {
        Ok(data.to_vec())
    }
}

fn decode_chunked(data: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    let mut pos = 0;

    while pos < data.len() {
        let line_end = data[pos..].windows(2).position(|w| w == b"\r\n");
        if line_end.is_none() {
            break;
        }
        let line_end = pos + line_end.unwrap();

        let size_str = String::from_utf8_lossy(&data[pos..line_end]);
        let size = usize::from_str_radix(size_str.trim(), 16).unwrap_or(0);
        if size == 0 {
            break;
        }

        let chunk_start = line_end + 2;
        let chunk_end = chunk_start + size;
        if chunk_end > data.len() {
            break;
        }

        result.extend_from_slice(&data[chunk_start..chunk_end]);
        pos = chunk_end + 2;
    }

    result
}
