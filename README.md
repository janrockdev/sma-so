# Shared Memory Access between Java and Rust

Type: .so library

## Rust
```bash
# src/native
cargo clean
cargo build --release

sudo cp src/native/target/release/libhello.so /usr/lib
```

## Java (JDK21)
```bash
#brew install gradle
gradle clean build run

# > Task :run
# Result from Rust: Rust says: Hello from Java! (modified!)

# BUILD SUCCESSFUL in 1s
# 7 actionable tasks: 7 executed
```





