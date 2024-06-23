# Genum

Genum is a Rust program that allows you to generate various types of random data. You can install it on your system and use it through the command line.

## Features

- Generate random integers, floating-point numbers.
- Customize the range.

## Installation

To install Genum, you'll need to have Rust installed on your system. If you don't have Rust installed, you can get it from [rust-lang.org](https://www.rust-lang.org/tools/install).

## Building from Source

If you prefer to build Genum from source, follow these steps:

1. Clone the repository:

```sh
git clone https://github.com/srvshg/genum.git
```

2. Change to the root directory of the project:

```sh
cd genum
```

3. Build the project to create a binary:

```sh
cargo build --release
```

4. Run the application:
```sh
./target/release/genum
```

## Usage

After installing or building Genum, you can use it from the command line. Here are some examples:

Generate a random integer

```sh
genum int
```

Generate a random floating-point number

```sh
genum float
```

Generate a random string
```sh
genum string
```

Customize the range of generated integers
```sh
genum int --min 10 --max 100
```

Customize the length of generated strings
```sh
genum string --length 16
```




















