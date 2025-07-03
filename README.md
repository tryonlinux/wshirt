# Wear Shirt

Wear Shirt is a command line application for managing when you last wore a shirt.

This should help prevent you from wearing the same shirt twice in a time period to prevent time wasted wondering if you wore that shirt recently when getting dressed.

You can add, wear, list, delete, and reset shirts using simple commands.

## Features

- Add a new shirt
- Wear a shirt
- List all shirts
- Delete a shirt by name or number
- Reset (clear) the shirt database
- Show version with `wshirt version` or `wshirt --version`
- Cross-Platform (Linux, MacOS, and Windows)

## Installation

To install the Shirt, clone the repository and build the project using Cargo:

```bash
git clone https://github.com/tryonlinux/wshirt
cd wshirt
cargo build --release
```

Or download a prebuilt binary from the [GitHub Releases](https://github.com/tryonlinux/wshirt/releases) page.

## Usage

Run the application with the following command:

```sh
wshirt <command> [options]
```

### Commands

- `add <name>`: Add a new shirt with the specified name.
- `wear <name|number>`: Wear the shirt with the specified name or number.
- `list`: List all shirts in your collection.
- `delete <name|number>`: Delete the shirt by name or number.
- `reset`: Clear the shirt database.
- `version`, `--version`, `-v`: Show the program version.

## Examples

1. Add a new shirt:

   ```sh
   wshirt add "Cool T-Shirt"
   ```

2. Wear a shirt:

   ```sh
   wshirt wear "Cool T-Shirt"
   ```

3. List all shirts:

   ```sh
   wshirt list
   ```

4. Delete a shirt:

   ```sh
   wshirt delete "Cool T-Shirt"
   ```

5. Reset the database:

   ```sh
   wshirt reset
   ```

6. Show version:
   ```sh
   wshirt version
   ```

## Contributing

Feel free to submit issues or pull requests to improve the Wear Shirt.
