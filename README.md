# pathifier

A simple CLI tool to deduplicate PATH-like environment variables while preserving order.

## Usage

```bash
pathifier "$PATH"
```

This will output your PATH with duplicate entries removed, keeping only the first occurrence of each path.

## Example

```bash
$ pathifier "/usr/bin:/bin:/usr/bin:/usr/local/bin:/bin"
/usr/bin:/bin:/usr/local/bin
```

## Installation

```bash
cargo install --path .
```

The binary will be at `$HOME/.cargo/bin/pathifier`.

## Shell Integration

To automatically clean your PATH, add this to your shell config:

```bash
export PATH=$(pathifier "$PATH")
```

## Platform Support

- **Unix/Linux/macOS**: Uses `:` as delimiter
- **Windows**: Uses `;` as delimiter

## Testing

Run the test suite with:

```bash
cargo test
```

The tests include:
- **Integration tests** (`tests/integration_test.rs`): End-to-end tests covering deduplication, edge cases (empty input, single entry, unicode paths), and CLI behavior
- **Test data tests** (`tests/testdata_test.rs`): Data-driven tests using input/expected file pairs from `tests/testdata/`

## License

MIT
