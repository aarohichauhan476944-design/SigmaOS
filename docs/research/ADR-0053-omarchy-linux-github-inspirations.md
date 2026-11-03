# Architecture Decision Record (ADR-0053): Omarchy Linux GitHub Repository Inspirations & Component Enhancements

## Context & Motivation
Omarchy Linux is a bleeding-edge, opinionated Arch Linux distribution curated with modern desktop workflows, Hyprland window tiling, Quickshell status applets, GPU terminal emulation (Ghostty/Kitty), Omakase Neovim developer configurations, Herdr multi-agent AI orchestration, dynamic Wallust 16-color theme switching, and GNU Stow dotfiles management.

SigmaOS adopts a PR-only, cadence-based research-and-implementation pipeline. To bring Omarchy's innovations into SigmaOS while maintaining `#![no_std]` core discipline, capability-based security, and zero-allocation execution principles, we model and implement these components as structured Pull Request (PR) proposals.

## Upstream Concepts & Inspirations
1. **Hyprland Dynamic Dwindle Layout Engine:**
   - Dwindle dynamic binary-tree tiling.
   - Smooth gesture-based workspace switching and floating scratchpads.
   - Material Design 3 (MD3) cubic-bezier animation curves.

2. **Quickshell Dynamic Status Bar & Widgets:**
   - Declarative status bar applets (Clock, Network MLO, PipeWire Audio Slider, System Tray, Battery Monitor).
   - Zero-copy notification queue and lockfree event routing.

3. **GPU Terminal Studio (Ghostty & Kitty):**
   - Font rendering profiles with ligatures, fallback font stacks, and shader effects.
   - Low-latency terminal window padding and true-color ANSI support.

4. **Omakase Developer Preset Suite:**
   - Pre-configured Neovim LSP configs, Treesitter AST parsers, and Mason package installer specs.
   - Fast language server lifecycle management.

5. **Herdr Multi-Agent AI Task Router:**
   - Multi-provider AI dispatch (Anthropic, OpenAI, Ollama, DeepSeek).
   - Instant Super+A keyboard shortcut invocation with screen/clipboard context injection.

6. **Wallust 16-Color Dynamic Theme Engine:**
   - Palette extraction from wallpapers with zero-flicker reload across Hyprland, Waybar, Foot, and GTK4 apps.
   - Pre-defined curated themes (Catppuccin Mocha, Tokyo Night, Gruvbox Dark, Nord, Everforest, Kanagawa).

7. **GNU Stow Dotfile Profiles & PQC Encrypted Backups:**
   - Symlink-based modular dotfile management.
   - Quantum-resistant cryptographic signing and backup snapshots.

8. **Ananicy GameMode & Performance Scheduler:**
   - Low-latency SCHED_EXT process prioritization for active games.
   - Dynamic PipeWire audio buffer latency clamping (1.3ms).

## SigmaOS Adaptation & Invariants
- **`#![no_std]` Compatibility:** All logic runs in kernel/microkernel or freestanding userland environments using `alloc` types without depending on `std`.
- **Capability-Based Access:** Privileged operations (CPU governor switching, process priority adjustment, display plane swapping) require valid capability tokens.
- **PR Proposal Format:** Component changes and configuration patches are represented as structured PR records with unified diffs, authors, and verification gates.

## Validation & Testing Strategy
- Standalone unit tests compiled with `rustc --test --edition=2021`.
- Verified execution through `./run_sigma_tests.sh`.
- Status tracked in `FEATURE_STATUS.toml`.
