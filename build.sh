#!/bin/bash

# Set Java home
export JAVA_HOME=$(dirname $(dirname $(readlink -f $(which javac))))

# Compile Java code
javac -d . src/main/java/com/example/App.java

# Generate JNI header
javac -h src/native src/main/java/com/example/App.java

# Build Rust shared library
cd src/native
cargo build --release
cd ../..

# Copy the shared library to a location Java can find
cp src/native/target/release/libhello.so .

# Ensure the library is in a standard path
export LD_LIBRARY_PATH=$PWD:$LD_LIBRARY_PATH