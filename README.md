<div align="center">

# 📚 Solana Library Program

[![Solana](https://img.shields.io/badge/Solana-Blockchain-9945FF?style=for-the-badge\&logo=solana\&logoColor=white)](https://solana.com/)
[![Rust](https://img.shields.io/badge/Rust-Programming_Language-000000?style=for-the-badge\&logo=rust\&logoColor=white)](https://www.rust-lang.org/)
[![Anchor](https://img.shields.io/badge/Anchor-Framework-1E1E1E?style=for-the-badge)](https://www.anchor-lang.com/)
[![License](https://img.shields.io/badge/License-MIT-green?style=for-the-badge)](LICENSE)

**A decentralized on-chain library management program built with Anchor on Solana**

</div>

---

## 🎯 About The Project

**Solana Library Program** is a smart contract built using the Anchor framework that allows users to create and manage a personal on-chain digital library.

Each user can:

* Create their own library (PDA-based account)
* Add books
* Remove books
* Toggle book availability
* View stored books

All data is stored directly on-chain using Program Derived Addresses (PDAs).

---

## ✨ Features

### 📖 Library Management

* Create a personal library (PDA per user)
* Store library name
* Owner-based access control

### 📚 Book Management

* Add books with:

  * Name
  * Number of pages
  * Availability status
  * Delete books by name
  * Toggle availability (available / not available)
  * View all stored books via transaction logs

### 🔐 Security

* Owner validation using `require!`
* PDA derived from:

  * Static seed (`"biblioteca"`)
  * Owner public key
  * Program ID
* Custom error handling with Anchor `#[error_code]`

---

## 🏗️ Program Architecture

### 📦 Main Account

```rust
Biblioteca {
    owner: Pubkey,
    nombre: String,
    libros: Vec<Libro>
}
```

### 📘 Internal Struct

```rust
Libro {
    nombre: String,
    paginas: u16,
    disponible: bool
}
```

### 🔑 PDA Seeds

```
["biblioteca", owner_pubkey]
```

Each user can only create and manage their own library.

---

## 🚀 Installation & Setup

### Prerequisites

Make sure you have installed:

| Requirement | Version       |
| ----------- | ------------- |
| Rust        | Latest stable |
| Solana CLI  | 1.17+         |
| Anchor CLI  | 0.29+         |
| Node.js     | 18+           |
| Yarn        | Latest        |

---

### 🔧 Quick Start

```bash
# Clone the repository
git clone https://github.com/cesarMalanco/solana-library.git

# Navigate into project
cd solana-library

# Install dependencies
yarn install

# Build the program
anchor build

# Run local validator
solana-test-validator

# Deploy locally
anchor deploy

# Run tests
anchor test
```

---

## 🧪 Available Instructions

| Instruction        | Description               |
| ------------------ | ------------------------- |
| `crear_biblioteca` | Creates a new library PDA |
| `agregar_libro`    | Adds a new book           |
| `eliminar_libro`   | Removes a book by name    |
| `ver_libros`       | Logs all books            |
| `alternar_estado`  | Toggles availability      |

---

## 📁 Project Structure

```
solana-library/
│
├── programs/
│   └── solana_library/
│       └── src/lib.rs        # Smart contract logic
│
├── tests/                    # Mocha tests
├── migrations/               # Deployment scripts
│
├── Anchor.toml               # Anchor configuration
├── Cargo.toml                # Rust dependencies
├── package.json              # JS dependencies
└── tsconfig.json             # TypeScript config
```

---

## 🛡️ Error Handling

The program defines custom errors:

```rust
NoEresElOwner
LibroNoExiste
```

These ensure:

* Only the library owner can modify data
* Invalid operations are safely rejected

---

## 🧠 Key Concepts Used

* Program Derived Addresses (PDAs)
* Anchor `#[account]` macro
* `InitSpace` for storage calculation
* `require!` macro for access control
* Custom error codes
* On-chain vector storage

---

## 🌐 Deployment

To deploy to Devnet:

```bash
solana config set --url devnet
anchor deploy
```

To deploy to Mainnet:

```bash
solana config set --url mainnet-beta
anchor deploy
```

---

## 📜 License

This project is licensed under the MIT License.

---

## 👨‍💻 Author

<div align="center">

**César M.**

Built with ❤️ using Rust, Anchor & Solana

</div>

---
