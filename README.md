# astra
Astra is an ABI stable, rust native tensor runtime that prioritizes reproducibility,reliability and designed as **infrastructure**.

The intention of Astra is not to replace existing ML stacks, but to define a small, explicit, and stable core C ABI that other systems can safely build upon. The intention is to eliminate hidden global state, treat CUDA and ROCm as infrastructures and avoid ABI fragility.

Python and other languages are just clients and not the owners of memory or execution. The system is designed to be a basis on which other systems can build upon.

## Core Design Principles

- **Stable ABI first**: the C ABI is the contract
- **Immutable tensors**: no in-place mutation, no implicit aliasing
- **Explicit devices & streams**: no global CUDA state
- **Precompiled kernels**: no runtime compilation
- **Deterministic execution**: same inputs → same outputs
- **Refuse to run if unsure**

## Why Rust was chosen?

This project requires 
- A **stable C ABI** as the primary interface
- Strong guarantees around **memory safety**
- Explicit ownership across FFI boundaries i.e clear ownership of the tensors and kernels
- Predictable behavior under concurrency
- clear separation of running safe and unsafe code like interacting with device memory, FFI integration etc

for these reasons rust was chosen as the primary language for this project.

## Why Nix package manager was chosen?

Nix provides fully declared, reproducible builds where every dependency like the compiler, libraries, CUDA runtime, kernel artifacts are explicit and versioned. This eliminates hidden system state, ABI and reproducibility issues.

Nix makes build inputs part of the contract, aligning with the stable ABI, precompiled kernel model, and maintainability.

## Project Status

Astra is in **early design and prototyping**.

Current focus:
- Architecture definition
- ABI stability
- Tensor semantics
- CPU first runtime



---

## Documentation

Architecture and design are documented using **arc42**:
