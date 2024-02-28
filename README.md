
# Image to ASCII Converter

This simple Rust program converts an image file into ASCII art. Each pixel in the image is replaced by an ASCII character based on its brightness level. The program resizes the image to reduce complexity and ensure the resulting ASCII art is visually coherent.

## How it Works

The program follows these steps:

1. Reads the filename of the image file to convert from command-line arguments.
2. Loads the image using the `image` crate.
3. Converts the image to grayscale (`Luma`) for simplicity.
4. Resizes the image to reduce complexity and ensure proper conversion.
5. Iterates over each pixel in the resized image and replaces it with an ASCII character based on its brightness level.

## Requirements

To run this program, you'll need:

- [Rust](https://www.rust-lang.org/tools/install) programming language: Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety.

  **Installation:**
  You can install Rust by running the following command in your terminal:

  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
## Usage

1. Clone this repository to your local machine.

```bash
git clone https://github.com/your-username/image-to-ascii.git
```

2. Navigate to the project directory.

```bash
cd image-to-ascii
```

3. Run the program with cargo, providing the path to the image file as an argument.

```bash
cargo run path/to/your/image.jpg
```

Replace `path/to/your/image.jpg` with the path to the image you want to convert.

## Example

Suppose you have an image file named `example.jpg`. To convert it to ASCII art, run the following command:

```bash
cargo run example.jpg
```

## Notes

- The resulting ASCII art will be printed to the console. You can redirect the output to a file if needed.
- For best results, use images with clear contrast and details.

## Credits

This program was created by Rofernweh. Feel free to contribute or report issues on GitHub.
