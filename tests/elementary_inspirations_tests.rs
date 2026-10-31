// SPDX-License-Identifier: MIT
// Integration Tests: Elementary OS GitHub Repo Inspirations for SigmaOS
// (`tests/elementary_inspirations_tests.rs`)

#[path = "../src/distro/elementary_github_repo_inspirations_pr.rs"]
mod elementary;

use elementary::*;

#[test]
fn test_elementary_granite_widget_toolkit_integration() {
    let mut granite = GraniteWidgetToolkitEngine::new();
    assert_eq!(granite.color_scheme, ColorSchemePreference::FollowSystem);

    granite.set_color_scheme(ColorSchemePreference::ForceDark);
    assert!(granite.is_dark_mode_active(false));
    assert!(granite.is_dark_mode_active(true));

    granite.set_color_scheme(ColorSchemePreference::ForceLight);
    assert!(!granite.is_dark_mode_active(true));
    assert!(!granite.is_dark_mode_active(false));

    granite.set_color_scheme(ColorSchemePreference::FollowSystem);
    assert!(granite.is_dark_mode_active(true));
    assert!(!granite.is_dark_mode_active(false));

    // Dynamic Notebook tab management
    let t0 = granite.add_tab("Home");
    let t1 = granite.add_tab("Documents");
    let t2 = granite.add_tab("Settings");
    assert_eq!(granite.open_tabs.len(), 3);
    assert_eq!(t0, 0);
    assert_eq!(t1, 1);
    assert_eq!(t2, 2);
    assert_eq!(granite.active_tab_index, 2);

    assert!(granite.close_tab(1)); // Close Documents
    assert_eq!(granite.open_tabs.len(), 2);
    assert_eq!(granite.open_tabs[0], "Home");
    assert_eq!(granite.open_tabs[1], "Settings");

    // Toast notification flow
    let toast_id = granite.post_toast("Download Complete", "kernel_v51.iso finished downloading", Some("Show in Files"), 4000);
    assert_eq!(toast_id, 1);
    assert_eq!(granite.toasts.len(), 1);
    assert!(!granite.toasts[0].dismissed);

    assert!(granite.dismiss_toast(toast_id));
    assert!(granite.toasts[0].dismissed);

    // High contrast
    granite.set_high_contrast(true);
    assert!(granite.high_contrast_enabled);
}

#[test]
fn test_elementary_gala_compositor_integration() {
    let mut gala = GalaWaylandCompositorEngine::new(6);
    assert_eq!(gala.workspaces_count, 6);
    assert_eq!(gala.active_workspace, 0);

    // Switch workspace
    assert!(gala.switch_workspace(3));
    assert_eq!(gala.active_workspace, 3);
    assert!(!gala.switch_workspace(10)); // Out of bounds

    // Window management & tiling snapping
    gala.create_window(101, "Files", "io.elementary.files");
    gala.create_window(102, "Terminal", "io.elementary.terminal");
    assert_eq!(gala.windows.len(), 2);
    assert!(gala.windows[1].is_focused);
    assert!(!gala.windows[0].is_focused);

    assert!(gala.snap_window(101, WindowSnapPosition::LeftHalf));
    assert!(gala.snap_window(102, WindowSnapPosition::RightHalf));

    // Picture-in-Picture mode
    assert!(gala.toggle_pip(102));
    assert!(gala.windows[1].is_pip);

    // Filter active workspace windows
    gala.switch_workspace(0);
    let visible_wins = gala.get_active_workspace_windows();
    // 102 is PiP so visible across all workspaces, 101 is on workspace 3 so not visible
    assert_eq!(visible_wins.len(), 1);
    assert_eq!(visible_wins[0].window_id, 102);
}

#[test]
fn test_elementary_wingpanel_and_plank_integration() {
    let mut wingpanel = WingpanelIndicatorBarEngine::new();
    assert_eq!(wingpanel.indicators.len(), 4);
    assert!(wingpanel.indicators.iter().any(|i| i.id == "wingpanel-indicator-datetime"));
    assert!(wingpanel.indicators.iter().any(|i| i.id == "wingpanel-indicator-power"));

    // Popover mutual exclusion
    assert!(wingpanel.toggle_popover("wingpanel-indicator-network"));
    let net_ind = wingpanel.indicators.iter().find(|i| i.id == "wingpanel-indicator-network").unwrap();
    assert!(net_ind.popover_open);

    assert!(wingpanel.toggle_popover("wingpanel-indicator-sound"));
    let net_ind_after = wingpanel.indicators.iter().find(|i| i.id == "wingpanel-indicator-network").unwrap();
    let sound_ind = wingpanel.indicators.iter().find(|i| i.id == "wingpanel-indicator-sound").unwrap();
    assert!(!net_ind_after.popover_open);
    assert!(sound_ind.popover_open);

    // Notification counter
    wingpanel.set_notifications_count(5);
    assert_eq!(wingpanel.notifications_count, 5);

    // Plank dock launcher
    let mut plank = PlankDockLauncherEngine::new();
    assert!(!plank.is_visible()); // Autohide active, unhovered
    plank.is_hovered = true;
    assert!(plank.is_visible());

    // Launch tracking
    plank.report_app_launch("io.elementary.files");
    let files_item = plank.items.iter().find(|i| i.app_id == "io.elementary.files").unwrap();
    assert_eq!(files_item.running_instances, 1);

    // Transient app launch
    plank.report_app_launch("org.gnome.Calculator");
    assert!(plank.items.iter().any(|i| i.app_id == "org.gnome.Calculator"));

    plank.report_app_exit("org.gnome.Calculator");
    assert!(!plank.items.iter().any(|i| i.app_id == "org.gnome.Calculator")); // Unpinned transient purged
}

#[test]
fn test_elementary_switchboard_and_contractor_integration() {
    let mut switchboard = SwitchboardControlCenterEngine::new();
    assert_eq!(switchboard.plugs.len(), 5);

    // Search functionality
    let power_plugs = switchboard.search("resolution");
    assert_eq!(power_plugs.len(), 1);
    assert_eq!(power_plugs[0].plug_id, "switchboard-plug-displays");

    switchboard.register_plug(
        "switchboard-plug-bluetooth",
        "Bluetooth",
        "Hardware",
        "preferences-system-bluetooth",
        &["wireless", "headset", "mouse", "keyboard"],
    );
    assert_eq!(switchboard.search("headset").len(), 1);

    // Contractor MIME-type service broker
    let mut contractor = ContractorServiceBrokerEngine::new();
    let text_contracts = contractor.find_contracts_for_mime("text/plain");
    assert!(text_contracts.iter().any(|c| c.contract_id == "contractor-share-email"));

    contractor.register_contract(
        "contractor-ocr-extract",
        "Extract Text with OCR",
        &["image/png", "image/jpeg", "application/pdf"],
        "sigma-tesseract-ocr %f",
    );
    let ocr_matches = contractor.find_contracts_for_mime("application/pdf");
    assert!(ocr_matches.iter().any(|c| c.contract_id == "contractor-ocr-extract"));
}

#[test]
fn test_elementary_appcenter_pwyw_integration() {
    let mut appcenter = AppCenterPwywStoreEngine::new();
    assert_eq!(appcenter.list_curated().len(), 3);

    // Purchase / contribution workflow
    assert!(appcenter.purchase_and_install("io.elementary.music", 0).is_ok()); // Free install
    let music_pkg = appcenter.packages.iter().find(|p| p.app_id == "io.elementary.music").unwrap();
    assert!(music_pkg.is_installed);
    assert_eq!(music_pkg.paid_amount_cents, 0);

    assert!(appcenter.purchase_and_install("com.github.alainm23.planner", 1000).is_ok()); // $10.00 contribution
    let planner_pkg = appcenter.packages.iter().find(|p| p.app_id == "com.github.alainm23.planner").unwrap();
    assert!(planner_pkg.is_installed);
    assert_eq!(planner_pkg.paid_amount_cents, 1000);
    assert_eq!(appcenter.total_contributions_cents, 1000);

    // Missing app
    assert!(appcenter.purchase_and_install("nonexistent.app", 500).is_err());
}

#[test]
fn test_pantheon_apps_and_onboarding_integration() {
    // Session governor
    let mut session = PantheonSessionGovernorEngine::new("developer");
    assert_eq!(session.session_state, SessionState::Active);
    session.set_sleep_inhibit(true);
    assert!(!session.trigger_idle_action()); // Sleep inhibited, doesn't lock
    session.set_sleep_inhibit(false);
    assert!(session.trigger_idle_action()); // Locks now
    assert_eq!(session.session_state, SessionState::Locked);
    assert!(session.unlock(true));

    // Files manager
    let mut files = PantheonFilesManagerEngine::new("/home/developer");
    files.set_view_mode(FileViewMode::GridView);
    assert_eq!(files.view_mode, FileViewMode::GridView);
    files.add_file_entry(".gitignore", false, 32);
    files.add_file_entry("src", true, 4096);
    assert_eq!(files.visible_entries().len(), 1);
    assert!(files.toggle_hidden_files());
    assert_eq!(files.visible_entries().len(), 2);

    // Terminal
    let mut term = PantheonTerminalEngine::new();
    assert!(term.should_warn_multiline_paste("sudo apt update\nsudo apt upgrade"));
    assert!(!term.should_warn_multiline_paste("cargo check"));
    term.spawn_tab("Rust Compiler Output");
    assert_eq!(term.tabs.len(), 2);
    term.execute_command("cargo test");
    assert_eq!(term.current_command_running.as_deref(), Some("cargo test"));
    term.finish_command();
    assert_eq!(term.current_command_running, None);

    // Code Editor
    let mut editor = PantheonCodeEditorEngine::new("main.rs");
    editor.load_content("fn main() {\n    println!(\"Hello Pantheon!\");\n}\n");
    editor.add_diagnostic(2, "unused variable", false);
    editor.add_diagnostic(3, "syntax error", true);
    assert_eq!(editor.diagnostics.len(), 2);
    assert_eq!(editor.error_count(), 1);

    // Initial Setup
    let mut setup = ElementaryInitialSetupEngine::new();
    assert!(!setup.onboarding_completed);
    setup.configure_privacy(false, false);
    setup.complete_setup();
    assert!(setup.onboarding_completed);

    // Captive Portal
    let mut captive = ElementaryCaptivePortalEngine::new();
    assert!(captive.evaluate_response(511)); // Network Authentication Required
    assert_eq!(captive.portal_detected, true);
    assert!(!captive.evaluate_response(204));
    assert_eq!(captive.portal_detected, false);

    // Shortcut Overlay
    let mut overlay = ElementaryShortcutOverlayEngine::new();
    assert_eq!(overlay.shortcuts.len(), 5);
    assert!(overlay.toggle_overlay());
    assert!(overlay.overlay_visible);
}

#[test]
fn test_elementary_github_repo_inspirations_master_pr_suite() {
    let mut master_suite = ElementaryGithubRepoInspirationsPrSuite::new();
    assert!(master_suite.verify_health_check());
    assert_eq!(master_suite.matrix.proposals.len(), 14);

    for prop in master_suite.matrix.proposals.values() {
        assert_eq!(prop.status, PrStatus::CapabilityVerified);
        assert!(prop.pqc_signature.is_some());
        assert!(!prop.description.is_empty());
    }

    assert!(master_suite.audit_all_prs_merged());
    for prop in master_suite.matrix.proposals.values() {
        assert_eq!(prop.status, PrStatus::Merged);
    }
}
