# astra
Astra is an ABI stable, rust native tensor runtime that prioritizes reproducibility,reliability and designed as **infrastructure**.

The intention of Astra is not to replace existing ML stacks, but to define a small, explicit, and stable core C ABI that other systems can safely build upon. The intention is to eliminate hidden global state, treat CUDA and ROCm as infrastructures and avoid ABI fragility.

Python and other languages are just clients and not the owners of memory or execution. The system is designed to be a basis on which other systems can build upon.


## What was the problem with ABI of existing ML stacks like pytorch and tensorflow?

Even though pytorch defines an ABI for C++ it is not stable one. The reason for the same is if we look at the setup.py file of pytorch, toolchain ABI is dependent upon host system toolchain and GLIBC version, the symbols may not exist on older systems. secondly pytorch compiles CUDA code against headers found in build time, which differs from the one in runtime.  This means that the same code can be compiled with different compilers, different CUDA versions, or even different architectures (e.g: x86 vs ARM). The result is an ABI that is based upon distro and how pytoch is built.

Tensorflow has better and stable ABI design, but the execution model semantics were changed from TF1 to TF2 , which means the behavior has changed for the same ABI. For e.g consider the below code snippet 
```
x = tf.placeholder(tf.float32)
y = x + 1

```
in TF1 the above code would not be executed until Session.run() is encountered . In TF2 the above code would be executed immediately. There are other similar behavioral changes which are documented in the [Tensorflow 2.0 Migration Guide](https://www.tensorflow.org/guide/migrate/tf1_vs_tf2). 

The main reason for this is that Tensorflow 2.0 has a different execution model than TF1, which means it's not possible to predict what will happen in the future without understanding the semantics of the underlying system.

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

The Rust Cargo build  ensures single standard library is used, single dependency graph, and a single compiler.The versions are locked and ensures consistency across platforms, compilers, and toolchains. It also provides a clear separation of concerns and for these reasons mentioned above rust was chosen as the primary language for this project.

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
