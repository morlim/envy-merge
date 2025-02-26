# Envy Merge

`envy-merge` is a CLI tool to intelligently merge multiple `.env` files while ensuring that a priority file (if provided) takes precedence over all others.

## Features

- **Merges multiple `.env` files** into a single output.
- **Respects a priority file** (if provided), ensuring its variables are never overwritten.
- **Later files overwrite earlier ones** unless the variable exists in the priority file.
- **Supports output redirection** to a file or prints to stdout.
- **Dry-run mode** to preview merged output without saving.

## Installation

Ensure you have Rust installed. If not, install it via [Rustup](https://rustup.rs/).

Clone the repository and navigate to the project folder:

```sh
git clone https://github.com/morlim/envy-merge.git
cd envy-merge
```

Then, build the project:

```sh
cargo build --release
```

## Usage

Run the CLI tool with multiple `.env` files:

```sh
./target/release/envy-merge file1.env file2.env
```

### Options

```sh
USAGE:
    envy-merge [OPTIONS] <FILES>...

ARGS:
    <FILES>...    List of .env files to merge (at least 2 required)

OPTIONS:
    -p, --priority <FILE>    File to take priority in case of conflicts
    -o, --output <FILE>      Output .env file (default: stdout)
    -d, --dry-run            Show merged output without writing to a file
    -h, --help               Show help message
    -V, --version            Show version
```

### Example

#### Input Files

##### `priority.env`

```text
DB_HOST=secure-db
DB_PORT=5432
API_KEY=supersecret
```

##### `file1.env`

```text
DB_HOST=localhost
DB_PORT=3306
DEBUG=false
```

##### `file2.env`

```text
API_KEY=publickey
LOG_LEVEL=info
```

#### Merging with Priority File

```sh
./target/release/envy-merge -p priority.env file1.env file2.env
```

#### Output

```text
DB_HOST=secure-db  # Taken from priority file (CANNOT be overwritten)
DB_PORT=5432       # Taken from priority file (CANNOT be overwritten)
API_KEY=supersecret # Taken from priority file (CANNOT be overwritten)
DEBUG=false        # From file1.env
LOG_LEVEL=info     # From file2.env
```

#### Dry Run Mode

```sh
./target/release/envy-merge -d file1.env file2.env
```

This will print the merged output without saving it to a file.

#### Saving to a File

```sh
./target/release/envy-merge -o merged.env file1.env file2.env
```

## Error Handling

- If a file cannot be read, an error message is displayed, and the process exits with a non-zero code.
- Uses [`anyhow`](https://crates.io/crates/anyhow) for robust error handling.

## Contributing

Feel free to open issues or submit pull requests to improve functionality!

## License

This project is licensed under the MIT License.
