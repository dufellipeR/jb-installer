
![Logo](/assets/painiteI.png)


# ♦️ Painite

A cli tool that installs tar.gz JetBrains IDEs into Linux


## Features

- Unpack .tar.gz files
- Create symbolic link
- Create destktop entry

### Supported IDEs

✓ IntelliJ IDEA\
✓ Rust Rover\
✓ Pycharm\
✓ GoLand


## Installation

Install with binaries

```bash
curl -LSfs https://japaric.github.io/trust/install.sh | sh -s -- --git your-name/repo-name
```

## Usage/Examples

```bash
sudo painite <gz_file_path>

> Welcome to Painite !
> ✅ successfully created JetBrains directory
> ✅ successfully unpacked IDE, took 109.65s
> ✅ successfully created symbolic link
> ✅ successfully created desktop entry
-----------------------------------------------
> IntelliJ IDEA successfully installed
```


## Contributing

Contributions are always welcome!



## Running Tests

To run tests, run the following command

```bash
  cargo test --package painite --lib tests
```

