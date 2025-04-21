# DicStructureCreater

![Crates.io](https://img.shields.io/crates/v/dic-structure-creater)
![License](https://img.shields.io/badge/license-MIT-blue)
![Stars](https://img.shields.io/github/stars/natsuki221/DicStructureCreater)
![Downloads](https://img.shields.io/crates/dr/dic-structure-creater)

DicStructureCreater is a lightweight, cross-platform CLI tool that parses directory structures from `.txt` or `.md` files and generates corresponding folders and files under a specified root directory. By default, it avoids overwriting existing files, and supports dry-run (`--dry-run`) and force overwrite (`--overwrite`) modes.

---
## README
[zh_TW](/docs/README_zh_TW.md)

## Features

- 📁 **Automatic generation**: Generate folder structures and empty files from `tree` ASCII diagrams or Markdown-style lists.
- 🔒 **Safe by default**: Existing files won't be overwritten unless explicitly allowed.
- 🔄 **Optional overwrite**: Use `--overwrite` (`-O`) to force replace files.
- 👀 **Dry-run support**: Use `--dry-run` (`-d`) to preview actions without changing your file system.
- ⚙️ **Cross-platform**: Built with Rust, compatible with Windows, macOS, and Linux.

---

## Installation

### Using Cargo
```bash
cargo install dic-structure-creater
```

### Download Precompiled Binary
1. Visit the Releases page:  
   https://github.com/natsuki221/DicStructureCreater/releases
2. Download the archive for your OS, extract it, and place `dsc` (or `dsc.exe`) in your system's `PATH`.

---

## Example Usage

Given a structure file `structure.txt`:
```txt
src/
├── lib/
│   └── utils.rs
└── main.rs
```

1. **Preview only** (dry-run):
```bash
dsc structure.txt --dry-run
```

2. **Generate structure** (without overwriting existing files):
```bash
dsc structure.txt
```

3. **Force overwrite existing files**:
```bash
dsc structure.txt --overwrite
```

---

## CLI Options

| Option                   | Description                                        |
|--------------------------|----------------------------------------------------|
| `STRUCTURE_FILE`         | Path to the input structure file (required)        |
| `-r, --root <path>`      | Root directory to create files/folders (default: `.`) |
| `-O, --overwrite`        | Overwrite existing files                           |
| `-d, --dry-run`          | Show actions without making changes                |
| `-h, --help`             | Display help message                               |

---

## Supported Input Formats

DSC supports two main formats:

1. **Tree ASCII format**:
```txt
src/
├── models/
│   └── user.rs
└── main.rs
```

2. **Markdown list format**:
```md
- src
  - models
    - user.rs
  - main.rs
```

> **Note**: Lines ending with `/` are treated as folders; otherwise, they are considered files.

---

## Contributing

Contributions are welcome! You can submit Issues, Pull Requests, suggestions, or bug reports.

Steps:
1. Fork this repository
2. Create a new branch (`git checkout -b feature/your-feature`)
3. Implement your changes
4. Submit a Pull Request

---

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.

---

> For questions, feel free to open an issue or discussion. Happy hacking! 🚀

