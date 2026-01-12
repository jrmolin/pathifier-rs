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

## License

MIT
