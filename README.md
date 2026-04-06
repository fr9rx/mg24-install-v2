# mg24-install

`mg24-install` is a Rust-based installer for the **Silicon Labs Gecko SDK**, specifically for the **EFR32MG24** series.  

It automatically copies the necessary SDK folders (`CMSIS`, `emlib`, `Device`) into local hidden directories for use with [`mg24-hal`](https://github.com/<your-user>/mg24-hal):

- `~/.cmsis` – CMSIS headers and libraries  
- `~/.emlib` – EMLIB source files  
- `~/.mg24` – Device-specific files  

This allows you to quickly set up your development environment without manually moving SDK files.

---

## Installation

If published on crates.io:

```bash
cargo install mg24-install
