# 🦀 CrabPy Programming Language

![Version](https://img.shields.io/badge/version-1.5.0-orange?style=for-the-badge)
![Rust](https://img.shields.io/badge/Engine-Rust-red?style=for-the-badge&logo=rust)
![Python](https://img.shields.io/badge/Runtime-Python_3.12-blue?style=for-the-badge&logo=python)
![License](https://img.shields.io/badge/license-MIT-green?style=for-the-badge)

A C-style syntax layer on top of Python, powered by Rust.

CrabPy is a lightweight programming language that transpiles a brace-based syntax into valid Python code and executes it using CPython via PyO3. It allows writing Python code using {} blocks instead of indentation.

## How CrabPy Works?

CrabPy is not a new virtual machine. It is a transpiler and runtime executor.

**Workflow:**

- Reads a .crpy file
- Replaces CrabPy keywords with Python keywords
- Converts {} blocks into Python indentation
- Executes the generated Python code using CPython through PyO3

You get full Python compatibility with C-style block syntax.

## 💻Example

🦀**CrabPy code:**
```rust
grab math
fn main() {
  if true {
    print(math.sqrt(16))
  }
}
```

🐍**Generated Python:**
```python
import math
def main():
  if True:
    print(math.sqrt(16))
```

## 📱Supported Keywords

CrabPy maps directly to Python.

- `grab` → `import`
- `fn` → `def`
- `true` → `True`
- `false` → `False`
- `null` → `None`
- `ask` → `input`
- `class` → `class`
- `if` / `elif` / `else` → `if` / `elif` / `else`
- `for` → `for`
- `while` → `while`
- `and` / `or` / `not` → `and` / `or` / `not`
- `return` → `return`
- `try` / `except` / `finally` → `try` / `except` / `finally`
- `async` / `await` → `async` / `await`

All other syntax is passed directly to Python.

## 📙Blocks and Syntax

CrabPy🦀 uses curly braces instead of indentation.

**Example:**
```rust
fn test() {
  if true {
    print("Hello")
  }
}
```

This becomes:
```python
def test():
  if True:
    print("Hello")
```

## 💿Running CrabPy

```
crabpy file.crpy
```

Only .crpy files are supported.

## Shrimp 🍤

Package manager for CrabPy.

Shrimp is a lightweight wrapper around pip, so CrabPy projects can install Python packages.

### Installing packages

```bash
shrimp net requests
```

This runs:
```bash
pip install requests
```

### Removing packages

```bash
shrimp toss requests
```

This runs:
```bash
pip uninstall requests -y
```

## Requirements

- Python 3.12+
- pip
- Rust

## 🧿Current Limitations

CrabPy uses a regex-based transpiler. There is no full parser, type system, or static analysis. Errors are reported by the Python runtime. However, it is fast, simple, and fully compatible with Python libraries.

## 👨‍🎓Philosophy

CrabPy is Python without indentation. It is built for developers who want Python's ecosystem with C-style syntax.

## 📢Contributing

We welcome contributions to CrabPy! Whether it's bug reports, feature requests, or code contributions, please feel free to open an issue or submit a pull request on our [GitHub repository](https://github.com/NeMopsNoCoder123/CrabPy).

### Development Setup

1. Clone the repository
2. Install Rust and Python
3. Build the project: `cargo build`
4. Run tests: `cargo test`


## 🧾License

CrabPy use MIT License
