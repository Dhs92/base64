use lazy_static::lazy_static;
use hex::decode;

lazy_static! {
    static ref BASE64CHARS: Vec<char> =
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
            .chars()
            .collect();
}

// TODO take user input
fn main() {
    //println!("{:#?}", *BASE64CHARS);
    let mut inp = match decode("1c0111001f010100061a024b53535009181c") {
        Ok(s) => s,
        Err(_) => Vec::from("hello world!".as_bytes()), 
    };
    println!("{}", encode_base64(&mut inp));
}

fn encode_base64(input: &mut [u8]) -> String {
    let mut input = input.iter().copied().collect::<Vec<u8>>();
    let mut encoded_str = String::new();
    let mut pad_string = String::new();
    let mut pad_count = input.len() % 3;

    if pad_count > 0 {
        while pad_count < 3 {
            pad_string.push('=');
            input.push(0);

            pad_count += 1;
        }
    }

    for chunk in input.chunks_exact(3) {
        let res: u32 = ((chunk[0] as u32) << 16) + ((chunk[1] as u32) << 8) + (chunk[2] as u32);
        let res: [u32; 4] = [
            (res >> 18) & 63,
            (res >> 12) & 63,
            (res >> 6) & 63,
            res & 63,
        ];

        for i in &res {
            encoded_str.push_str(&BASE64CHARS[*i as usize].to_string());
        }
    }

    let mut result_str = encoded_str[..encoded_str.len() - pad_string.len()].to_string();

    result_str.push_str(&pad_string);

    result_str
}
