# Pigments

A color palette creator that extracts dominant colors from images using k-means clustering.

## Features

- Extract dominant colors from any image
- Customize the number of colors to extract
- Output in text or JSON format
- Save results to a file
- Production-ready with proper error handling and logging

## Installation

```bash
cargo install --path .
```

## Usage

```bash
# Basic usage
pigments -i image.jpg

# Extract 10 colors
pigments -i image.jpg -n 10

# Output in JSON format
pigments -i image.jpg -f json

# Save output to a file
pigments -i image.jpg -o palette.txt

# Get help
pigments --help
```

## Examples

1. Extract 5 colors from an image and display them:
```bash
pigments -i sunset.jpg
```

2. Extract 10 colors and save them in JSON format:
```bash
pigments -i sunset.jpg -n 10 -f json -o colors.json
```

## Development

To run tests:
```bash
cargo test
```

To run with logging:
```bash
RUST_LOG=info pigments -i image.jpg
```

## License

MIT