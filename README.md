# Hello

A PoC set of applications for prototyping the External Field Computer API connection management

## How to build and run the C++ application

```sh
cmake -B build -DCMAKE_EXPORT_COMPILE_COMMANDS=ON
cmake --build build -j8
./build/cxx/hello
```

## How to build and run the Rust application

```sh
cargo run
```
