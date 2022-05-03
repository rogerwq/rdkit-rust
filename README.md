# RDKit bindings for Rust


## Binding Progress
:white_check_mark: RWMol, ROMol

:white_check_mark: SmilesParse 

:white_check_mark: MorganFingerprint 

:white_check_mark: FMCS 


## Testing

- Currently tested on Mac OS X

## Prerequisites

- Install dependency of boost: `conda install -c conda-forge boost` 

## References
- [A new way to use the RDKit from other languages](https://greglandrum.github.io/rdkit-blog/technical/2021/05/01/rdkit-cffi-part1.html)
- Test cases from [Getting Started with the RDKit in Python](https://www.rdkit.org/docs/GettingStartedInPython.html)
- Code structure follows [libcurl bindings for Rust](https://github.com/alexcrichton/curl-rust)
- Rust <-> C++ relies on [CXX — safe interop between Rust and C++](https://cxx.rs/index.html)