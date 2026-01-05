# 🧱 UTXO Ledger in Rust

A minimal implementation of a **Bitcoin-style UTXO (Unspent Transaction Output) ledger** written in pure Rust.

This project focuses on the **core accounting logic** behind blockchains:
how coins are tracked, transferred, and protected from double spending —  
without networking, mining, or cryptography.

---

## Core Concepts

### What is a UTXO?
A **UTXO (Unspent Transaction Output)** represents a piece of value that:
- belongs to someone
- can be spent exactly once

There are **no balances stored**.
A wallet’s balance is derived by summing its UTXOs.

