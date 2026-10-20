# Pull Request Proposal: Gentoo Linux System Architecture & Portage Optimization Parity

## Executive Summary

This Pull Request proposal introduces a comprehensive, `#![no_std]` and `alloc`-compatible Gentoo Linux System Architecture & Portage Parity framework for SigmaOS, drawing architectural inspiration directly from Gentoo Linux (`gentoo/gentoo` GitHub repository & Portage package manager).

The goal is to achieve native source-based compilation governance, EAPI-8 slotted dependency resolution, USE-flag expression evaluation, package set management (`@world`, `@system`, `@selected`, `@profile`), configuration merge management (`etc-update` / `dispatch-conf`), distfile and binary package cleanup (`eclean`), and cryptographically verified binhost client integration.

---

## Targeted Tools & Subsystems Inspired by Gentoo Linux

| Subsystem / Utility | Gentoo Inspiration | Implementation in SigmaOS |
| :--- | :--- | :--- |
| **Portage Package Sets** | `@world`, `@system`, `@selected`, `@profile` sets | `GentooPackageSetManager` for hierarchical tracking of system dependencies and user-selected software. |
| **Configuration File Merge Engine** | `etc-update`, `dispatch-conf`, `._cfg0000_` updates | `GentooConfigMergeEngine` for detecting, diffing, and merging configuration updates without overwriting customized user settings. |
| **Distfile & Binary Cleaners** | `eclean-dist`, `eclean-pkg`, `gentoolkit` | `GentooEcleanEngine` for scanning and purging obsolete source tarballs and stale binary packages. |
| **Binhost Client Engine** | `PORTAGE_BINHOST`, `emerge --usepkg` | `GentooBinhostEngine` for secure, hash-verified binary package fetching and installation. |
| **USE-Flag & EAPI Resolution** | `IUSE`, `use.mask`, EAPI-8 slot operators | `UseFlagManager`, `PortageSlotResolver`, and `EbuildSandbox` in `src/package/gentoo_portage.rs` and `src/package/gentoo_opt.rs`. |
| **Compilation Tuning & Genkernel** | CFLAGS/CXXFLAGS optimization, `genkernel` | `GccOptimizationTuner` and `GenkernelOrchestrator` for automated hardware-tailored kernel and initramfs builds. |

---

## Architectural Architecture & Design

### 1. Portage Package Sets Manager (`GentooPackageSetManager`)
- **Package Set Abstraction**: Support for standard Gentoo sets (`@system`, `@selected`, `@world`) and custom user-defined sets (`@set`).
- **World Set Resolution**: Dynamically calculates `@world` membership as the union of `@system` (base operating system required tools) and `@selected` (explicitly requested user applications).

### 2. Configuration Merge Engine (`GentooConfigMergeEngine`)
- **Protected Path Monitoring**: Intercepts protected configuration files in `/etc` when new versions are installed with `._cfg0000_` prefixes.
- **Resolution Policies**: Provides automated and interactive merge strategies (`UseNew`, `KeepCurrent`, `InteractiveMerge`, `DiffSummary`).

### 3. Source & Binary Package Cleanup Engine (`GentooEcleanEngine`)
- **Distfile Purging**: Tracks source archives in `/var/cache/distfiles` and purges unreferenced or obsolete tarballs (`eclean_distfiles`).
- **Binary Package Purging**: Identifies and cleans stale `.tbz2`/`.gpkg` binary archives in `/var/cache/binpkgs` (`eclean_pkg`).

### 4. Binary Package Host Engine (`GentooBinhostEngine`)
- **Pre-Compiled Acceleration**: Resolves pre-built binary packages from configured remote binhosts.
- **Integrity Verification**: Enforces SHA-256 checksum validation for binary packages before installation.

---

## Verification & Status

Verified via standalone Rust unit tests (`rustc --test src/package/gentoo_portage.rs --edition=2021 -o build/gentoo_portage_test && ./build/gentoo_portage_test`):
- `test_portage_emerge`
- `test_use_flags`
- `test_package_set_manager`
- `test_config_merge_engine`
- `test_eclean_engine`
- `test_binhost_engine`

Status: **Ready for Merge**.
