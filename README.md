# Sha256 Rust

The Sha256 struct represents a SHA-256 hasher and contains the current hash value, the length of the input data processed so far, a block of data to be hashed, and the length of the data in the block.

- The `new` method creates a new `Sha256` struct with the initial hash values and an empty block.
- The update method processes a block of data and updates the hash value. It also updates the length of the input data processed so far.
- The finalize method processes the final block of data, pads it if necessary, and returns the final hash value as a slice of bytes.
- The sha256 function creates a new `Sha256` struct, processes the input data using the update method, and returns the final hash value using the finalize method.
- This implementation provides a basic understanding of how the SHA-256 hash function works and can be used to compute the hash of a piece of data in Rust. However, it is not intended for use in production environments as there are faster and more secure implementations of the SHA-256 hash function available.


```rust
mod sha256;

fn main() {
    let data = b"hello, world";
    let hash = sha256::sha256(data);

    for byte in hash.iter() {
        print!("{:02x}", byte);
    }
}
```