//! # doom_random
//!
//! A fast, deterministic pseudo-random number generator based on DOOM's (1993)
//! original RNG implementation.
//!
//! This crate provides a lightweight RNG that uses a pre-defined table of 256
//! bytes and a simple index counter. It is designed for games, simulations, and
//! applications where **speed** and **determinism** are more important than
//! cryptographic security.
//!
//! ## Features
//!
//! - **Fast**: One table lookup per random byte.
//! - **Deterministic**: The same seed always produces the same sequence.
//! - **Zero dependencies**: No external crates required.
//! - **Thread-local**: Each thread gets its own independent RNG state.
//! - **Small**: The entire table is 256 bytes.
//!
//! ## Security Warning
//!
//! ⚠️ **This crate is NOT cryptographically secure.** It should never be used
//! for cryptography, password generation, or any security-critical application.
//!
//! ## Example
//!
//! ```
//! use doom_random::{random8, random32, random_seed};
//!
//! // Seed the RNG for reproducible output
//! random_seed(42);
//!
//! let a = random8();
//! let b = random32();
//!
//! println!("Random byte: {}", a);
//! println!("Random 32-bit: {}", b);
//! ```
//!
//! ## How It Works
//!
//! DOOM's RNG is a simple lookup table. Each call to [`random8`] reads the
//! current byte from the table and advances the index. When the index reaches
//! 256, it wraps around to 0.
//!
//! This design makes the RNG extremely fast (a single array access) but
//! limits its period to 256 bytes. The table itself contains intentional
//! duplicates and biases that give the RNG a "natural" feel for gameplay.
//!
//! ## License
//!
//! This crate is licensed under the GPL License. See `LICENSE` for details.
//!

use std::cell::Cell;
use std::time::UNIX_EPOCH;

const RANDOM_TABLE: [u8; 256] = [
    0, 8, 109, 220, 222, 241, 149, 107, 75, 248, 254, 140, 16, 66, 74, 21, 211, 47, 80, 242, 154,
    27, 205, 128, 161, 89, 77, 36, 95, 110, 85, 48, 212, 140, 211, 249, 22, 79, 200, 50, 28, 188,
    52, 140, 202, 120, 68, 145, 62, 70, 184, 190, 91, 197, 152, 224, 149, 104, 25, 178, 252, 182,
    202, 182, 141, 197, 4, 81, 181, 242, 145, 42, 39, 227, 156, 198, 225, 193, 219, 93, 122, 175,
    249, 0, 175, 143, 70, 239, 46, 246, 163, 53, 163, 109, 168, 135, 2, 235, 25, 92, 20, 145, 138,
    77, 69, 166, 78, 176, 173, 212, 166, 113, 94, 161, 41, 50, 239, 49, 111, 164, 70, 60, 2, 37,
    171, 75, 136, 156, 11, 56, 42, 146, 138, 229, 73, 146, 77, 61, 98, 196, 135, 106, 63, 197, 195,
    86, 96, 203, 113, 101, 170, 247, 181, 113, 80, 250, 108, 7, 255, 237, 129, 226, 79, 107, 112,
    166, 103, 241, 24, 223, 239, 120, 198, 58, 60, 82, 128, 3, 184, 66, 143, 224, 145, 224, 81,
    206, 163, 45, 63, 90, 168, 114, 59, 33, 159, 95, 28, 139, 123, 98, 125, 196, 15, 70, 194, 253,
    54, 14, 109, 226, 71, 17, 161, 93, 186, 87, 244, 138, 20, 52, 123, 251, 26, 36, 17, 46, 52,
    231, 232, 76, 31, 221, 84, 37, 216, 165, 212, 106, 197, 242, 98, 43, 39, 175, 254, 145, 190,
    84, 118, 222, 187, 136, 120, 163, 236, 249,
];

thread_local! {
    static INDEX: Cell<u8> = const { Cell::new(0) };
}

/// Generates a random 8-bit unsigned integer.
///
/// This is the core function of the crate. It reads the current byte from
/// the DOOM random table and advances the index by one, wrapping around
/// after 255.
///
/// # Returns
///
/// A random `u8` in the range `0..=255`.
///
/// # Example
///
/// ```
/// use doom_random::random8;
///
/// let byte = random8();
/// assert!(byte <= 255);
/// ```
#[inline]
pub fn random8() -> u8 {
    let current_index = INDEX.get();
    let value = RANDOM_TABLE[current_index as usize];
    INDEX.set(current_index.wrapping_add(1));
    value
}

/// Generates a random 16-bit unsigned integer.
///
/// Combines two calls to [`random8`] into a single 16-bit value
///
/// # Returns
///
/// A random `u16` in the range `0..=65_535`.
///
/// # Example
///
/// ```
/// use doom_random::random16;
///
/// let word = random16();
/// println!("{}", word);
/// ```
#[inline]
pub fn random16() -> u16 {
    (random8() as u16) << 8 | random8() as u16
}

/// Generates a random 32-bit unsigned integer.
///
/// Combines two calls to [`random16`] into a 32-bit value.
///
/// # Returns
///
/// A random `u32` in the range `0..=4_294_967_295`.
#[inline]
pub fn random32() -> u32 {
    (random16() as u32) << 16 | random16() as u32
}

/// Generates a random 64-bit unsigned integer.
///
/// Combines two calls to [`random32`] into a 64-bit value.
///
/// # Returns
///
/// A random `u64` in the range `0..=18_446_744_073_709_551_615`.
#[inline]
pub fn random64() -> u64 {
    (random32() as u64) << 32 | random32() as u64
}

/// Generates a random 128-bit unsigned integer.
///
/// Combines two calls to [`random64`] into a 128-bit value.
/// # Returns
///
/// A random `u128` in the range `0..=340_282_366_920_938_463_463_374_607_431_768_211_455`.
#[inline]
pub fn random128() -> u128 {
    (random64() as u128) << 64 | random64() as u128
}

/// Generates a random boolean value.
///
/// Uses the least significant bit of [`random8`]. Returns `true` or `false`
/// with approximately equal probability.
///
/// # Example
///
/// ```
/// use doom_random::random_bool;
///
/// if random_bool() {
///     println!("Heads!");
/// } else {
///     println!("Tails!");
/// }
/// ```
#[inline]
pub fn random_bool() -> bool {
    random8() & 1 == 0
}

/// Seeds the RNG with a specific 8-bit value.
///
/// This sets the internal table index to the given value. The next call to
/// [`random8`] will return the byte at that index.
///
/// # Arguments
///
/// * `seed` - The new index (0-255).
///
/// # Example
///
/// ```
/// use doom_random::{random_seed, random8};
///
/// random_seed(42);
/// let first = random8();
///
/// random_seed(42);
/// let second = random8();
///
/// assert_eq!(first, second); // Deterministic!
/// ```
#[inline]
pub fn random_seed(seed: u8) {
    INDEX.set(seed);
}

/// Seeds the RNG using a string (DJB2 hash).
///
/// Computes a hash of the input string and uses the lower 8 bits as the
/// seed. This is useful for generating a deterministic RNG stream from a
/// human-readable seed (e.g., a player name or level name).
///
/// If the string is empty, the index is set to 0.
///
/// # Arguments
///
/// * `seed` - The string to hash.
///
/// # Example
///
/// ```
/// use doom_random::{random_seedstr, random8};
///
/// random_seedstr("my_seed");
/// let value = random8();
/// ```
pub fn random_seedstr(seed: &str) {
    INDEX.set(seed.chars().fold(5381u32, |hash, c| {
        hash.wrapping_shl(5)
            .wrapping_add(hash)
            .wrapping_add(c as u32)
    }) as u8);
}

/// Seeds the RNG using system entropy (time-based).
///
/// Uses the current system time in nanoseconds to generate a seed. This is
/// useful for getting a different sequence every time the program runs.
///
/// # Note
///
/// This is **not** cryptographically secure. It only provides enough entropy
/// to make the RNG "feel" random for games and simulations.
///
/// # Example
///
/// ```
/// use doom_random::{random_seed_entropy, random32};
///
/// random_seed_entropy();
/// let value = random32();
/// ```
pub fn random_seed_entropy() {
    let now = std::time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();

    INDEX.set(now as u8);
}

/// Returns the current index of the RNG table.
///
/// This is primarily useful for debugging and testing. It allows you to
/// verify that the RNG is at the expected position in its sequence.
///
/// # Example
///
/// ```
/// use doom_random::{random_seed, random8, random_get_index};
///
/// random_seed(0);
/// assert_eq!(random_get_index(), 0);
/// random8();
/// assert_eq!(random_get_index(), 1);
/// ```
#[inline]
pub fn random_get_index() -> u8 {
    INDEX.get()
}

/// Reset the index of the RNG table back  to zero.
///
/// This allows you to for example reset the RNG table when the player enters a new level.
pub fn random_reset() {
    INDEX.set(0);
}
