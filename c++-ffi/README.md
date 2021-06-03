# c++-ffi: Providing a C function to C++ code from Rust or C.

This directory contains

* A C++ driver `demo.cc`

* A function called by the driver and written in C in `add1.c`

* A function called by the driver and written in Rust in
  `add1.rs`

* A `Makefile` that makes the whole thing compile

This has only been tested on Debian Linux with `clang` /
`clang++`: other platforms and compilers might work. Who
knows?

The `demo-rs` executable is quite large: 4MB before stripping
debug symbols, 1MB after. The executable brings in big
pieces of `std` for various reasons. The good news is that
this scales reasonably — adding a bunch of extra Rust isn't
going to change it substantially.

The build is controlled by `make`. One could add a fancy
`build.rs` and build it all with Cargo; doesn't seem worth
it here.
