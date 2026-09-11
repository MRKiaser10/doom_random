# doom_random

A fast, deterministic RNG based on DOOM's (1993) original implementation.

## Features

- ⚡ Fast: One table lookup per random byte
- 🎯 Deterministic: The same seed always produces the same sequence
- 📦 Zero dependencies
- 🧵 Thread-local state

## ⚠️ Security Warning

**This crate is NOT cryptographically secure.**

## Usage

```rust
use doom_random::{random8, random_seed};

fn main() {
    random_seed(42);
    println!("Random byte: {}", random8());
