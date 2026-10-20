# Study: Fedora Linux System Mechanisms & SigmaOS Adaptation Architecture

## Reference System

- **Upstream Project**: Fedora Linux (Fedora Project / Red Hat)
- **Mechanisms Researched**:
  1. **Koji / Bodhi Build and Update Gating**:
     - Koji: Centralized RPM build system utilizing isolated task runners.
     - Bodhi: Release updates management system with karma threshold gating (+3/-3 karma), automated CI test pass requirements, and CVE advisory tracking.
  2. **Pagure & COPR**:
     - Pagure: Git forge with pull request code review workflows and git-notes metadata integration.
     - COPR: Community build system providing custom repository chroot generation.
  3. **Rootless OCI Container Isolation**:
     - Podman / Buildah / Skopeo stack utilizing user namespaces (`subuid`/`subgid`), slirp4netns network virtualization, and SELinux MCS labeling (`container_file_t:s0:c1,c2`).
  4. **Mock Clean Chroot Builder & DNF5 Package Engine**:
     - Mock: Isolated rootfs build sandbox ensuring reproducible RPM compilation.
     - DNF5: High-performance C++ package transaction solver with comps group resolution and repodata caching.
  5. **rpm-ostree & Greenboot Atomic Rollback**:
     - rpm-ostree: Hybrid OSTree image and RPM package layering with ostree commit deployments.
     - Greenboot: Health-check script runner for systemd targets; automatically triggers `rpm-ostree rollback` if boot checks fail.

## SigmaOS Current State & Adaptation

- **Code Location**: `src/compatibility/sovereign_fedora_pr_components_engine.rs`, `src/distro/sovereign_open_source_os_fedora_github_pr_suite.rs`.
- **Adaptation Strategy**:
  - Implement zero-dependency, `#![no_std]` compliant PR deployer structures in Rust representing each Fedora mechanism.
  - Track pull request records with FNV-1a patch digests and merge status verifications.
  - Expose PR deployment verification methods to validate complete Fedora component coverage.

## Verification & PR Format Requirements

- **PR Format**: Every mechanism integration is modeled and validated as a pull request specification (`FedoraPrPullRequestSpec`).
- **Tests**: Standalone `rustc --test` verification and integration in `./run_sigma_tests.sh`.

## Follow-up Action Items

- [x] Create research study document `docs/studies/2026-W42-fedora-mechanisms-and-pr-architecture.md`
- [ ] Enhance `src/compatibility/sovereign_fedora_pr_components_engine.rs` with comprehensive Fedora PR deployers and tests
- [ ] Update `run_sigma_tests.sh` to run the updated Fedora PR components test suite
- [ ] Update status matrices (`CAPABILITY_MATRIX.toml` and `FEATURE_STATUS.toml`)
