# Study: EndeavourOS Component Parity & GitHub Repository Inspirations

## Reference Systems

- **EndeavourOS (`endeavouros-team` GitHub Repositories)**:
  - `eos-welcome`: Welcome application providing post-installation setup, mirror ranking, maintenance quick-tasks, desktop switcher, and software quickstart installer.
  - `eos-rankmirrors` & `reflector-simple`: Automated pacman mirror selection tool ranking mirrors by latency and speed with country/protocol filtering.
  - `eos-log-tool`: System diagnostic uploader aggregating hardware (`inxi`), `journalctl`, `Xorg.0.log`, `dmesg`, and `pacman.log` with privacy data redaction.
  - `eos-update`: Unified system updater orchestrating pacman, AUR helpers (yay/paru), and flatpak with breaking-news feed notifications and reboot necessity detection.
  - `akm` (Akm Kernel Manager): Kernel manager handling mainline, LTS, Zen, Hardened, and Real-Time (RT) kernel installations, DKMS module rebuilding, and bootloader configuration.
  - `calamares` (EndeavourOS Profile): Offline and Netinstall Calamares installer configurations with Btrfs subvolume layout (`@`, `@home`, `@cache`, `@log`, `@swap`).

## Mechanism & Adaptations

- **Problem**: Managing a user-friendly, terminal-centric Arch-based distribution environment with automated maintenance, privacy-preserving diagnostics, multi-kernel management, and reproducible installer configurations.
- **Data Structures**:
  - `CalamaresConfig` & `BtrfsSubvolumeSpec` for automated Btrfs subvolume hierarchy generation and netinstall package selection.
  - `PacmanMirror` & `ReflectorMirrorManager` for protocol-aware latency ranking and mirrorlist output formatting.
  - `EosLogRedactor` & `EosLogTool` for diagnostic log aggregation and regex-based redaction of IPv4/v6 addresses, MAC addresses, home usernames, and passwords.
  - `EosUpdateEngine` & `EosNewsArticle` for news advisory checks, multi-package manager update execution, and reboot required flags.
  - `AkmKernelManager` & `DkmsModuleSpec` for kernel flavor switching, DKMS module recompilation, and GRUB/systemd-boot entry generation.

## SigmaOS Core Implementation

- **Code Location**: `src/distro/endeavour_os.rs`.
- **Integration**: `src/distro/mod.rs` re-exports all EndeavourOS parity tools and `SovereignEndeavourOsGitHubRepoPrEngine` PR generator.
- **Verification**: `tests/os_components_tests.rs` (`test_endeavour_os_parity`) and `run_sigma_tests.sh`.

## Decision

- **Adopt**: Terminal-centric maintenance, log redaction, mirror latency scoring, unified updating, and AKM multi-kernel management from EndeavourOS.
- **Adapt**: Implement `#![no_std]` safe Rust structures with `alloc` support for core microkernel and userland deployment.
- **Reject**: Unsandboxed bash script execution or unredacted diagnostic log pastes.

## Follow-up PR Proposals

- [x] feat(endeavour_os): add EndeavourOS Welcome App with quickstart installer & desktop switcher
- [x] feat(endeavour_os): add Reflector mirror ranker with protocol filtering
- [x] feat(endeavour_os): add EOS Log Tool with privacy data redactor
- [x] feat(endeavour_os): add EOS Update Engine with news feed & reboot detection
- [x] feat(endeavour_os): add AKM Kernel Manager with DKMS module rebuilder
- [x] feat(endeavour_os): add Calamares Btrfs subvolume & netinstall config engine
- [x] feat(endeavour_os): add EndeavourOS GitHub Repo PR Proposal Engine
