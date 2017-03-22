# Rust version
Implementation of 'to' backend in Rust, using [serde](https://github.com/serde-rs/serde) for parsing the settings file.

# Compiling
Using stable rust (minimum 1.15.0) run:
 
    cargo build --release
    
# Usage
Run backend directly using:
    
    cargo run list
    
Or hook it up to the frontend by setting the target to 'rust'.
