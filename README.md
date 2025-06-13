# 🔐 Rust Password Generator

A secure, flexible, and user-friendly password generator written in Rust.  
This tool allows you to generate strong, random passwords with customizable options, calculate their entropy, and assess their strength.

---

## ✨ Features

- **Interactive mode**: Answer prompts in your terminal to generate passwords step by step.
- **Command-line mode**: Use CLI arguments for quick, scriptable password generation.
- **Customizable character sets**: Choose lowercase, uppercase, digits, symbols, or any combination.
- **Multiple password generation**: Generate one or many passwords at once.
- **Entropy calculation**: See the entropy and strength classification for each password.
- **Modular codebase**: Easy to extend and maintain.

---

## 🚀 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (edition 2021 or newer)

### Build & Run

Clone the repository and build the project:

```bash
git clone https://github.com/yourusername/rust-password-generator.git
cd rust-password-generator
cargo build --release
```

---

## 🛠️ Usage

### 🖥️ Interactive Mode

Simply run:

```bash
cargo run
```

You will be prompted for:
- Password length
- Number of passwords to generate
- Whether to include lowercase, uppercase, digits, and symbols

Example session:
```
rust-password-generator - Secure password generator CLI
(Interactive mode)
Enter password length (default 16): 20
Enter number of passwords to generate (default 1): 2
Include character sets? (y/n, default: y)
Lowercase letters? y
Uppercase letters? y
Digits? y
Symbols? n

Pass 1:
yQw8z2Xv9pLm4sRt1uIo

Pass 2:
aB3dE5fG7hJ9kL1mN2pQ

Password 1: entropy = 94.60 bits (Strong)
Password 2: entropy = 94.60 bits (Strong)
```

---

### ⚡ Command-Line Mode

You can also generate passwords directly using CLI arguments:

```bash
cargo run -- --length 20 --symbols --count 3
```

#### Available options:

- `--length <LENGTH>`: Length of each password (default: 16)
- `--lowercase` / `--no-lowercase`: Include/exclude lowercase letters (default: include)
- `--uppercase` / `--no-uppercase`: Include/exclude uppercase letters (default: include)
- `--digits` / `--no-digits`: Include/exclude digits (default: include)
- `--symbols` / `--no-symbols`: Include/exclude symbols (default: exclude)
- `--count <COUNT>`: Number of passwords to generate (default: 1)

#### Example:

```bash
cargo run -- --length 12 --no-symbols --count 2
```

---

## 📊 Entropy Calculation

For each generated password, the program calculates its entropy using the formula:

```
entropy = password_length × log2(charset_size)
```

- **password_length**: Number of characters in the password
- **charset_size**: Number of unique characters available for generation

**Strength classification:**
- 🟥 **Weak**: < 40 bits
- 🟨 **Medium**: 40–60 bits
- 🟩 **Strong**: > 60 bits

---

## 📝 Example Output

```
Pass 1:
A1b2C3d4E5f6

Pass 2:
G7h8I9j0K1l2

Password 1: entropy = 71.51 bits (Strong)
Password 2: entropy = 71.51 bits (Strong)
```

---

## ⚠️ Security Notice

- Generated passwords are displayed in clear text in your terminal.  
  **Ensure your environment is secure and do not share your screen when generating sensitive passwords.**
- For even greater security, consider adding clipboard support or file output.

---

## 📄 License

This project is licensed under the MIT License.

---

## 👤 Author

Created by Franciszek Tokarek.  
