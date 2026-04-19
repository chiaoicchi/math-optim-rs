# Math & Optimization Algorithms for Rust

Mathematical optimization library built on algebraic trait abstractions (Band, Monoid, Group, Ring, Field).

Correctness is verified via [math-optim-rs-verify](https://github.com/chiaoicchi/math-optim-rs-verify).

## Crates

| Crate | Contents |
|-------|----------|
| algebrae | GF(p), Miller-Rabin, Pollard's rho, Eratosthenes sieve, NTT, FPS, Gaussian elimination |
| data-strux | Segment tree (plain / lazy / dual), Fenwick tree, Sparse table, DSU (plain / weighted) |
| geome | 2D point / vector, convex hull, argument sort |
| graphia | CSR, Dinic's max flow, Kosaraju's SCC, LCA, Heavy path decomposition, Euler tour, tree diameter |
| seqenz | LIS, Directed acyclic subsequence graph (DASG) |

## Environment

- Rust 1.89.0 (pinned via [rust-toolchain.toml](rust-toolchain.toml))
- Nix flake for reproducible development environment

## Setup

### Prerequisites

- [Nix](https://nixos.org/) with flakes enabled
- [direnv](https://github.com/direnv/direnv) (optional)

### Getting Started

Enter the development environment:

```bash
nix develop
```

Or, if you have direnv set up:

```bash
direnv allow
```

## License

This project is licensed under the [MIT License](LICENSE).
