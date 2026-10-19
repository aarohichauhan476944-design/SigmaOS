# Study: CachyOS Kernel, Compiler & Hardware Performance Advancements

## Reference System

- **CachyLinux (CachyOS)**:
  - High-performance Arch Linux derivative optimized for x86-64 microarchitecture levels (`x86-64-v3` and `x86-64-v4`).
  - Integrates `sched_ext` BPF schedulers (`scx_bore`, `scx_lavd`, `scx_rusty`, `scx_central`) for ultra-low latency interactive desktop & gaming workloads.
  - Features `chwd` (CachyOS Hardware Detection) for automated driver installation (Nvidia Open-DKMS, Mesa v3/v4, PRIME hybrid graphics).
  - Uses tuned kernel build pipeline with AutoFDO, Propeller, ThinLTO/FullLto, and `-march=x86-64-v3/v4` auto-vectorization.
  - Implements `cachyos-settings` with THP (`always`/`madvise`), ZRAM zstd swappiness=180, Kyber/MQ-Deadline NVMe I/O scheduling, and split-lock mitigation toggle.
  - Features `cachyos-handheld` for Steam Deck, ROG Ally, and Legion Go TDP tuning and Gamescope VRR session management.
  - Features `cachyos-snapper-support` for automated Btrfs/Bcachefs pacman transaction snapshotting and Limine/GRUB bootloader entry sync.

## Mechanisms & Data Structures

- **BORE & SchedExt BPF Schedulers**:
  - Dynamically calculates task burstiness scores via sliding window runtime tracking.
  - Grants low-latency timeslice boosts to interactive tasks (GUI input, gaming display pipelines) while maintaining CFS/EEVDF fairness for background tasks.
- **x86-64-v3/v4 ISA Auto-Tuning**:
  - Detects CPU capabilities (AVX2, FMA, BMI2 for v3; AVX-512F/BW/CD/DQ/VL for v4).
  - Routes package manager requests to microarch-optimized repositories and sets compiler preferred vector width (`-mprefer-vector-width=512` vs `256`).
- **Snapper & Bootloader Sync**:
  - Intercepts pacman transactions to create pre/post Btrfs snapshots (`@` and `@home`).
  - Automatically updates Limine/GRUB boot entries to allow direct boot into historical snapshots.
- **Proton / Wine Gaming Runtime**:
  - Configures eSync (eventfd), fSync (futex), and NTSync (kernel NT sync driver) along with LatencyFleX, MangoHud, and VKBasalt overlays.

## SigmaOS Adaptation Decision

- **Adopt**: PR proposal format (`src/distro/cachy_linux_missing_components_v53_pr.rs`) providing modular engines for BORE/sched_ext scheduling, microarch ISA auto-detection, CHWD hardware detection, sysctl performance tuning, Btrfs snapper snapshotting, gaming runtimes, handheld TDP management, and interactive kernel building.
- **Adapt**: Integrated `#![no_std]` / `alloc` compatible PR proposal structures and standalone test harness for zero-dependency validation in SigmaOS.
- **Reject**: Direct copy of Linux C CachyOS patches; clean-room Rust implementation tailored to SigmaOS architecture.

## Verification & PR Deliverables

1. `docs/studies/2026-W42-cachyos-performance-and-gaming-advancements.md` (Research note & ADR).
2. `src/distro/cachy_linux_missing_components_v53_pr.rs` (Implementation & V53 PR Suite).
3. `run_sigma_tests.sh` integration for standalone test execution.
4. `FEATURE_STATUS.toml` updated with test evidence.
