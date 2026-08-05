# Contributing to Equator Contracts (`equator-contracts`)

Thank you for your interest in contributing to **Equator Finance**! We welcome contributions from developers, security researchers, and Web3 enthusiasts.

---

## 🛠 Prerequisites & Local Setup

To build and test the Soroban smart contracts locally, ensure you have installed:
1. **Rust:** `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. **WASM Target:** `rustup target add wasm32-unknown-unknown`
3. **Soroban CLI:** `cargo install --locked soroban-cli`

### Running Local Tests
```bash
# Clone the repository
git clone https://github.com/Equator-Finance/equator-contracts.git
cd equator-contracts

# Run Cargo unit & integration tests
cargo test
```

---

## 📜 Development Guidelines

### 1. Branching Strategy
* Create a feature branch off `main`: `git checkout -b feature/your-feature-name` or `fix/issue-description`.
* Keep commits focused, descriptive, and atomic.

### 2. Code Quality & Security Rules
* All contracts must compile targeting `wasm32-unknown-unknown` without warnings.
* Ensure 100% test coverage for any new smart contract state transitions or authorization checks.
* Run `cargo fmt` and `cargo clippy` before submitting a Pull Request.

### 3. Submitting a Pull Request (PR)
* Open a PR against the `main` branch.
* Describe the problem being solved and detail the on-chain state changes introduced.
* Ensure all continuous integration (CI) tests pass.

---

## 🔐 Security & Bug Bounties

If you discover a security vulnerability in these smart contracts, **do not** open a public issue. Please report it directly to the core security team at `security@equator.finance`.
