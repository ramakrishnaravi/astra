# 1. Introduction and Goals

## 1.1 Introduction

Astra is an ABI stable, rust native tensor runtime that prioritizes reproducibility, and long-term reliability and designed as **infrastructure**.

What started as reproducible builds of existing ml stacks targeting different builds has driven me into a rabbit hole of issues and was the main reason to start this project. The main intention is to address problems in existing ML stacks i.e: unstable ABIs, vague dependency management, implicit assumptions and poor reproducible builds.

The intention of Astra is not to replace existing ML stacks, but to define a small, explicit, and stable core C ABI that other systems can safely build upon. The intention is to eliminate hidden global state, treat CUDA and ROCm as infrastructures and avoid ABI fragility.

Python and other languages are just clients and not the owners of memory or execution. The system is designed to be a basis on which other systems can build upon.
---

## 1.2 Purpose of the System

The purpose of Astra is to provide:

- **stable binary interface** for tensor computation
- **deterministic execution environment**
- **reproducible builds and reproducible systems**
---

## 1.3 Target Audience

- ML infrastructure engineers
- Systems programmers
- Inference and deployment teams
- Organizations requiring long-term stability

---

## 1.4 High-Level Goals

### G1 — ABI Stability
Provide a C ABI that remains stable across versions, toolchains, and languages.

### G2 — Explicit Semantics
Eliminate hidden global state and implicit mutation.

### G3 — Reproducibility
Ensure builds and executions are deterministic and inspectable.

### G4 — Safe Embedding
Allow Astra to be embedded in services, applications, and other runtimes
without leaking ownership or control.

### G5 — Long-Term Maintainability
Favor designs that reduce complexity, surface area, and failure modes.

---

## 1.5 Non-Goals

Astra explicitly does not aim to:

- Replace PyTorch or TensorFlow as research frameworks
- Support runtime kernel compilation or tracing
- Optimize for minimum code or maximum convenience

These concerns may be addressed by layers built on top of Astra.

---

## 1.6 Success Criteria

- ABI Stability, i.e  a binary built today can be run even after few years
- Unsupported configurations fail early and clearly
- Kernels can be reasoned about in isolation
- There are no vague and implicit dependencies, the complete dependency graph is available at any point of time.

