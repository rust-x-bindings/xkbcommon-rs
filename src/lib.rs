#![no_std]

#[cfg(feature = "std")]
extern crate std;

extern crate alloc;

extern crate libc;
#[cfg(all(feature = "wayland", feature = "std"))]
extern crate memmap2;

pub mod xkb;
