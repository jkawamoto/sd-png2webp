# sd-png2webp
[![Rust application](https://github.com/jkawamoto/sd-png2webp/actions/workflows/ci.yaml/badge.svg)](https://github.com/jkawamoto/sd-png2webp/actions/workflows/ci.yaml)

sd-png2webp is a tool for converting PNG images to WebP format, built with Rust and designed to operate in a multi-threading. It also embeds generation data from the AUTOMATIC1111 Stable Diffusion web UI into the converted WebP files.

## Usage
The usage is as follows:

```
sd-png2webp path...
```

The `path` argument can be either a file or a directory. If a directory is specified, the tool will recursively search for PNG files in that directory and its subdirectories and convert them. You can specify multiple paths.

## Installation
```shell
cargo install --git https://github.com/jkawamoto/sd-png2webp
```

## License
This application is released under the MIT License. For details, see the [LICENSE](LICENSE) file.
