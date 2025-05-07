package com.example;

import com.sun.jna.Library;
import com.sun.jna.Native;
import com.sun.jna.Pointer;
import com.sun.jna.platform.unix.LibC;

public class App {
    static {
        System.loadLibrary("hello"); // Loads libhello.so
    }

    // Native method to share memory with Rust
    private static native String shareMemory(String input);

    public static void main(String[] args) {
        String input = "Hello from Java!";
        String result = shareMemory(input);
        System.out.println("Result from Rust: " + result);
    }
}