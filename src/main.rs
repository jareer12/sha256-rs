mod sha256;

fn main() {
    let data = b"hello, world";
    let hash = sha256::sha256(data);

    for byte in hash.iter() {
        print!("{:02x}", byte);
    }
}
