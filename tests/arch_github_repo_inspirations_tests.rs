// SPDX-License-Identifier: MIT
// Integration Test Suite for Arch Linux GitHub Repo Inspirations PR Engine

use arch_inspirations_lib::{
    ArchAlpmDatabaseSyncPrEngine, ArchDevtoolsCleanChrootPrEngine,
    ArchInspirationPrStatus, ArchMkinitcpioHooksPrEngine,
    ArchReflectorMirrorlistRankerPrEngine, ArchSecurityTrackerCvePrEngine,
    ArchinstallAutomatedProfilePrEngine, ArchisoLiveImageBuilderPrEngine,
    SovereignArchLinuxGitHubRepoInspirationsPrEngine,
};

#[test]
fn test_arch_alpm_sync_pr_engine() {
    let engine = ArchAlpmDatabaseSyncPrEngine::new();
    let pr = engine.submit_pr(4001);
    assert_eq!(pr.pr_id, 4001);
    assert_eq!(pr.status, ArchInspirationPrStatus::Merged);
    assert_eq!(pr.upstream_repo, "archlinux/pacman");

    let hooks = engine.match_hooks("PostTransaction", "usr/lib/modules/6.12/vmlinuz");
    assert_eq!(hooks.len(), 1);
    assert!(hooks[0].contains("mkinitcpio"));
}

#[test]
fn test_archinstall_profile_pr_engine() {
    let engine = ArchinstallAutomatedProfilePrEngine::new();
    let pr = engine.submit_pr(4002);
    assert_eq!(pr.pr_id, 4002);
    assert_eq!(pr.upstream_repo, "archlinux/archinstall");

    let json = engine.export_profile_json();
    assert!(json.contains("minimal-hyprland"));
    assert!(json.contains("systemd-boot"));
}

#[test]
fn test_archiso_builder_pr_engine() {
    let engine = ArchisoLiveImageBuilderPrEngine::new();
    let pr = engine.submit_pr(4003);
    assert_eq!(pr.pr_id, 4003);
    assert_eq!(pr.upstream_repo, "archlinux/archiso");
    assert!(engine.is_hybrid_iso);
}

#[test]
fn test_arch_reflector_ranker_pr_engine() {
    let engine = ArchReflectorMirrorlistRankerPrEngine::new();
    let pr = engine.submit_pr(4004);
    assert_eq!(pr.pr_id, 4004);
    assert_eq!(pr.upstream_repo, "archlinux/reflector");

    let ranked = engine.rank_mirrors(30);
    assert_eq!(ranked.len(), 1);
    assert!(ranked[0].contains("rackspace.com"));
}

#[test]
fn test_arch_devtools_chroot_pr_engine() {
    let engine = ArchDevtoolsCleanChrootPrEngine::new();
    let pr = engine.submit_pr(4005);
    assert_eq!(pr.pr_id, 4005);
    assert_eq!(pr.upstream_repo, "archlinux/devtools");

    let cmd = engine.build_cmd("/tmp/pkg");
    assert!(cmd.contains("extra-x86_64-build"));
    assert!(cmd.contains("namcap"));
}

#[test]
fn test_arch_security_tracker_pr_engine() {
    let engine = ArchSecurityTrackerCvePrEngine::new();
    let pr = engine.submit_pr(4006);
    assert_eq!(pr.pr_id, 4006);
    assert_eq!(pr.upstream_repo, "archlinux/arch-security-tracker");

    let advisory = engine.lookup("xz");
    assert!(advisory.is_some());
    assert_eq!(advisory.unwrap().cve_id, "CVE-2024-3094");
}

#[test]
fn test_arch_mkinitcpio_pr_engine() {
    let engine = ArchMkinitcpioHooksPrEngine::new();
    let pr = engine.submit_pr(4007);
    assert_eq!(pr.pr_id, 4007);
    assert_eq!(pr.upstream_repo, "archlinux/mkinitcpio");

    let config = engine.generate_preset_config();
    assert!(config.contains("default_image"));
    assert!(config.contains("HOOKS="));
}

#[test]
fn test_sovereign_arch_github_repo_inspirations_master_engine() {
    let master = SovereignArchLinuxGitHubRepoInspirationsPrEngine::new();
    let prs = master.execute_all_prs();
    assert_eq!(prs.len(), 7);

    for pr in prs {
        assert_eq!(pr.status, ArchInspirationPrStatus::Merged);
        assert!(pr.pr_id >= 3001);
        assert!(!pr.title.is_empty());
        assert!(!pr.upstream_repo.is_empty());
        assert!(!pr.diagnostic_info.is_empty());
    }
}
