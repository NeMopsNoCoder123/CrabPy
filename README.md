# 🦀 CrabPy Programming Language

![Version](https://img.shields.io/badge/version-1.1.0-orange?style=for-the-badge)
![Rust](https://img.shields.io/badge/Engine-Rust-red?style=for-the-badge&logo=rust)
![Python](https://img.shields.io/badge/Runtime-Python_3.12-blue?style=for-the-badge&logo=python)
![License](https://img.shields.io/badge/license-MIT-green?style=for-the-badge)

**CrabPy** is a high-performance programming language (transpiler) that redefines the Python development experience. Built with a lightning-fast **Rust** core, it replaces Python's indentation-based system with a modern curly-brace `{}` syntax while maintaining full compatibility with the Python ecosystem.



---

## ✨ Key Features

- **No More IndentationErrors:** Structure your code using `{ }` blocks, just like in C++, Rust, or JavaScript.
- **Powered by Rust:** Lexical analysis and keyword mapping are performed by a native Rust binary for near-instant execution.
- **Full Library Support:** Import any Python library (`tkinter`, `pygame`, `pandas`, `requests`) using the `grab` keyword.
- **Modern F-Strings:** Native support for string interpolation: `f"Hello, {name}"`.
- **Integrated Package Manager:** Manage your dependencies with the built-in `Shrimp` tool.

---

## 🚀 Keyword Mapping (CrabPy Dictionary)

We have modernized 35 standard Python keywords to improve the developer experience:

| CrabPy | Python | Description |
| :--- | :--- | :--- |
| `fn` | `def` | Function definition |
| `grab` | `import` | Import a module |
| `print` | `print` | Output to console |
| `ask` | `input` | User input |
| `true` | `True` | Boolean True |
| `false` | `False` | Boolean False |
| `null` | `None` | NoneType / Null |

*Note: All other standard keywords (`if`, `else`, `for`, `while`, `try`, `class`, etc.) remain the same but utilize `{ }` syntax.*

---

## 📦 Shrimp: Package Manager

CrabPy comes with **Shrimp**, a streamlined CLI tool for managing your libraries.

```bash
# Install a package
shrimp net requests

# Remove a package
shrimp toss requests
```

---

## 💻 Code Example

The example code for test working CrabPy files

```rust
grab tkinter

fn greet(name) {
    if name != "" {
        print(f"Hello, {name}! Welcome to CrabPy.")
    } else {
        print("Hello, anonymous!")
    }
}

# GUI Setup
root = tkinter.Tk()
root.title("CrabPy GUI App")

user_name = ask("What is your name? ")
greet(user_name)

root.mainloop()
```

---

## 🔨 Installation & Build

**Prerequisites**
- **Rust (Cargo)** to compile the core engine.
- **Python 3.12+** (3.13 recommended) to provide the runtime environment.
**Building from source**
1. Clone the repository:
   ```bash
   git clone [https://github.com/NeMopsNoCoder123/crabpy.git](https://github.com/NeMopsNoCoder123/crabpy.git)
   cd crabpy
   ```
2. Configure environment and build:
   ```powershell
   # Windows (PowerShell)
   $env:PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1
   cargo build --release
   ```
3. Your executable ```crabpy.exe``` will be located in ```target/release/```.

---

## 🏗 Architecture
**CrabPy** operates as a multi-layer **transpiler**:
1. **Lexical Analysis (Rust)**: Reads the ```.crpy``` file and swaps keywords using optimized Regex patterns.

2. **Indentation Engine**: Analyzes ```{}``` nesting levels and converts them into Python-compliant whitespace.

3. **PyO3 Bridge**: Utilizes the embedded Python C-API to execute the processed code within a high-performance runtime.

---

## 📄 License
**This project** is licensed under the **MIT License**.

Developed with 🦀 and ⚡ by the **CrabPy MrApocs / NeMopsNoCoder123**.

## 📞 Contact
**Email**: mrapocscrabpycontact@gmail.com

**Discord Channel**: https://discord.gg/VkGqCsFg

**Telegram Channel**: www.t.me/crabpyofficial
