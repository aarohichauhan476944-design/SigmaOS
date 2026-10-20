# Study: EndeavourOS Inspirations, Tools and Architectural Parity

## Reference System

- **EndeavourOS (`endeavouros-team`)**:
  - Terminal-centric, user-empowering Arch Linux derivative.
  - **Calamares Installer Integration**: Custom Netinstall profiles, desktop selection (XFCE, KDE Plasma, GNOME, i3, Sway, Hyprland), dual-boot, and Btrfs subvolume layouts with LUKS option.
  - **Reflector Mirror Ranking**: Automated sorting of pacman mirrorlists based on latency, download speed, protocol, and country filtering.
  - **EOS Welcome Application**: Desktop/GUI quickstart utility offering system maintenance shortcuts (mirror ranker, system update, package cache cleanup, keyring refresh, diagnostic log viewer) and launcher for distro management tools.
  - **EOS Log Tool**: One-click diagnostic aggregator and pastebin uploader (e.g. 0x0.st / pastebin) gathering `journalctl`, hardware specs, Xorg/Wayland traces, and boot logs for quick support.
  - **AKM (Arch Kernel Manager)**: GUI & CLI tool for inspecting, installing, removing, and setting default boot kernel flavors (`linux`, `linux-lts`, `linux-zen`, `linux-hardened`).
  - **Yay / Paru Helpers**: Arch User Repository (AUR) helpers with sandboxed building and dependency resolution.

## Problem Statement

SigmaOS aims to offer an intuitive user experience for installation, system maintenance, kernel switching, and package management while remaining sovereign and fail-closed. Taking inspiration from EndeavourOS, SigmaOS requires robust installer config generation, intelligent mirror selection, automated diagnostic reporting, multi-kernel management, and AUR package building abstractions.

## Architectural Adaptations in SigmaOS

### 1. Calamares Netinstall & Disk Layouts
- **Config**: Extends `CalamaresConfig` with Netinstall DE package selection matrices (GUI vs headless, tiling vs traditional DE), LUKS encryption toggles, and subvolume partitioning configurations.
- **Installer Engine**: Executes pre-flight validation (username, hostname, disk availability), disk layout mapping, and returns structured installation summaries.

### 2. Reflector Automated Mirror Sorting Engine
- **Engine**: `ReflectorMirrorManager` parses available mirror candidates, filters by country and HTTPS support, ranks by lowest latency and highest bandwidth speed (`speed_kbps`), and exports top-N ranked lists for system configuration.

### 3. EOS Welcome & Quickstart App Suite
- **Engine**: `EosWelcomeApp` handles automated maintenance tasks (`UpdateMirrors`, `UpdateSystem`, `CleanPackages`, `FixKeys`, `ViewLogs`) and launches default distro utilities (Reflector GUI, AKM, System Info, EOS Apps).

### 4. EOS Log Tool & Diagnostic Aggregator
- **Engine**: `EosLogTool` aggregates hardware topology, system boot logs, journal entries, and returns secure diagnostic upload links.

### 5. AKM Kernel Manager & Yay/Paru AUR Helpers
- **AKM Engine**: `AkmKernelManager` maintains installed kernel flavors (`LinuxStable`, `LinuxLts`, `LinuxZen`, `LinuxHardened`), checks bootloader configuration, and validates boot parameter updates.
- **Yay/Paru Engine**: `YayParuHelper` registers AUR package specs, validates PKGBUILD sources, builds packages in isolated chroots, and completes system installation.

## SigmaOS Current State & Verification

- **Code Location**: `src/distro/endeavour_os.rs`.
- **Testing**: Dedicated unit and integration tests run via `rustc --test src/distro/endeavour_os.rs` and `./run_sigma_tests.sh`.
- **Evidence**: Verified clean compilation and 100% test pass rate across installer, reflector, welcome app, log tool, kernel manager, and yay/paru helper.

## Follow-up Action Items
- [x] docs(research): create EndeavourOS inspiration and architecture study document
- [x] feat(distro): enhance EndeavourOS installer, mirror manager, welcome app, log tool, AKM, and yay/paru helper
- [x] test(distro): add EndeavourOS native test suite runner to `./run_sigma_tests.sh`
