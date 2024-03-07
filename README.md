<div align="center">

![Logo](/assets/painiteIII.png)

</div>

# ♦️ painite

A cli tool that installs tar.gz JetBrains IDEs into Linux

<div align="center">

![Usage](/assets/pntIII.gif)

</div>

## Features

- Unpack .tar.gz files
- Create symbolic link
- Create desktop entry

### Supported IDEs

✓ IntelliJ IDEA\
✓ Pycharm\
✓ RubyMine\
✓ Rust Rover\
✓ GoLand


## Installation

Install with binaries

1- Visit the releases page.\
2- Look for the latest release (e.g., v1.0.0).\
3- Download the binary to your preferred location (e.g., your home directory).

## Usage/Examples

1- Open a terminal and navigate to the directory where you downloaded the binary. Then give it permission to run:

```bash
chmod +x painite
```
2- Run the binary:
```bash
sudo ./painite <gz_file_path>

> Welcome to Painite !
> ✅ successfully created JetBrains directory
> ✅ successfully unpacked IDE, took 18s
> ✅ successfully created symbolic link
> ✅ successfully created desktop entry
-----------------------------------------------
> IntelliJ IDEA successfully installed
```


## Contributing

If you encounter any issues or have suggestions, feel free to open an issue on our GitHub repository.


## Running Tests

To run tests, run the following command

```bash
  cargo test
```

