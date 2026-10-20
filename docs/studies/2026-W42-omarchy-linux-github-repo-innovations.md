# Upstream Study & Architectural Decision Record (ADR)
## Omarchy Linux GitHub Repository Innovations and SigmaOS Parity Engine

- **Study ID:** `2026-W42`
- **Topic:** Omarchy Linux Desktop Innovations & Omakase Subsystem Parity
- **Upstream Reference:** Omarchy Linux (Hyprland + Quickshell + Stow Dotfiles + Omakase CLI Workflow)
- **Status:** Approved / ADR Implemented

---

## 1. Upstream Study Context
Omarchy Linux is an Arch Linux derivative focused on an out-of-the-box ("Omakase") tiling desktop experience using Hyprland, Quickshell widgets, GNU Stow dotfile management, and automated workflow scripts. Core mechanisms include:
1. **Dynamic Theme Switching:** Synchronized palette switching (Catppuccin, TokyoNight, Gruvbox, Nord, Everforest, Kanagawa, RosePine, Dracula) across Hyprland window borders, Ghostty terminal, QuickShell applets, and Neovim.
2. **Quickshell Applet Palette:** QML/native status bar applets for clock, workspace indicator, audio slider, MLO network monitoring, system tray, and multi-agent AI assistant toggles.
3. **Stow Dotfile Profiles & PKGBUILD Management:** Managed GNU Stow profile directories coupled with custom Arch AUR PKGBUILD generation.
4. **Agentic CLI & PR Workflow:** Unified `sigomarchy` CLI controller and PR submission/gating engine.

---

## 2. SigmaOS Adaptation Decision
SigmaOS integrates these features through a zero-dependency, `#![no_std]` capable Rust architecture under `src/distro/sovereign_omarchy_github_repo_parity.rs`.

Key abstractions:
- `OmarchyThemeName`: Expanded enum supporting `CatppuccinMocha`, `TokyoNight`, `GruvboxDark`, `Nord`, `Everforest`, `Kanagawa`, `RosePine`, and `Dracula`.
- `OmarchyHyprlandThemeSyncEngine`: Generates dynamic configuration snippets for Hyprland borders, Ghostty terminals, and desktop applications.
- `OmarchyQuickshellWidgetPalette`: Manages widget status and desktop notifications.
- `OmarchyDotfilePkgbuildInstaller`: Handles Stow profile tracking and PKGBUILD generation.
- `OmarchyOmakaseEcosystemMasterSuite`: Orchestrates theme application, applets, and GitHub repo parity verification.

---

## 3. Verification & Safety Assumptions
- Pure Rust memory safety (zero unvalidated raw pointer dereferences).
- Standalone test runner compatibility (`rustc --test`).
- Pre-commit gating and automated PR diff generation.
