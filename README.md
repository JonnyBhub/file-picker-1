# file-picker

## Overview

`file-picker` is an interactive command-line tool that allows users to navigate through their folder structure visually. It supports both keyboard and mouse interactions, enabling users to expand or collapse folders and open files seamlessly.

## Features

- Display folder structure with icons for folders and files.
- Interactive navigation using keyboard and mouse.
- Expand and collapse folders to view contents.
- Open files directly from the interface.

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

### Installation

1. Clone the repository:

   ```
   git clone <repository-url>
   cd file-picker
   ```

2. Build the project:

   ```
   cargo build
   ```

3. Run the application:
   ```
   cargo run
   ```

### Usage

- Use the arrow keys to navigate through the folder structure.
- Press `Enter` to open a file.
- Click on folders to expand or collapse them.

## Project Structure

```
file-picker
├── src
│   ├── main.rs          # Entry point of the application
│   ├── app.rs           # Main application logic
│   ├── ui.rs            # User interface rendering
│   ├── events.rs        # User input event management
│   ├── fs
│   │   ├── mod.rs       # File system operations module
│   │   ├── tree.rs      # Folder tree structure
│   │   └── icons.rs     # Icons for folders and files
│   └── types
│       └── mod.rs       # Custom types and structures
├── Cargo.toml           # Project configuration
├── .gitignore           # Git ignore file
└── README.md            # Project documentation
```

## Customisation

You are able to customise the look and behavior of the file-picker.
Each file extension can have a default program you want to open with.

**Macos program example**
'''
FILE_PICKER_EXT_rs ='open -a "Visual studio code"'
FILE_PICKER_EXT_md ='open -a "preview"'

'''

**Macos color example**
'''
FILE_PICKER_EXT_rs ='open -a "Visual studio code"'
FILE_PICKER_EXT_md ='open -a "preview"'

'''

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any enhancements or bug fixes.

## License

This project is licensed under the MIT License. See the LICENSE file for details.
