# Study: Gentoo Linux Portage, OpenRC, and Hardened Paradigms (2026-W42)

## Reference Systems

- **Gentoo Linux (`gentoo/portage`, `gentoo/openrc`, `gentoo/gentoo`)**:
  - **Portage / EAPI 8**: Source-based package management using `ebuild` specs with fine-grained USE flags, SLOT/SUBSLOT tracking, and REQUIRED_USE boolean algebra (`USE="ssl -gtk X"`).
  - **Package Config Rules**: Fine-grained per-package configuration overriding via `/etc/portage/package.use`, `/etc/portage/package.accept_keywords`, `/etc/portage/package.mask`, and `/etc/portage/package.unmask`.
  - **Manifest Integrity**: Distfile, ebuild, and auxiliary file integrity validation using SHA-512 and BLAKE2b digests.
  - **System Target Selection (`eselect`)**: Modular system profile and active toolchain/kernel/python target switching.
  - **Protected Config Merging (`etc-update` / `dispatch-conf`)**: 3-way line diff and merge engine for `._cfg0000_*` protected configuration updates.
  - **Overlay Management (`layman` / `eselect-repository`)**: Third-party ebuild overlay synchronization with `repos.conf` routing and GPG signature verification.
  - **OpenRC Supervisor**: Init and service supervision with dynamic runlevels (`sysinit`, `boot`, `default`) and dependency resolution (`rc-service`).
  - **Gentoo Hardened / PaX**: Security profile enforcing W^X (`PAGEEXEC`, `MPROTECT`), GCC Stack Smashing Protector (SSP), and `FORTIFY_SOURCE=3`.
  - **Catalyst & Distcc**: Automated Stage 1-3 tarball release builder and distcc parallel compilation cluster.

## Mechanism

- **Problem**: Providing fine-grained source compilation controls, zero-overhead modular service initialization, and robust security hardening while maintaining reproducible package builds across custom kernel configurations.
- **Data Structures**:
  - `GentooUseFlagManager`: Hash maps and bitsets for global/per-package USE flag status.
  - `GentooOpenRcRunlevelSupervisor`: Service dependency graph resolving `needs`, `uses`, and `provides` directives.
  - `GentooManifestChecksumVerifier`: Recorded file digests mapped against file size and SHA-512 / BLAKE2b calculations.
  - `GentooSlotSubslotConflictEngine`: Version slot collision detection and rebuild triggers.

## SigmaOS Current State

- **Code**: Basic Gentoo USE flag manager in `src/sigpkg/gentoo_use_flags.rs` and Portage dependency solver in `src/sigpkg/portage.rs`.
- **Gaps**: Lack of a complete, unified PR deployment engine covering Gentoo's GitHub repository paradigms (EAPI 8 phase lifecycle, `package.use`/`package.accept_keywords` rules parser, `etc-update` protected config merger, PaX security profile, OpenRC supervisor, and Catalyst stage builder).

## Decision

- **Adopt**: Portage EAPI 8 phase lifecycle, USE flag conditional dependency algebra, OpenRC runlevels, and `eselect` target switching.
- **Adapt**: Gentoo Hardened PaX memory protection controls and `etc-update` 3-way protected config merger into `#![no_std]` core Rust primitives for SigmaOS.
- **Reject**: Direct bash ebuild execution in kernel space; replace with safe, zero-allocation Rust specifications.

## Follow-up PRs

- [x] `feat(distro): add Gentoo Linux GitHub repo inspirations and PR deployment engine`
- [x] `test(distro): add Gentoo Portage, OpenRC, and PaX security profile test suite`
