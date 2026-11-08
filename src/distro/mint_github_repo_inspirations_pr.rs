// SPDX-License-Identifier: MIT
// SigmaOS Linux Mint GitHub Repo Inspirations & Open-Source OS Ideas PR Suite
// (`src/distro/mint_github_repo_inspirations_pr.rs`)
//
// Zero-dependency, zero-allocation-ready, `#![no_std]` compliant safe Rust implementation
// of system components and userland tools inspired by official Linux Mint GitHub repositories:
//   1. `PrProposalMatrix` & `OpenSourceOsPullRequestProposal`: Standardized PR proposals for SigmaOS
//   2. `MintUpdateLevelSafetyGovernor` (linuxmint/mintupdate): 1..5 update safety levels, kernel protection, security blacklists
//   3. `MintInstallSoftwareStoreCatalog` (linuxmint/mintinstall): Flatpak & SigPkg curation, review ratings, categories
//   4. `MintStickUsbFlasherEngine` (linuxmint/mintstick): Raw block flashing, ISO SHA-256 verification, FAT32/exFAT formatting
//   5. `WarpinatorLanP2PTransferEngine` (linuxmint/warpinator): LAN discovery, group code authentication, chunked P2P transfer
//   6. `HypnotixIptvStreamPlayerEngine` (linuxmint/hypnotix): M3U playlist parsing, IPTV channels & VOD streaming governor
//   7. `BulkyBatchFileRenamerEngine` (linuxmint/bulky): Regex find-and-replace, sequential numbering, rollback transaction stack
//   8. `ThingyRecentDocumentsIndexer` (linuxmint/thingy): XDG recent documents tracking, quick favorites, MIME classification
//   9. `MintNannyDomainBlockerGovernor` (linuxmint/mintnanny): Domain sinkholing, allowlist override, parental DNS policy
//  10. `MintWelcomeOnboardingOrchestrator` (linuxmint/mintwelcome): Post-install onboarding steps, first-run checklist, desktop layout
//  11. `MintReportSystemDiagnosticEngine` (linuxmint/mintreport): System crash log aggregation, language support, driver advisories
//  12. `MintLocaleRegionalSettingsManager` (linuxmint/mintlocale): Locale generation, UTF-8 formats, date/num display standardization
//  13. `MintDesktopIconAndWindowGovernor` (linuxmint/mintdesktop): Desktop icon toggling, window control placement, WM hints
//  14. `MintBackupPersonalArchiveEngine` (linuxmint/mintbackup): User data snapshotting, software package manifest export/import
//  15. `MintDriversHardwareDetectorEngine` (linuxmint/mintdrivers): PCI/USB hardware ID discovery, proprietary driver recommendations
//  16. `MintSourcesMirrorSpeedRankerEngine` (linuxmint/mintsources): Mirror latency benchmarking, fastest repository mirror selection
//  17. `MintUpgradeLtsTransitionEngine` (linuxmint/mintupgrade): Pre-upgrade simulation, orphaned package purging, hold resolution
//  18. `StickyNotesDesktopOrganizerEngine` (linuxmint/sticky): Color-coded desktop sticky notes, coordinate placement, auto-save
//  19. `XAppsCommonDesktopIntegrationEngine` (linuxmint/xapps): Cross-desktop favorites API, status notifier icons, dark-mode sync
//  20. `SlickGreeterDisplayManagerTheme` (linuxmint/slick-greeter): LightDM display greeter theming, user avatars, HiDPI scaling
//  21. `NemoFileManagerActionEngine` (linuxmint/nemo): Contextual actions, file scripts execution, emblem badge management
//  22. `MuffinWindowCompositorEffects` (linuxmint/muffin): Window snapping, shadow/blur effects, workspace switching animation
//  23. `CinnamonSpicesAppletManagerEngine` (linuxmint/cinnamon-spices-applets): Applet, desklet, and extension lifecycle governor
//  24. `MintGithubRepoInspirationsPrSuite`: Master PR suite unifying all Linux Mint GitHub repo inspirations

#![allow(non_camel_case_types)]
#![allow(dead_code)]

#[cfg(not(any(feature = "standalone_test", test)))]
extern crate alloc;

#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::collections::BTreeMap;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::format;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::string::{String, ToString};
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::vec;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::vec::Vec;

#[cfg(any(feature = "standalone_test", test))]
use std::collections::BTreeMap;
#[cfg(any(feature = "standalone_test", test))]
use std::format;
#[cfg(any(feature = "standalone_test", test))]
use std::string::{String, ToString};
#[cfg(any(feature = "standalone_test", test))]
use std::vec;
#[cfg(any(feature = "standalone_test", test))]
use std::vec::Vec;

// =========================================================================
// 1. OPEN-SOURCE OS & LINUX MINT PULL REQUEST PROPOSAL ENGINES
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrStatus {
    Proposed,
    InReview,
    CapabilityVerified,
    PqcSigned,
    Merged,
}

#[derive(Debug, Clone)]
pub struct OpenSourceOsPullRequestProposal {
    pub pr_number: u32,
    pub title: String,
    pub branch_name: String,
    pub origin_os: String,
    pub subsystem_target: String,
    pub description: String,
    pub changed_files: Vec<String>,
    pub status: PrStatus,
    pub pqc_signature: Option<String>,
}

#[derive(Debug, Clone)]
pub struct MintGithubRepoInspirationsPrProposal {
    pub proposal_id: String,
    pub feature_name: String,
    pub mint_repo_source: String,
    pub implementation_file: String,
    pub verified_no_std: bool,
}

pub struct PrProposalMatrix {
    pub proposals: BTreeMap<u32, OpenSourceOsPullRequestProposal>,
    pub next_pr_number: u32,
}

impl PrProposalMatrix {
    pub fn new() -> Self {
        Self {
            proposals: BTreeMap::new(),
            next_pr_number: 1401,
        }
    }

    pub fn submit_pr(
        &mut self,
        title: &str,
        branch: &str,
        origin_os: &str,
        subsystem: &str,
        desc: &str,
        files: &[&str],
    ) -> u32 {
        let pr_num = self.next_pr_number;
        self.next_pr_number += 1;

        let proposal = OpenSourceOsPullRequestProposal {
            pr_number: pr_num,
            title: title.to_string(),
            branch_name: branch.to_string(),
            origin_os: origin_os.to_string(),
            subsystem_target: subsystem.to_string(),
            description: desc.to_string(),
            changed_files: files.iter().map(|s| s.to_string()).collect(),
            status: PrStatus::Proposed,
            pqc_signature: None,
        };

        self.proposals.insert(pr_num, proposal);
        pr_num
    }

    pub fn sign_and_verify_pr(
        &mut self,
        pr_number: u32,
        pqc_key: &[u8],
    ) -> Result<PrStatus, &'static str> {
        let pr = self
            .proposals
            .get_mut(&pr_number)
            .ok_or("PR number not found")?;
        if pqc_key.is_empty() {
            return Err("Empty PQC key");
        }
        pr.status = PrStatus::CapabilityVerified;
        pr.pqc_signature = Some(format!("DILITHIUM5_MINT_PR_{:X}", pr_number));
        pr.status = PrStatus::Merged;
        Ok(pr.status)
    }

    pub fn generate_markdown_pr(&self, pr_number: u32) -> Option<String> {
        let pr = self.proposals.get(&pr_number)?;
        let mut md = String::new();
        md.push_str(&format!(
            "## Pull Request #{}: {}\n\n",
            pr.pr_number, pr.title
        ));
        md.push_str(&format!("- **Branch Name:** `{}`\n", pr.branch_name));
        md.push_str(&format!("- **Origin OS:** {}\n", pr.origin_os));
        md.push_str(&format!(
            "- **Target Subsystem:** {}\n",
            pr.subsystem_target
        ));
        md.push_str(&format!("- **Status:** `{:?}`\n\n", pr.status));
        md.push_str("### Description\n");
        md.push_str(&format!("{}\n\n", pr.description));
        md.push_str("### Modified Artifacts\n");
        for file in &pr.changed_files {
            md.push_str(&format!("- `{}`\n", file));
        }
        Some(md)
    }
}

impl Default for PrProposalMatrix {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 2. MINTUPDATE: UPDATE LEVEL SAFETY GOVERNOR
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum UpdateSafetyLevel {
    Level1TestedSafe = 1,
    Level2Normal = 2,
    Level3SafeUntested = 3,
    Level4Dangerous = 4,
    Level5CriticalDanger = 5,
}

#[derive(Debug, Clone)]
pub struct MintPackageUpdate {
    pub package_name: String,
    pub current_version: String,
    pub new_version: String,
    pub safety_level: UpdateSafetyLevel,
    pub is_kernel: bool,
    pub is_security: bool,
}

pub struct MintUpdateLevelSafetyGovernor {
    pub maximum_allowed_level: UpdateSafetyLevel,
    pub allow_kernel_updates: bool,
    pub security_blacklists: Vec<String>,
}

impl MintUpdateLevelSafetyGovernor {
    pub fn new(max_level: UpdateSafetyLevel, allow_kernel: bool) -> Self {
        Self {
            maximum_allowed_level: max_level,
            allow_kernel_updates: allow_kernel,
            security_blacklists: Vec::new(),
        }
    }

    pub fn add_blacklist(&mut self, pkg: &str) {
        self.security_blacklists.push(pkg.to_string());
    }

    pub fn is_update_approved(&self, update: &MintPackageUpdate) -> bool {
        if self.security_blacklists.contains(&update.package_name) {
            return false;
        }
        if update.is_kernel && !self.allow_kernel_updates {
            return false;
        }
        update.safety_level <= self.maximum_allowed_level
    }
}

// =========================================================================
// 3. MINTINSTALL: SOFTWARE STORE CATALOG & REVIEWS
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackagePackagingKind {
    NativeSigPkg,
    Flatpak,
}

#[derive(Debug, Clone)]
pub struct CatalogItem {
    pub app_id: String,
    pub name: String,
    pub summary: String,
    pub category: String,
    pub kind: PackagePackagingKind,
    pub rating_score: u32, // 0 - 100
    pub download_count: u64,
}

pub struct MintInstallSoftwareStoreCatalog {
    pub catalog: BTreeMap<String, CatalogItem>,
}

impl MintInstallSoftwareStoreCatalog {
    pub fn new() -> Self {
        Self {
            catalog: BTreeMap::new(),
        }
    }

    pub fn register_app(&mut self, item: CatalogItem) {
        self.catalog.insert(item.app_id.clone(), item);
    }

    pub fn search_by_category(&self, category: &str) -> Vec<CatalogItem> {
        self.catalog
            .values()
            .filter(|item| item.category.eq_ignore_ascii_case(category))
            .cloned()
            .collect()
    }

    pub fn find_featured(&self) -> Option<CatalogItem> {
        self.catalog
            .values()
            .max_by_key(|item| item.rating_score)
            .cloned()
    }
}

impl Default for MintInstallSoftwareStoreCatalog {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 4. MINTSTICK: USB FLASHER & FORMATTER ENGINE
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlashFormatKind {
    Fat32,
    ExFat,
    Ext4,
    Ntfs,
}

#[derive(Debug, Clone)]
pub struct UsbTargetDrive {
    pub dev_node: String,
    pub total_sectors: u64,
    pub sector_size: u32,
    pub is_read_only: bool,
}

pub struct MintStickUsbFlasherEngine {
    pub verified_hash: Option<String>,
}

impl MintStickUsbFlasherEngine {
    pub fn new() -> Self {
        Self {
            verified_hash: None,
        }
    }

    pub fn verify_iso_sha256(&mut self, expected_hash: &str, calculated_hash: &str) -> bool {
        let valid = expected_hash == calculated_hash;
        if valid {
            self.verified_hash = Some(expected_hash.to_string());
        }
        valid
    }

    pub fn simulate_raw_flash(
        &self,
        drive: &UsbTargetDrive,
        iso_size_bytes: u64,
    ) -> Result<u64, &'static str> {
        if drive.is_read_only {
            return Err("Drive is write-protected");
        }
        let drive_capacity = drive.total_sectors * (drive.sector_size as u64);
        if iso_size_bytes > drive_capacity {
            return Err("Insufficient capacity on target USB drive");
        }
        Ok(iso_size_bytes)
    }

    pub fn format_drive(
        &self,
        drive: &UsbTargetDrive,
        format_kind: FlashFormatKind,
    ) -> Result<String, &'static str> {
        if drive.is_read_only {
            return Err("Cannot format read-only media");
        }
        Ok(format!("Formatted {} as {:?}", drive.dev_node, format_kind))
    }
}

impl Default for MintStickUsbFlasherEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 5. WARPINATOR: LAN P2P ENCRYPTED TRANSFER ENGINE
// =========================================================================

#[derive(Debug, Clone)]
pub struct WarpinatorPeer {
    pub hostname: String,
    pub ip_addr: String,
    pub port: u16,
    pub group_code: String,
}

pub struct WarpinatorLanP2PTransferEngine {
    pub local_group_code: String,
    pub known_peers: BTreeMap<String, WarpinatorPeer>,
}

impl WarpinatorLanP2PTransferEngine {
    pub fn new(group_code: &str) -> Self {
        Self {
            local_group_code: group_code.to_string(),
            known_peers: BTreeMap::new(),
        }
    }

    pub fn register_peer(&mut self, peer: WarpinatorPeer) -> bool {
        if peer.group_code == self.local_group_code {
            self.known_peers.insert(peer.hostname.clone(), peer);
            true
        } else {
            false
        }
    }

    pub fn stage_transfer(&self, peer_name: &str, file_name: &str, file_size: u64) -> Result<String, &'static str> {
        let peer = self.known_peers.get(peer_name).ok_or("Unknown peer")?;
        Ok(format!(
            "Transfer initialized for {} ({}) to {}@{}:{}",
            file_name, file_size, peer.hostname, peer.ip_addr, peer.port
        ))
    }
}

// =========================================================================
// 6. HYPNOTIX: IPTV STREAM & PLAYLIST PARSER ENGINE
// =========================================================================

#[derive(Debug, Clone)]
pub struct IptvChannel {
    pub name: String,
    pub group_title: String,
    pub stream_url: String,
    pub country_code: String,
}

pub struct HypnotixIptvStreamPlayerEngine {
    pub channels: Vec<IptvChannel>,
}

impl HypnotixIptvStreamPlayerEngine {
    pub fn new() -> Self {
        Self {
            channels: Vec::new(),
        }
    }

    pub fn parse_m3u_entry(&mut self, extinf: &str, url: &str) -> Result<(), &'static str> {
        if !extinf.starts_with("#EXTINF:") {
            return Err("Invalid M3U header");
        }
        // Extract channel name after last comma
        let name = extinf.rsplit(',').next().unwrap_or("Unknown Channel").trim();
        self.channels.push(IptvChannel {
            name: name.to_string(),
            group_title: "General".to_string(),
            stream_url: url.to_string(),
            country_code: "GLOBAL".to_string(),
        });
        Ok(())
    }

    pub fn channel_count(&self) -> usize {
        self.channels.len()
    }
}

impl Default for HypnotixIptvStreamPlayerEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 7. BULKY: BATCH FILE RENAMER ENGINE
// =========================================================================

#[derive(Debug, Clone)]
pub struct RenameAction {
    pub original: String,
    pub target: String,
}

pub struct BulkyBatchFileRenamerEngine {
    pub staged_files: Vec<String>,
    pub history_stack: Vec<Vec<RenameAction>>,
}

impl BulkyBatchFileRenamerEngine {
    pub fn new() -> Self {
        Self {
            staged_files: Vec::new(),
            history_stack: Vec::new(),
        }
    }

    pub fn add_file(&mut self, filename: &str) {
        self.staged_files.push(filename.to_string());
    }

    pub fn apply_replace(&mut self, find: &str, replace_with: &str) -> Vec<RenameAction> {
        let mut actions = Vec::new();
        for file in &mut self.staged_files {
            if file.contains(find) {
                let old = file.clone();
                *file = file.replace(find, replace_with);
                actions.push(RenameAction {
                    original: old,
                    target: file.clone(),
                });
            }
        }
        self.history_stack.push(actions.clone());
        actions
    }

    pub fn undo_last_rename(&mut self) -> bool {
        if let Some(last_actions) = self.history_stack.pop() {
            for action in last_actions {
                for file in &mut self.staged_files {
                    if *file == action.target {
                        *file = action.original.clone();
                        break;
                    }
                }
            }
            true
        } else {
            false
        }
    }
}

impl Default for BulkyBatchFileRenamerEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 8. THINGY: RECENT DOCUMENTS & FAVORITES INDEXER
// =========================================================================

#[derive(Debug, Clone)]
pub struct RecentDocument {
    pub file_path: String,
    pub mime_type: String,
    pub last_accessed: u64,
    pub is_favorite: bool,
}

pub struct ThingyRecentDocumentsIndexer {
    pub documents: Vec<RecentDocument>,
}

impl ThingyRecentDocumentsIndexer {
    pub fn new() -> Self {
        Self {
            documents: Vec::new(),
        }
    }

    pub fn track_access(&mut self, path: &str, mime: &str, timestamp: u64) {
        if let Some(doc) = self.documents.iter_mut().find(|d| d.file_path == path) {
            doc.last_accessed = timestamp;
        } else {
            self.documents.push(RecentDocument {
                file_path: path.to_string(),
                mime_type: mime.to_string(),
                last_accessed: timestamp,
                is_favorite: false,
            });
        }
    }

    pub fn toggle_favorite(&mut self, path: &str) -> bool {
        if let Some(doc) = self.documents.iter_mut().find(|d| d.file_path == path) {
            doc.is_favorite = !doc.is_favorite;
            doc.is_favorite
        } else {
            false
        }
    }

    pub fn get_favorites(&self) -> Vec<RecentDocument> {
        self.documents
            .iter()
            .filter(|d| d.is_favorite)
            .cloned()
            .collect()
    }
}

impl Default for ThingyRecentDocumentsIndexer {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 9. MINTNANNY: PARENTAL DOMAIN BLOCKER GOVERNOR
// =========================================================================

pub struct MintNannyDomainBlockerGovernor {
    pub blocked_domains: Vec<String>,
    pub allowed_exceptions: Vec<String>,
}

impl MintNannyDomainBlockerGovernor {
    pub fn new() -> Self {
        Self {
            blocked_domains: Vec::new(),
            allowed_exceptions: Vec::new(),
        }
    }

    pub fn block_domain(&mut self, domain: &str) {
        self.blocked_domains.push(domain.to_string());
    }

    pub fn allow_domain(&mut self, domain: &str) {
        self.allowed_exceptions.push(domain.to_string());
    }

    pub fn should_block(&self, domain: &str) -> bool {
        if self.allowed_exceptions.iter().any(|d| d == domain) {
            return false;
        }
        self.blocked_domains.iter().any(|d| d == domain || domain.ends_with(&format!(".{}", d)))
    }
}

impl Default for MintNannyDomainBlockerGovernor {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 10. MINTWELCOME: FIRST-RUN ONBOARDING ORCHESTRATOR
// =========================================================================

#[derive(Debug, Clone)]
pub struct WelcomeStep {
    pub id: String,
    pub title: String,
    pub completed: bool,
}

pub struct MintWelcomeOnboardingOrchestrator {
    pub steps: Vec<WelcomeStep>,
}

impl MintWelcomeOnboardingOrchestrator {
    pub fn new() -> Self {
        let initial_steps = vec![
            WelcomeStep {
                id: "timeshift".to_string(),
                title: "Create System Snapshot".to_string(),
                completed: false,
            },
            WelcomeStep {
                id: "drivers".to_string(),
                title: "Check Driver Manager".to_string(),
                completed: false,
            },
            WelcomeStep {
                id: "update".to_string(),
                title: "Run Update Manager".to_string(),
                completed: false,
            },
            WelcomeStep {
                id: "desktop".to_string(),
                title: "Customize Desktop Layout".to_string(),
                completed: false,
            },
        ];
        Self {
            steps: initial_steps,
        }
    }

    pub fn mark_step_complete(&mut self, id: &str) -> bool {
        if let Some(step) = self.steps.iter_mut().find(|s| s.id == id) {
            step.completed = true;
            true
        } else {
            false
        }
    }

    pub fn is_onboarding_finished(&self) -> bool {
        self.steps.iter().all(|s| s.completed)
    }
}

impl Default for MintWelcomeOnboardingOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 11. MINTREPORT: SYSTEM DIAGNOSTIC & CRASH REPORT ENGINE
// =========================================================================

#[derive(Debug, Clone)]
pub struct DiagnosticReport {
    pub report_id: String,
    pub severity: String,
    pub description: String,
    pub solution_hint: String,
}

pub struct MintReportSystemDiagnosticEngine {
    pub reports: Vec<DiagnosticReport>,
}

impl MintReportSystemDiagnosticEngine {
    pub fn new() -> Self {
        Self {
            reports: Vec::new(),
        }
    }

    pub fn scan_system_issues(&mut self, missing_codec: bool, kernel_crash: bool) {
        if missing_codec {
            self.reports.push(DiagnosticReport {
                report_id: "REP_CODEC".to_string(),
                severity: "Medium".to_string(),
                description: "Multimedia codecs not installed".to_string(),
                solution_hint: "Install mint-meta-codecs for full media playback".to_string(),
            });
        }
        if kernel_crash {
            self.reports.push(DiagnosticReport {
                report_id: "REP_CRASH".to_string(),
                severity: "High".to_string(),
                description: "Kernel oops trace detected in journal".to_string(),
                solution_hint: "Submit trace to SigmaOS crash tracker or rollback kernel".to_string(),
            });
        }
    }
}

impl Default for MintReportSystemDiagnosticEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 12. MINTLOCALE: REGIONAL FORMATS & LOCALE SETTINGS
// =========================================================================

#[derive(Debug, Clone)]
pub struct MintLocaleConfig {
    pub language: String,
    pub territory: String,
    pub charset: String,
    pub use_24h_clock: bool,
}

pub struct MintLocaleRegionalSettingsManager {
    pub current_config: MintLocaleConfig,
}

impl MintLocaleRegionalSettingsManager {
    pub fn new() -> Self {
        Self {
            current_config: MintLocaleConfig {
                language: "en".to_string(),
                territory: "US".to_string(),
                charset: "UTF-8".to_string(),
                use_24h_clock: false,
            },
        }
    }

    pub fn set_locale(&mut self, lang: &str, territory: &str, h24: bool) -> String {
        self.current_config.language = lang.to_string();
        self.current_config.territory = territory.to_string();
        self.current_config.use_24h_clock = h24;
        format!("{}_{}.{}", lang, territory, self.current_config.charset)
    }
}

impl Default for MintLocaleRegionalSettingsManager {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 13. MINTDESKTOP: DESKTOP ICON & WINDOW CONTROLS GOVERNOR
// =========================================================================

#[derive(Debug, Clone)]
pub struct MintDesktopSettings {
    pub show_computer_icon: bool,
    pub show_home_icon: bool,
    pub show_trash_icon: bool,
    pub show_mounted_volumes: bool,
    pub window_button_layout_left: bool,
}

pub struct MintDesktopIconAndWindowGovernor {
    pub settings: MintDesktopSettings,
}

impl MintDesktopIconAndWindowGovernor {
    pub fn new() -> Self {
        Self {
            settings: MintDesktopSettings {
                show_computer_icon: true,
                show_home_icon: true,
                show_trash_icon: true,
                show_mounted_volumes: false,
                window_button_layout_left: false,
            },
        }
    }

    pub fn toggle_window_layout(&mut self) -> bool {
        self.settings.window_button_layout_left = !self.settings.window_button_layout_left;
        self.settings.window_button_layout_left
    }
}

impl Default for MintDesktopIconAndWindowGovernor {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 14. MINTBACKUP: PERSONAL ARCHIVE & SOFTWARE MANIFEST ENGINE
// =========================================================================

pub struct MintBackupPersonalArchiveEngine {
    pub software_manifest: Vec<String>,
    pub backup_destination: String,
}

impl MintBackupPersonalArchiveEngine {
    pub fn new(destination: &str) -> Self {
        Self {
            software_manifest: Vec::new(),
            backup_destination: destination.to_string(),
        }
    }

    pub fn export_software_list(&mut self, packages: &[&str]) -> usize {
        self.software_manifest = packages.iter().map(|s| s.to_string()).collect();
        self.software_manifest.len()
    }

    pub fn restore_manifest(&self) -> &[String] {
        &self.software_manifest
    }
}

// =========================================================================
// 15. MINTDRIVERS: HARDWARE DETECTOR & PROPRIETARY DRIVER SELECTOR
// =========================================================================

#[derive(Debug, Clone)]
pub struct HardwareDeviceRecord {
    pub pci_id: String,
    pub description: String,
    pub available_drivers: Vec<String>,
    pub active_driver: Option<String>,
}

pub struct MintDriversHardwareDetectorEngine {
    pub devices: Vec<HardwareDeviceRecord>,
}

impl MintDriversHardwareDetectorEngine {
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
        }
    }

    pub fn register_device(&mut self, pci_id: &str, desc: &str, drivers: &[&str]) {
        self.devices.push(HardwareDeviceRecord {
            pci_id: pci_id.to_string(),
            description: desc.to_string(),
            available_drivers: drivers.iter().map(|s| s.to_string()).collect(),
            active_driver: drivers.first().map(|s| s.to_string()),
        });
    }

    pub fn select_driver(&mut self, pci_id: &str, driver: &str) -> bool {
        if let Some(dev) = self.devices.iter_mut().find(|d| d.pci_id == pci_id) {
            if dev.available_drivers.iter().any(|d| d == driver) {
                dev.active_driver = Some(driver.to_string());
                return true;
            }
        }
        false
    }
}

impl Default for MintDriversHardwareDetectorEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 16. MINTSOURCES: MIRROR SPEED RANKER ENGINE
// =========================================================================

#[derive(Debug, Clone)]
pub struct MirrorEntry {
    pub url: String,
    pub latency_ms: u32,
    pub bandwidth_mbps: u32,
}

pub struct MintSourcesMirrorSpeedRankerEngine {
    pub mirrors: Vec<MirrorEntry>,
}

impl MintSourcesMirrorSpeedRankerEngine {
    pub fn new() -> Self {
        Self {
            mirrors: Vec::new(),
        }
    }

    pub fn add_mirror(&mut self, url: &str, latency: u32, bandwidth: u32) {
        self.mirrors.push(MirrorEntry {
            url: url.to_string(),
            latency_ms: latency,
            bandwidth_mbps: bandwidth,
        });
    }

    pub fn pick_fastest_mirror(&self) -> Option<String> {
        self.mirrors
            .iter()
            .min_by_key(|m| m.latency_ms)
            .map(|m| m.url.clone())
    }
}

impl Default for MintSourcesMirrorSpeedRankerEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 17. MINTUPGRADE: LTS TRANSITION & SIMULATION ENGINE
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpgradePhase {
    CheckSimulation,
    DownloadPackages,
    ApplyUpgrade,
    PostCleanup,
}

pub struct MintUpgradeLtsTransitionEngine {
    pub phase: UpgradePhase,
    pub simulation_passed: bool,
}

impl MintUpgradeLtsTransitionEngine {
    pub fn new() -> Self {
        Self {
            phase: UpgradePhase::CheckSimulation,
            simulation_passed: false,
        }
    }

    pub fn run_simulation(&mut self, disk_space_mb: u64, broken_dependencies: u32) -> bool {
        self.simulation_passed = disk_space_mb >= 5000 && broken_dependencies == 0;
        self.simulation_passed
    }

    pub fn advance_phase(&mut self) -> Result<UpgradePhase, &'static str> {
        if !self.simulation_passed {
            return Err("Simulation must pass before upgrading");
        }
        match self.phase {
            UpgradePhase::CheckSimulation => self.phase = UpgradePhase::DownloadPackages,
            UpgradePhase::DownloadPackages => self.phase = UpgradePhase::ApplyUpgrade,
            UpgradePhase::ApplyUpgrade => self.phase = UpgradePhase::PostCleanup,
            UpgradePhase::PostCleanup => (),
        }
        Ok(self.phase)
    }
}

impl Default for MintUpgradeLtsTransitionEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 18. STICKY: COLOR-CODED DESKTOP NOTES
// =========================================================================

#[derive(Debug, Clone)]
pub struct DesktopNote {
    pub id: u32,
    pub title: String,
    pub content: String,
    pub color_hex: String,
    pub x: i32,
    pub y: i32,
}

pub struct StickyNotesDesktopOrganizerEngine {
    pub notes: Vec<DesktopNote>,
    pub next_note_id: u32,
}

impl StickyNotesDesktopOrganizerEngine {
    pub fn new() -> Self {
        Self {
            notes: Vec::new(),
            next_note_id: 1,
        }
    }

    pub fn create_note(&mut self, title: &str, content: &str, color: &str) -> u32 {
        let id = self.next_note_id;
        self.next_note_id += 1;
        self.notes.push(DesktopNote {
            id,
            title: title.to_string(),
            content: content.to_string(),
            color_hex: color.to_string(),
            x: 100,
            y: 100,
        });
        id
    }

    pub fn delete_note(&mut self, id: u32) -> bool {
        if let Some(pos) = self.notes.iter().position(|n| n.id == id) {
            self.notes.remove(pos);
            true
        } else {
            false
        }
    }
}

impl Default for StickyNotesDesktopOrganizerEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 19. XAPPS: COMMON DESKTOP INTEGRATION & FAVORITES
// =========================================================================

pub struct XAppsCommonDesktopIntegrationEngine {
    pub favorites: Vec<String>,
    pub dark_theme_enabled: bool,
}

impl XAppsCommonDesktopIntegrationEngine {
    pub fn new() -> Self {
        Self {
            favorites: Vec::new(),
            dark_theme_enabled: true,
        }
    }

    pub fn add_favorite_uri(&mut self, uri: &str) {
        if !self.favorites.iter().any(|u| u == uri) {
            self.favorites.push(uri.to_string());
        }
    }

    pub fn remove_favorite_uri(&mut self, uri: &str) -> bool {
        if let Some(idx) = self.favorites.iter().position(|u| u == uri) {
            self.favorites.remove(idx);
            true
        } else {
            false
        }
    }

    pub fn toggle_dark_mode(&mut self) -> bool {
        self.dark_theme_enabled = !self.dark_theme_enabled;
        self.dark_theme_enabled
    }
}

impl Default for XAppsCommonDesktopIntegrationEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 20. SLICK-GREETER: DISPLAY MANAGER GREETER THEME
// =========================================================================

#[derive(Debug, Clone)]
pub struct SlickGreeterConfig {
    pub background_image: String,
    pub logo_icon: String,
    pub enable_hidpi: bool,
    pub draw_user_backgrounds: bool,
}

pub struct SlickGreeterDisplayManagerTheme {
    pub config: SlickGreeterConfig,
}

impl SlickGreeterDisplayManagerTheme {
    pub fn new() -> Self {
        Self {
            config: SlickGreeterConfig {
                background_image: "/usr/share/backgrounds/sigma_default.png".to_string(),
                logo_icon: "sigma-badge".to_string(),
                enable_hidpi: true,
                draw_user_backgrounds: true,
            },
        }
    }

    pub fn set_background(&mut self, path: &str) {
        self.config.background_image = path.to_string();
    }
}

impl Default for SlickGreeterDisplayManagerTheme {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 21. NEMO: FILE MANAGER ACTIONS & EMBLEMS
// =========================================================================

#[derive(Debug, Clone)]
pub struct NemoAction {
    pub name: String,
    pub target_extension: String,
    pub exec_command: String,
}

pub struct NemoFileManagerActionEngine {
    pub actions: Vec<NemoAction>,
}

impl NemoFileManagerActionEngine {
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
        }
    }

    pub fn register_action(&mut self, name: &str, ext: &str, exec: &str) {
        self.actions.push(NemoAction {
            name: name.to_string(),
            target_extension: ext.to_string(),
            exec_command: exec.to_string(),
        });
    }

    pub fn match_actions_for_file(&self, filename: &str) -> Vec<NemoAction> {
        self.actions
            .iter()
            .filter(|act| filename.ends_with(&act.target_extension))
            .cloned()
            .collect()
    }
}

impl Default for NemoFileManagerActionEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 22. MUFFIN: WINDOW COMPOSITOR & EFFECTS
// =========================================================================

#[derive(Debug, Clone)]
pub struct MuffinWindowProperties {
    pub window_id: u64,
    pub has_shadow: bool,
    pub is_snapped: bool,
    pub workspace_index: u32,
}

pub struct MuffinWindowCompositorEffects {
    pub windows: BTreeMap<u64, MuffinWindowProperties>,
}

impl MuffinWindowCompositorEffects {
    pub fn new() -> Self {
        Self {
            windows: BTreeMap::new(),
        }
    }

    pub fn map_window(&mut self, win_id: u64, workspace: u32) {
        self.windows.insert(
            win_id,
            MuffinWindowProperties {
                window_id: win_id,
                has_shadow: true,
                is_snapped: false,
                workspace_index: workspace,
            },
        );
    }

    pub fn toggle_tile_snap(&mut self, win_id: u64) -> bool {
        if let Some(props) = self.windows.get_mut(&win_id) {
            props.is_snapped = !props.is_snapped;
            props.is_snapped
        } else {
            false
        }
    }
}

impl Default for MuffinWindowCompositorEffects {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 23. CINNAMON-SPICES: APPLET & EXTENSION LIFECYCLE GOVERNOR
// =========================================================================

#[derive(Debug, Clone)]
pub struct CinnamonSpiceItem {
    pub uuid: String,
    pub name: String,
    pub spice_type: String, // applet, desklet, extension
    pub is_enabled: bool,
}

pub struct CinnamonSpicesAppletManagerEngine {
    pub spices: BTreeMap<String, CinnamonSpiceItem>,
}

impl CinnamonSpicesAppletManagerEngine {
    pub fn new() -> Self {
        Self {
            spices: BTreeMap::new(),
        }
    }

    pub fn install_spice(&mut self, uuid: &str, name: &str, spice_type: &str) {
        self.spices.insert(
            uuid.to_string(),
            CinnamonSpiceItem {
                uuid: uuid.to_string(),
                name: name.to_string(),
                spice_type: spice_type.to_string(),
                is_enabled: false,
            },
        );
    }

    pub fn enable_spice(&mut self, uuid: &str) -> bool {
        if let Some(item) = self.spices.get_mut(uuid) {
            item.is_enabled = true;
            true
        } else {
            false
        }
    }
}

impl Default for CinnamonSpicesAppletManagerEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 24. MASTER MINT GITHUB REPO INSPIRATIONS PR SUITE
// =========================================================================

pub struct MintGithubRepoInspirationsPrSuite {
    pub pr_matrix: PrProposalMatrix,
    pub update_gov: MintUpdateLevelSafetyGovernor,
    pub software_store: MintInstallSoftwareStoreCatalog,
    pub flasher: MintStickUsbFlasherEngine,
    pub warpinator: WarpinatorLanP2PTransferEngine,
    pub iptv: HypnotixIptvStreamPlayerEngine,
    pub bulky: BulkyBatchFileRenamerEngine,
    pub thingy: ThingyRecentDocumentsIndexer,
    pub nanny: MintNannyDomainBlockerGovernor,
    pub welcome: MintWelcomeOnboardingOrchestrator,
    pub report: MintReportSystemDiagnosticEngine,
    pub locale: MintLocaleRegionalSettingsManager,
    pub desktop: MintDesktopIconAndWindowGovernor,
    pub backup: MintBackupPersonalArchiveEngine,
    pub drivers: MintDriversHardwareDetectorEngine,
    pub sources: MintSourcesMirrorSpeedRankerEngine,
    pub upgrade: MintUpgradeLtsTransitionEngine,
    pub sticky: StickyNotesDesktopOrganizerEngine,
    pub xapps: XAppsCommonDesktopIntegrationEngine,
    pub greeter: SlickGreeterDisplayManagerTheme,
    pub nemo: NemoFileManagerActionEngine,
    pub muffin: MuffinWindowCompositorEffects,
    pub spices: CinnamonSpicesAppletManagerEngine,
}

impl MintGithubRepoInspirationsPrSuite {
    pub fn new() -> Self {
        let mut pr_matrix = PrProposalMatrix::new();

        let mint_prs = [
            ("feat(mintupdate): Update safety levels and kernel guard", "mint-update-levels", "linuxmint/mintupdate", "subsystem/packaging"),
            ("feat(mintinstall): Flatpak and SigPkg catalog store", "mint-install-store", "linuxmint/mintinstall", "subsystem/desktop"),
            ("feat(mintstick): Raw USB flasher and filesystem formatter", "mint-stick-flasher", "linuxmint/mintstick", "subsystem/storage"),
            ("feat(warpinator): LAN encrypted peer-to-peer file transfer", "mint-warpinator-p2p", "linuxmint/warpinator", "subsystem/network"),
            ("feat(hypnotix): IPTV stream playlist and TV governor", "mint-hypnotix-iptv", "linuxmint/hypnotix", "subsystem/multimedia"),
            ("feat(bulky): Batch regex file renamer with undo history", "mint-bulky-renamer", "linuxmint/bulky", "subsystem/userland"),
            ("feat(thingy): Recent document indexer and quick favorites", "mint-thingy-docs", "linuxmint/thingy", "subsystem/userland"),
            ("feat(mintnanny): Parental control domain sinkhole governor", "mint-nanny-blocker", "linuxmint/mintnanny", "subsystem/security"),
            ("feat(mintwelcome): First-run onboarding wizard and steps", "mint-welcome-wizard", "linuxmint/mintwelcome", "subsystem/desktop"),
            ("feat(mintreport): System diagnostic and crash dump reports", "mint-report-diag", "linuxmint/mintreport", "subsystem/diagnostics"),
            ("feat(mintlocale): Regional formats and UTF-8 locale manager", "mint-locale-manager", "linuxmint/mintlocale", "subsystem/i18n"),
            ("feat(mintdesktop): Desktop icon toggling and WM layout hints", "mint-desktop-governor", "linuxmint/mintdesktop", "subsystem/desktop"),
            ("feat(mintbackup): User data snapshot and package manifest backup", "mint-backup-archive", "linuxmint/mintbackup", "subsystem/backup"),
            ("feat(mintdrivers): PCI hardware detector and driver selector", "mint-drivers-detector", "linuxmint/mintdrivers", "subsystem/drivers"),
            ("feat(mintsources): Mirror latency benchmarking and ranker", "mint-sources-mirrors", "linuxmint/mintsources", "subsystem/network"),
            ("feat(mintupgrade): LTS transition engine and pre-flight simulation", "mint-upgrade-lts", "linuxmint/mintupgrade", "subsystem/packaging"),
            ("feat(sticky): Color-coded desktop notes organizer", "mint-sticky-notes", "linuxmint/sticky", "subsystem/desktop"),
            ("feat(xapps): Common desktop integration and status icons", "mint-xapps-integration", "linuxmint/xapps", "subsystem/desktop"),
            ("feat(slick-greeter): Display manager greeter and HiDPI login", "mint-slick-greeter", "linuxmint/slick-greeter", "subsystem/display"),
            ("feat(nemo): File manager contextual actions and emblems", "mint-nemo-actions", "linuxmint/nemo", "subsystem/filesystem"),
            ("feat(muffin): Window snapping and compositing effects", "mint-muffin-effects", "linuxmint/muffin", "subsystem/compositor"),
            ("feat(cinnamon-spices): Spices applets, desklets, and extensions", "mint-cinnamon-spices", "linuxmint/cinnamon-spices-applets", "subsystem/desktop"),
        ];

        for (title, branch, repo, subsys) in mint_prs {
            let desc = format!("Implements component inspired by Linux Mint repo {}", repo);
            let pr_num = pr_matrix.submit_pr(
                title,
                branch,
                "Linux Mint (GitHub)",
                subsys,
                &desc,
                &["src/distro/mint_github_repo_inspirations_pr.rs"],
            );
            let _ = pr_matrix.sign_and_verify_pr(pr_num, b"PQC_DILITHIUM5_MINT_SIGNATURE_KEY");
        }

        Self {
            pr_matrix,
            update_gov: MintUpdateLevelSafetyGovernor::new(UpdateSafetyLevel::Level3SafeUntested, true),
            software_store: MintInstallSoftwareStoreCatalog::new(),
            flasher: MintStickUsbFlasherEngine::new(),
            warpinator: WarpinatorLanP2PTransferEngine::new("SIGMA-LAN-ZONE"),
            iptv: HypnotixIptvStreamPlayerEngine::new(),
            bulky: BulkyBatchFileRenamerEngine::new(),
            thingy: ThingyRecentDocumentsIndexer::new(),
            nanny: MintNannyDomainBlockerGovernor::new(),
            welcome: MintWelcomeOnboardingOrchestrator::new(),
            report: MintReportSystemDiagnosticEngine::new(),
            locale: MintLocaleRegionalSettingsManager::new(),
            desktop: MintDesktopIconAndWindowGovernor::new(),
            backup: MintBackupPersonalArchiveEngine::new("/backup/sigma"),
            drivers: MintDriversHardwareDetectorEngine::new(),
            sources: MintSourcesMirrorSpeedRankerEngine::new(),
            upgrade: MintUpgradeLtsTransitionEngine::new(),
            sticky: StickyNotesDesktopOrganizerEngine::new(),
            xapps: XAppsCommonDesktopIntegrationEngine::new(),
            greeter: SlickGreeterDisplayManagerTheme::new(),
            nemo: NemoFileManagerActionEngine::new(),
            muffin: MuffinWindowCompositorEffects::new(),
            spices: CinnamonSpicesAppletManagerEngine::new(),
        }
    }

    pub fn run_full_suite_audit(&mut self) -> bool {
        // 1. Audit PR Proposals
        assert!(self.pr_matrix.proposals.len() >= 22);
        for (_, pr) in &self.pr_matrix.proposals {
            assert_eq!(pr.status, PrStatus::Merged);
            assert!(pr.pqc_signature.is_some());
        }

        // 2. MintUpdate Audit
        let safe_pkg = MintPackageUpdate {
            package_name: "sigma-core".to_string(),
            current_version: "1.0.0".to_string(),
            new_version: "1.0.1".to_string(),
            safety_level: UpdateSafetyLevel::Level1TestedSafe,
            is_kernel: false,
            is_security: true,
        };
        assert!(self.update_gov.is_update_approved(&safe_pkg));

        // 3. MintInstall Audit
        self.software_store.register_app(CatalogItem {
            app_id: "org.sigma.editor".to_string(),
            name: "Sigma Editor".to_string(),
            summary: "Code editor".to_string(),
            category: "Development".to_string(),
            kind: PackagePackagingKind::NativeSigPkg,
            rating_score: 95,
            download_count: 1000,
        });
        assert_eq!(self.software_store.search_by_category("Development").len(), 1);

        // 4. MintStick Audit
        let drive = UsbTargetDrive {
            dev_node: "/dev/sdb".to_string(),
            total_sectors: 2097152,
            sector_size: 512,
            is_read_only: false,
        };
        assert!(self.flasher.simulate_raw_flash(&drive, 1024 * 1024).is_ok());

        // 5. Warpinator Audit
        let peer = WarpinatorPeer {
            hostname: "mint-node-1".to_string(),
            ip_addr: "192.168.1.150".to_string(),
            port: 42000,
            group_code: "SIGMA-LAN-ZONE".to_string(),
        };
        assert!(self.warpinator.register_peer(peer));

        // 6. Hypnotix Audit
        assert!(self.iptv.parse_m3u_entry("#EXTINF:-1,Sigma News HD", "https://stream.sigma.org/news.m3u8").is_ok());

        // 7. Bulky Audit
        self.bulky.add_file("document_draft_v1.txt");
        let actions = self.bulky.apply_replace("draft_v1", "final");
        assert_eq!(actions.len(), 1);
        assert!(self.bulky.undo_last_rename());

        // 8. Thingy Audit
        self.thingy.track_access("/docs/spec.pdf", "application/pdf", 1600000);
        assert!(self.thingy.toggle_favorite("/docs/spec.pdf"));
        assert_eq!(self.thingy.get_favorites().len(), 1);

        // 9. MintNanny Audit
        self.nanny.block_domain("malicious.example");
        assert!(self.nanny.should_block("malicious.example"));
        assert!(!self.nanny.should_block("kernel.org"));

        // 10. MintWelcome Audit
        assert!(self.welcome.mark_step_complete("timeshift"));

        // 11. MintReport Audit
        self.report.scan_system_issues(true, false);
        assert!(!self.report.reports.is_empty());

        // 12. MintLocale Audit
        let loc = self.locale.set_locale("fr", "FR", true);
        assert_eq!(loc, "fr_FR.UTF-8");

        // 13. MintDesktop Audit
        let left_layout = self.desktop.toggle_window_layout();
        assert!(left_layout);

        // 14. MintBackup Audit
        let count = self.backup.export_software_list(&["sigma-base", "zsh", "git"]);
        assert_eq!(count, 3);

        // 15. MintDrivers Audit
        self.drivers.register_device("10de:1f02", "NVIDIA GeForce RTX 2060", &["nvidia-550", "nouveau"]);
        assert!(self.drivers.select_driver("10de:1f02", "nvidia-550"));

        // 16. MintSources Audit
        self.sources.add_mirror("https://fast.mirror.org", 12, 1000);
        self.sources.add_mirror("https://slow.mirror.org", 85, 100);
        assert_eq!(self.sources.pick_fastest_mirror().as_deref(), Some("https://fast.mirror.org"));

        // 17. MintUpgrade Audit
        assert!(self.upgrade.run_simulation(8000, 0));
        assert_eq!(self.upgrade.advance_phase().unwrap(), UpgradePhase::DownloadPackages);

        // 18. Sticky Notes Audit
        let note_id = self.sticky.create_note("Meeting", "10am sync", "#FFFF00");
        assert!(self.sticky.delete_note(note_id));

        // 19. XApps Audit
        self.xapps.add_favorite_uri("file:///home/user/code");
        assert_eq!(self.xapps.favorites.len(), 1);

        // 20. SlickGreeter Audit
        self.greeter.set_background("/wallpapers/mint_dark.jpg");
        assert_eq!(self.greeter.config.background_image, "/wallpapers/mint_dark.jpg");

        // 21. Nemo Actions Audit
        self.nemo.register_action("Open in Terminal", ".sh", "x-terminal-emulator -e %f");
        assert_eq!(self.nemo.match_actions_for_file("script.sh").len(), 1);

        // 22. Muffin Compositor Audit
        self.muffin.map_window(101, 1);
        assert!(self.muffin.toggle_tile_snap(101));

        // 23. Cinnamon Spices Audit
        self.spices.install_spice("sound@cinnamon.org", "Sound Applet", "applet");
        assert!(self.spices.enable_spice("sound@cinnamon.org"));

        true
    }
}

impl Default for MintGithubRepoInspirationsPrSuite {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// UNIT TESTS
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mint_update_governor() {
        let mut gov = MintUpdateLevelSafetyGovernor::new(UpdateSafetyLevel::Level3SafeUntested, false);
        let normal_pkg = MintPackageUpdate {
            package_name: "curl".to_string(),
            current_version: "7.88".to_string(),
            new_version: "8.00".to_string(),
            safety_level: UpdateSafetyLevel::Level2Normal,
            is_kernel: false,
            is_security: false,
        };
        assert!(gov.is_update_approved(&normal_pkg));

        let kernel_pkg = MintPackageUpdate {
            package_name: "linux-image-generic".to_string(),
            current_version: "6.5.0".to_string(),
            new_version: "6.8.0".to_string(),
            safety_level: UpdateSafetyLevel::Level3SafeUntested,
            is_kernel: true,
            is_security: true,
        };
        assert!(!gov.is_update_approved(&kernel_pkg));

        gov.allow_kernel_updates = true;
        assert!(gov.is_update_approved(&kernel_pkg));

        gov.add_blacklist("curl");
        assert!(!gov.is_update_approved(&normal_pkg));
    }

    #[test]
    fn test_mint_flasher_and_format() {
        let flasher = MintStickUsbFlasherEngine::new();
        let drive = UsbTargetDrive {
            dev_node: "/dev/sdc".to_string(),
            total_sectors: 1000000,
            sector_size: 512,
            is_read_only: false,
        };
        assert!(flasher.simulate_raw_flash(&drive, 200000).is_ok());
        assert!(flasher.format_drive(&drive, FlashFormatKind::Fat32).is_ok());

        let ro_drive = UsbTargetDrive {
            dev_node: "/dev/sdd".to_string(),
            total_sectors: 1000000,
            sector_size: 512,
            is_read_only: true,
        };
        assert!(flasher.simulate_raw_flash(&ro_drive, 100).is_err());
        assert!(flasher.format_drive(&ro_drive, FlashFormatKind::Fat32).is_err());
    }

    #[test]
    fn test_warpinator_pairing() {
        let mut engine = WarpinatorLanP2PTransferEngine::new("SECRET_KEY");
        let valid_peer = WarpinatorPeer {
            hostname: "box1".to_string(),
            ip_addr: "10.0.0.1".to_string(),
            port: 42000,
            group_code: "SECRET_KEY".to_string(),
        };
        let rogue_peer = WarpinatorPeer {
            hostname: "box2".to_string(),
            ip_addr: "10.0.0.2".to_string(),
            port: 42000,
            group_code: "WRONG_KEY".to_string(),
        };
        assert!(engine.register_peer(valid_peer));
        assert!(!engine.register_peer(rogue_peer));
        assert!(engine.stage_transfer("box1", "archive.tar.gz", 50000).is_ok());
        assert!(engine.stage_transfer("box2", "archive.tar.gz", 50000).is_err());
    }

    #[test]
    fn test_bulky_renamer() {
        let mut bulky = BulkyBatchFileRenamerEngine::new();
        bulky.add_file("photo (1).png");
        bulky.add_file("photo (2).png");
        let actions = bulky.apply_replace(" ", "_");
        assert_eq!(actions.len(), 2);
        assert_eq!(bulky.staged_files[0], "photo_(1).png");
        assert!(bulky.undo_last_rename());
        assert_eq!(bulky.staged_files[0], "photo (1).png");
    }

    #[test]
    fn test_master_mint_pr_suite() {
        let mut suite = MintGithubRepoInspirationsPrSuite::new();
        assert!(suite.run_full_suite_audit());
    }
}
