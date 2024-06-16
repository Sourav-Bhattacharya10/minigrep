* To run the application in Linux with case sensitivity:
```
export IGNORE_CASE=false
cargo run -- rUsT Rust.txt // no output
cargo run -- duct Rust.txt // one output
```

* To run the application in Linux with case insensitivity:
```
export IGNORE_CASE=true
cargo run -- rUsT Rust.txt // one output
cargo run -- duct Rust.txt // one output
```