# Upstream Mechanism Study: Arch Linux `mkinitcpio` and `arch-chroot`/`pacstrap` Integration

- **Upstream Repositories:** [archlinux/mkinitcpio](https://github.com/archlinux/mkinitcpio), [archlinux/arch-install-scripts](https://github.com/archlinux/arch-install-scripts)
- **Author:** Jules (SigmaOS Agent)
- **Date:** 2026-10-30
- **PR Scope:** `feat/distro-arch-mkinitcpio-chroot-pr-gateway`

---

## 1. Upstream Mechanism Overview

Arch Linux relies on two foundational utilities during system bootstrapping, initialization, and maintenance:
1. **`mkinitcpio`**: A modular C/Bash-based initramfs generator that parses hooks (`base`, `udev`, `autodetect`, `modconf`, `block`, `filesystems`, `fsck`) to construct a minimal root filesystem in memory before pivoting to real root (`/sysroot`).
2. **`arch-chroot` / `pacstrap`**: Shell-based environment provisioners that set up target root file trees, establish API bind mounts (`/proc`, `/sys`, `/dev`, `/run`), and execute isolated configuration commands without host system pollution.

---

## 2. Core Data Structures & Flow

### A. Initramfs Hook Pipeline
```text
HOOKS=(base udev autodetect modconf block filesystems fsck)
  ├── Resolve hook dependencies
  ├── Collect kernel modules & runtime binaries
  ├── Generate /init early-boot script
  └── Compress initramfs image (zstd/gzip)
```

### B. Chroot Provisioning Hierarchy
```text
/mnt/target (RootFS)
  ├── bind mount /proc  -> /mnt/target/proc
  ├── bind mount /sys   -> /mnt/target/sys
  ├── bind mount /dev   -> /mnt/target/dev
  └── bind mount /run   -> /mnt/target/run
```

---

## 3. SigmaOS Adaptation Decision

For SigmaOS:
- We implement `#![no_std]` compatible Rust primitives:
  1. `ArchMkinitcpioHookEngine`: Handles registration, dependency resolution, validation, and simulated early-boot execution of initramfs hooks.
  2. `ArchChrootProvisionerEngine`: Manages rootfs staging, required virtual mounts validation, safety isolation, and command execution within chroot bounds.
  3. `ArchEcosystemPrGateway`: Encapsulates these capabilities into reviewable pull request proposals with evidence generation.

---

## 4. Security & Failure Invariants

1. **Least Privilege & Isolation:** Chroot staging must verify mount bounds and enforce explicit cleanup on teardown to prevent mount point leaks.
2. **Fail Closed:** Any missing hook dependency or invalid rootfs directory structure must immediately halt execution and report diagnostic context.
3. **Reproducibility:** All hook generation and chroot staging scripts produce deterministic output logs suitable for automated CI verification.

---

## 5. Verification Plan

1. **Unit Tests:** Verify hook dependency ordering, missing hook handling, rootfs mount validation, and PR gateway proposal generation.
2. **Native Test Runner:** Integrate test execution into `./run_sigma_tests.sh`.
