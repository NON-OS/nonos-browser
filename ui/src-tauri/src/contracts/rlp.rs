pub fn encode_uint(value: u128) -> Vec<u8> {
    if value == 0 {
        return vec![];
    }
    let bytes = value.to_be_bytes();
    let start = bytes.iter().position(|&b| b != 0).unwrap_or(bytes.len());
    bytes[start..].to_vec()
}

pub fn encode_list(items: &[Vec<u8>]) -> Vec<u8> {
    let mut payload = Vec::new();
    for item in items {
        if item.is_empty() {
            payload.push(0x80);
        } else if item.len() == 1 && item[0] < 0x80 {
            payload.push(item[0]);
        } else if item.len() < 56 {
            payload.push(0x80 + item.len() as u8);
            payload.extend(item);
        } else {
            let len_bytes = item.len().to_be_bytes();
            let start = len_bytes
                .iter()
                .position(|&b| b != 0)
                .unwrap_or(len_bytes.len());
            let len_bytes = &len_bytes[start..];
            payload.push(0xb7 + len_bytes.len() as u8);
            payload.extend(len_bytes);
            payload.extend(item);
        }
    }

    let mut result = Vec::new();
    if payload.len() < 56 {
        result.push(0xc0 + payload.len() as u8);
    } else {
        let len_bytes = payload.len().to_be_bytes();
        let start = len_bytes
            .iter()
            .position(|&b| b != 0)
            .unwrap_or(len_bytes.len());
        let len_bytes = &len_bytes[start..];
        result.push(0xf7 + len_bytes.len() as u8);
        result.extend(len_bytes);
    }
    result.extend(payload);
    result
}
