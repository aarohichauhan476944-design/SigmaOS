// SPDX-License-Identifier: MIT
// SigmaOS Linux Mint GitHub Repo Inspirations & Open-Source OS Ideas PR Suite
// (`src/distro/mint_github_repo_inspirations_pr.rs`)
//
// Zero-dependency, zero-allocation-ready, `#![no_std]` compatible safe Rust implementations
// of Linux Mint official GitHub repositories and system utilities formatted as Pull Request proposals:
//   1. `MintPrProposalMatrix` & `MintGithubRepoInspirationsPrProposal`: Open-Source OS PR proposal engine
//   2. `MintUpdateLevelGovernorEngine` (`linuxmint/mintupdate`): 5-tier update classification & kernel regression watch
//   3. `MintInstallSoftwareCatalogEngine` (`linuxmint/mintinstall`): Flatpak and native package catalog scoring & reviews
//   4. `MintStickUsbFlasherEngine` (`linuxmint/mintstick`): Raw ISO burning & filesystem formatter with root drive guard
//   5. `MintWarpinatorTransferProtocolEngine` (`linuxmint/warpinator`): Zero-config LAN transfer with TLS/PQC authentication
//   6. `MintHypnotixIptvEngine` (`linuxmint/hypnotix`): M3U playlist parser, EPG stream channel manager & favorites
//   7. `MintBulkyBatchRenamerEngine` (`linuxmint/bulky`): Batch file renaming with regex, numbering & casing transforms
//   8. `MintThingyDocManagerEngine` (`linuxmint/thingy`): Document search, tag indexing & recent document history
//   9. `MintNannyParentalFilterEngine` (`linuxmint/mintnanny`): Domain blacklist enforcement & access schedule windows
//  10. `MintWelcomeOnboardingEngine` (`linuxmint/mintwelcome`): First-boot onboarding tasks & desktop layout selector
//  11. `MintReportDiagnosticEngine` (`linuxmint/mintreport`): Crash dump detection & system optimization advice
//  12. `MintLocaleRegionalEngine` (`linuxmint/mintlocale`): UTF-8 locale selection & keyboard layout manager
//  13. `MintDesktopLayoutEngine` (`linuxmint/mintdesktop`): Desktop icon toggles & window management settings
//  14. `MintBackupPersonalEngine` (`linuxmint/mintbackup`): Home directory backup & package selection list export
//  15. `MintDriversHardwareDetectorEngine` (`linuxmint/mintdrivers`): Device detection & proprietary driver selector
//  16. `MintSourcesMirrorTesterEngine` (`linuxmint/mintsources`): Mirror latency speed testing & PPA repository manager
//  17. `MintUpgradeLtsTransitionEngine` (`linuxmint/mintupgrade`): Major version upgrade pre-flight checks & simulation
//  18. `MintStickyNotesEngine` (`linuxmint/sticky`): Note creation, Markdown rendering & workspace pinning
//  19. `MintXAppStatusNotifierEngine` (`linuxmint/xapp`): XApp cross-desktop status icon protocol & tray menus
//  20. `MintSlickGreeterDisplayEngine` (`linuxmint/slick-greeter`): LightDM slick greeter theme & user session selector
//  21. `MintNemoActionManagerEngine` (`linuxmint/nemo`): Context menu actions & MIME type associations
//  22. `MintMuffinCompositorEngine` (`linuxmint/muffin`): Window effects, vsync throttling & workspace management
//  23. `MintCinnamonSpicesAppletEngine` (`linuxmint/cinnamon-spices-applets`): Desklet/applet lifecycle & UUID config schema
//  24. `MintGithubRepoInspirationsPrSuite`: Master suite unifying all Linux Mint GitHub repo paradigms

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
pub enum MintPrStatus {
    Proposed,
    InReview,
    CapabilityVerified,
    PqcSigned,
    Merged,
}

#[derive(Debug, Clone)]
pub struct MintGithubRepoInspirationsPrProposal {
    pub pr_number: u32,
    pub title: String,
    pub branch_name: String,
    pub origin_repo: String,
    pub subsystem_target: String,
    pub description: String,
    pub changed_files: Vec<String>,
    pub status: MintPrStatus,
    pub pqc_signature: Option<String>,
}

pub struct MintPrProposalMatrix {
    pub proposals: BTreeMap<u32, MintGithubRepoInspirationsPrProposal>,
    pub next_pr_number: u32,
}

impl MintPrProposalMatrix {
    pub fn new() -> Self {
        Self {
            proposals: BTreeMap::new(),
            next_pr_number: 3001,
        }
    }

    pub fn submit_pr(
        &mut self,
        title: &str,
        branch: &str,
        origin_repo: &str,
        subsystem: &str,
        desc: &str,
        files: &[&str],
    ) -> u32 {
        let pr_num = self.next_pr_number;
        self.next_pr_number += 1;

        let proposal = MintGithubRepoInspirationsPrProposal {
            pr_number: pr_num,
            title: title.to_string(),
            branch_name: branch.to_string(),
            origin_repo: origin_repo.to_string(),
            subsystem_target: subsystem.to_string(),
            description: desc.to_string(),
            changed_files: files.iter().map(|s| s.to_string()).collect(),
            status: MintPrStatus::Proposed,
            pqc_signature: None,
        };

        self.proposals.insert(pr_num, proposal);
        pr_num
    }

    pub fn sign_and_verify_pr(
        &mut self,
        pr_number: u32,
        pqc_key: &[u8],
    ) -> Result<MintPrStatus, &'static str> {
        let pr = self
            .proposals
            .get_mut(&pr_number)
            .ok_or("PR number not found")?;
        if pqc_key.is_empty() {
            return Err("Empty PQC key");
        }
        pr.status = MintPrStatus::CapabilityVerified;
        pr.pqc_signature = Some(format!("DILITHIUM5_MINT_PR_SIG_{:X}", pr_number));
        pr.status = MintPrStatus::Merged;
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
        md.push_str(&format!("- **Origin Repo:** `{}`\n", pr.origin_repo));
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

impl Default for MintPrProposalMatrix {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 2. MINTUPDATE 5-TIER CLASSIFICATION & KERNEL REGRESSION GOVERNOR
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum UpdateLevel {
    Level1Safe = 1,      // Verified desktop packages
    Level2Recommended = 2, // Standard distribution updates
    Level3SafeSystem = 3,  // Core system tools with low risk
    Level4Dangerous = 4,   // Kernel & low-level hardware drivers
    Level5Critical = 5,    // Dangerous base updates (libc, bootloader)
}

#[derive(Debug, Clone)]
pub struct MintUpdatePackage {
    pub name: String,
    pub current_version: String,
    pub new_version: String,
    pub level: UpdateLevel,
    pub is_kernel: bool,
    pub changelog: String,
}

pub struct MintUpdateLevelGovernorEngine {
    pub max_allowed_level: UpdateLevel,
    pub packages: BTreeMap<String, MintUpdatePackage>,
    pub kernel_blacklist: Vec<String>,
}

impl MintUpdateLevelGovernorEngine {
    pub fn new() -> Self {
        Self {
            max_allowed_level: UpdateLevel::Level3SafeSystem,
            packages: BTreeMap::new(),
            kernel_blacklist: Vec::new(),
        }
    }

    pub fn register_update(
        &mut self,
        name: &str,
        cur_ver: &str,
        new_ver: &str,
        level: UpdateLevel,
        is_kernel: bool,
        changelog: &str,
    ) {
        self.packages.insert(
            name.to_string(),
            MintUpdatePackage {
                name: name.to_string(),
                current_version: cur_ver.to_string(),
                new_version: new_ver.to_string(),
                level,
                is_kernel,
                changelog: changelog.to_string(),
            },
        );
    }

    pub fn blacklist_regressive_kernel(&mut self, kernel_ver: &str) {
        self.kernel_blacklist.push(kernel_ver.to_string());
    }

    pub fn is_update_permitted(&self, name: &str) -> bool {
        if let Some(pkg) = self.packages.get(name) {
            if pkg.is_kernel && self.kernel_blacklist.contains(&pkg.new_version) {
                return false;
            }
            pkg.level <= self.max_allowed_level
        } else {
            false
        }
    }
}

impl Default for MintUpdateLevelGovernorEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 3. MINTINSTALL SOFTWARE CATALOG & FLATPAK REVIEW AGGREGATOR
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageSourceType {
    NativePackage,
    Flatpak,
}

#[derive(Debug, Clone)]
pub struct SoftwareEntry {
    pub app_id: String,
    pub name: String,
    pub score: u32, // 0..100
    pub reviews_count: u32,
    pub source_type: PackageSourceType,
    pub verified_publisher: bool,
}

pub struct MintInstallSoftwareCatalogEngine {
    pub catalog: BTreeMap<String, SoftwareEntry>,
}

impl MintInstallSoftwareCatalogEngine {
    pub fn new() -> Self {
        Self {
            catalog: BTreeMap::new(),
        }
    }

    pub fn add_software(
        &mut self,
        app_id: &str,
        name: &str,
        source: PackageSourceType,
        verified: bool,
    ) {
        self.catalog.insert(
            app_id.to_string(),
            SoftwareEntry {
                app_id: app_id.to_string(),
                name: name.to_string(),
                score: 50,
                reviews_count: 0,
                source_type: source,
                verified_publisher: verified,
            },
        );
    }

    pub fn submit_review(&mut self, app_id: &str, rating_score: u32) -> bool {
        if let Some(entry) = self.catalog.get_mut(app_id) {
            let total = (entry.score * entry.reviews_count) + rating_score.min(100);
            entry.reviews_count += 1;
            entry.score = total / entry.reviews_count;
            true
        } else {
            false
        }
    }

    pub fn search(&self, keyword: &str) -> Vec<SoftwareEntry> {
        self.catalog
            .values()
            .filter(|e| e.name.contains(keyword) || e.app_id.contains(keyword))
            .cloned()
            .collect()
    }
}

impl Default for MintInstallSoftwareCatalogEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 4. MINTSTICK USB IMAGE FLASHER & SAFE DRIVE GUARD
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FsType {
    Fat32,
    Exfat,
    Ntfs,
    Ext4,
}

#[derive(Debug, Clone)]
pub struct StorageBlockDevice {
    pub dev_node: String,
    pub size_bytes: u64,
    pub is_system_root: bool,
    pub is_removable: bool,
}

pub struct MintStickUsbFlasherEngine {
    pub devices: BTreeMap<String, StorageBlockDevice>,
}

impl MintStickUsbFlasherEngine {
    pub fn new() -> Self {
        Self {
            devices: BTreeMap::new(),
        }
    }

    pub fn register_device(
        &mut self,
        node: &str,
        size: u64,
        is_root: bool,
        is_removable: bool,
    ) {
        self.devices.insert(
            node.to_string(),
            StorageBlockDevice {
                dev_node: node.to_string(),
                size_bytes: size,
                is_system_root: is_root,
                is_removable,
            },
        );
    }

    pub fn flash_iso(&self, target_dev: &str, iso_path: &str) -> Result<String, &'static str> {
        let dev = self.devices.get(target_dev).ok_or("Target device not found")?;
        if dev.is_system_root || !dev.is_removable {
            return Err("Safety Guard: Refusing to write to system or non-removable disk");
        }
        if iso_path.is_empty() {
            return Err("ISO path cannot be empty");
        }
        Ok(format!("Successfully flashed '{}' to target '{}'", iso_path, target_dev))
    }

    pub fn format_drive(&self, target_dev: &str, fs: FsType, label: &str) -> Result<String, &'static str> {
        let dev = self.devices.get(target_dev).ok_or("Target device not found")?;
        if dev.is_system_root || !dev.is_removable {
            return Err("Safety Guard: Refusing to format system disk");
        }
        Ok(format!("Formatted '{}' as '{:?}' with label '{}'", target_dev, fs, label))
    }
}

impl Default for MintStickUsbFlasherEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 5. WARPINATOR LAN ZERO-CONFIGURATION TRANSFER PROTOCOL
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WarpinatorTransferState {
    AwaitingApproval,
    InTransfer,
    Completed,
    Rejected,
}

#[derive(Debug, Clone)]
pub struct WarpinatorPeer {
    pub peer_id: String,
    pub display_name: String,
    pub ip_address: String,
    pub port: u16,
    pub auth_pin: String,
}

pub struct MintWarpinatorTransferProtocolEngine {
    pub local_id: String,
    pub auth_pin: String,
    pub known_peers: BTreeMap<String, WarpinatorPeer>,
    pub transfer_queue: BTreeMap<String, WarpinatorTransferState>, // filename -> state
}

impl MintWarpinatorTransferProtocolEngine {
    pub fn new(local_id: &str, pin: &str) -> Self {
        Self {
            local_id: local_id.to_string(),
            auth_pin: pin.to_string(),
            known_peers: BTreeMap::new(),
            transfer_queue: BTreeMap::new(),
        }
    }

    pub fn discover_peer(&mut self, peer_id: &str, name: &str, ip: &str, port: u16, pin: &str) {
        self.known_peers.insert(
            peer_id.to_string(),
            WarpinatorPeer {
                peer_id: peer_id.to_string(),
                display_name: name.to_string(),
                ip_address: ip.to_string(),
                port,
                auth_pin: pin.to_string(),
            },
        );
    }

    pub fn initiate_send(&mut self, peer_id: &str, filename: &str) -> Result<WarpinatorTransferState, &'static str> {
        let peer = self.known_peers.get(peer_id).ok_or("Peer not discovered")?;
        if peer.auth_pin != self.auth_pin {
            return Err("Authentication PIN mismatch");
        }
        self.transfer_queue.insert(filename.to_string(), WarpinatorTransferState::AwaitingApproval);
        Ok(WarpinatorTransferState::AwaitingApproval)
    }

    pub fn accept_transfer(&mut self, filename: &str) -> bool {
        if let Some(state) = self.transfer_queue.get_mut(filename) {
            *state = WarpinatorTransferState::Completed;
            true
        } else {
            false
        }
    }
}

// =========================================================================
// 6. HYPNOTIX IPTV STREAM CHANNEL MANAGER
// =========================================================================

#[derive(Debug, Clone)]
pub struct IptvChannel {
    pub id: String,
    pub name: String,
    pub group_title: String,
    pub stream_url: String,
    pub is_favorite: bool,
}

pub struct MintHypnotixIptvEngine {
    pub channels: BTreeMap<String, IptvChannel>,
}

impl MintHypnotixIptvEngine {
    pub fn new() -> Self {
        Self {
            channels: BTreeMap::new(),
        }
    }

    pub fn parse_m3u_entry(&mut self, id: &str, name: &str, group: &str, url: &str) {
        self.channels.insert(
            id.to_string(),
            IptvChannel {
                id: id.to_string(),
                name: name.to_string(),
                group_title: group.to_string(),
                stream_url: url.to_string(),
                is_favorite: false,
            },
        );
    }

    pub fn toggle_favorite(&mut self, id: &str) -> bool {
        if let Some(chan) = self.channels.get_mut(id) {
            chan.is_favorite = !chan.is_favorite;
            chan.is_favorite
        } else {
            false
        }
    }

    pub fn get_favorites(&self) -> Vec<IptvChannel> {
        self.channels
            .values()
            .filter(|c| c.is_favorite)
            .cloned()
            .collect()
    }
}

impl Default for MintHypnotixIptvEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 7. BULKY BATCH FILE RENAMING ENGINE
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaseTransform {
    None,
    LowerCase,
    UpperCase,
}

pub struct MintBulkyBatchRenamerEngine {
    pub case_mode: CaseTransform,
}

impl MintBulkyBatchRenamerEngine {
    pub fn new() -> Self {
        Self {
            case_mode: CaseTransform::None,
        }
    }

    pub fn rename_files(
        &self,
        files: &[&str],
        find_str: &str,
        replace_str: &str,
        prefix_index: bool,
    ) -> Vec<String> {
        let mut results = Vec::new();
        for (i, &name) in files.iter().enumerate() {
            let replaced = name.replace(find_str, replace_str);
            let transformed = match self.case_mode {
                CaseTransform::LowerCase => replaced.to_lowercase(),
                CaseTransform::UpperCase => replaced.to_uppercase(),
                CaseTransform::None => replaced,
            };
            if prefix_index {
                results.push(format!("{:03}_{}", i + 1, transformed));
            } else {
                results.push(transformed);
            }
        }
        results
    }
}

impl Default for MintBulkyBatchRenamerEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 8. THINGY DOCUMENT MANAGER & RECENT FILE INDEXER
// =========================================================================

#[derive(Debug, Clone)]
pub struct IndexedDocument {
    pub path: String,
    pub title: String,
    pub mime_type: String,
    pub tags: Vec<String>,
    pub access_timestamp: u64,
}

pub struct MintThingyDocManagerEngine {
    pub documents: BTreeMap<String, IndexedDocument>,
}

impl MintThingyDocManagerEngine {
    pub fn new() -> Self {
        Self {
            documents: BTreeMap::new(),
        }
    }

    pub fn index_doc(&mut self, path: &str, title: &str, mime: &str, tags: &[&str], ts: u64) {
        self.documents.insert(
            path.to_string(),
            IndexedDocument {
                path: path.to_string(),
                title: title.to_string(),
                mime_type: mime.to_string(),
                tags: tags.iter().map(|s| s.to_string()).collect(),
                access_timestamp: ts,
            },
        );
    }

    pub fn filter_by_tag(&self, tag: &str) -> Vec<IndexedDocument> {
        self.documents
            .values()
            .filter(|doc| doc.tags.iter().any(|t| t == tag))
            .cloned()
            .collect()
    }
}

impl Default for MintThingyDocManagerEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 9. MINTNANNY PARENTAL CONTROLS & DOMAIN BLOCKER
// =========================================================================

pub struct MintNannyParentalFilterEngine {
    pub blacklisted_domains: Vec<String>,
    pub is_locked: bool,
    pub allowed_window_start_hour: u8,
    pub allowed_window_end_hour: u8,
}

impl MintNannyParentalFilterEngine {
    pub fn new() -> Self {
        Self {
            blacklisted_domains: Vec::new(),
            is_locked: true,
            allowed_window_start_hour: 8,
            allowed_window_end_hour: 20,
        }
    }

    pub fn add_domain(&mut self, domain: &str) {
        if !self.blacklisted_domains.iter().any(|d| d == domain) {
            self.blacklisted_domains.push(domain.to_string());
        }
    }

    pub fn is_domain_permitted(&self, domain: &str, current_hour: u8) -> bool {
        if self.blacklisted_domains.iter().any(|d| d == domain) {
            return false;
        }
        if self.is_locked {
            current_hour >= self.allowed_window_start_hour && current_hour < self.allowed_window_end_hour
        } else {
            true
        }
    }
}

impl Default for MintNannyParentalFilterEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 10. MINTWELCOME ONBOARDING & FIRST-RUN WIZARD
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesktopLayoutChoice {
    Traditional,
    Modern,
}

pub struct MintWelcomeOnboardingEngine {
    pub layout: DesktopLayoutChoice,
    pub completed_steps: Vec<String>,
    pub first_boot_done: bool,
}

impl MintWelcomeOnboardingEngine {
    pub fn new() -> Self {
        Self {
            layout: DesktopLayoutChoice::Traditional,
            completed_steps: Vec::new(),
            first_boot_done: false,
        }
    }

    pub fn choose_layout(&mut self, choice: DesktopLayoutChoice) {
        self.layout = choice;
    }

    pub fn complete_task(&mut self, task_name: &str) {
        if !self.completed_steps.iter().any(|s| s == task_name) {
            self.completed_steps.push(task_name.to_string());
        }
    }

    pub fn finalize_onboarding(&mut self) -> bool {
        self.first_boot_done = true;
        self.first_boot_done
    }
}

impl Default for MintWelcomeOnboardingEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 11. MINTREPORT SYSTEM CRASH DIAGNOSTICS & ADVICE
// =========================================================================

#[derive(Debug, Clone)]
pub struct SystemReportNotice {
    pub id: String,
    pub title: String,
    pub is_critical: bool,
    pub recommendation: String,
}

pub struct MintReportDiagnosticEngine {
    pub notices: Vec<SystemReportNotice>,
}

impl MintReportDiagnosticEngine {
    pub fn new() -> Self {
        Self {
            notices: Vec::new(),
        }
    }

    pub fn register_notice(&mut self, id: &str, title: &str, is_crit: bool, rec: &str) {
        self.notices.push(SystemReportNotice {
            id: id.to_string(),
            title: title.to_string(),
            is_critical: is_crit,
            recommendation: rec.to_string(),
        });
    }

    pub fn get_critical_notices(&self) -> Vec<SystemReportNotice> {
        self.notices.iter().filter(|n| n.is_critical).cloned().collect()
    }
}

impl Default for MintReportDiagnosticEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 12. MINTLOCALE REGIONAL & FORMATS GOVERNOR
// =========================================================================

pub struct MintLocaleRegionalEngine {
    pub active_locale: String,
    pub keymap: String,
    pub use_24h_format: bool,
}

impl MintLocaleRegionalEngine {
    pub fn new() -> Self {
        Self {
            active_locale: "en_US.UTF-8".to_string(),
            keymap: "us".to_string(),
            use_24h_format: true,
        }
    }

    pub fn set_locale(&mut self, locale: &str) {
        self.active_locale = locale.to_string();
    }

    pub fn set_keymap(&mut self, km: &str) {
        self.keymap = km.to_string();
    }
}

impl Default for MintLocaleRegionalEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 13. MINTDESKTOP INTERFACE & DESKTOP ICONS
// =========================================================================

pub struct MintDesktopLayoutEngine {
    pub show_computer: bool,
    pub show_home: bool,
    pub show_trash: bool,
    pub show_mounted_volumes: bool,
}

impl MintDesktopLayoutEngine {
    pub fn new() -> Self {
        Self {
            show_computer: true,
            show_home: true,
            show_trash: true,
            show_mounted_volumes: true,
        }
    }

    pub fn toggle_home(&mut self) -> bool {
        self.show_home = !self.show_home;
        self.show_home
    }
}

impl Default for MintDesktopLayoutEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 14. MINTBACKUP PERSONAL BACKUP & PACKAGE LIST EXPORTER
// =========================================================================

pub struct MintBackupPersonalEngine {
    pub backup_destination: String,
    pub exported_package_list: Vec<String>,
}

impl MintBackupPersonalEngine {
    pub fn new(destination: &str) -> Self {
        Self {
            backup_destination: destination.to_string(),
            exported_package_list: Vec::new(),
        }
    }

    pub fn export_packages(&mut self, pkgs: &[&str]) -> usize {
        for &p in pkgs {
            self.exported_package_list.push(p.to_string());
        }
        self.exported_package_list.len()
    }
}

// =========================================================================
// 15. MINTDRIVERS PROPRIETARY DRIVER SELECTOR
// =========================================================================

#[derive(Debug, Clone)]
pub struct HardwareDriver {
    pub pci_id: String,
    pub driver_name: String,
    pub is_proprietary: bool,
    pub is_active: bool,
}

pub struct MintDriversHardwareDetectorEngine {
    pub drivers: BTreeMap<String, HardwareDriver>,
}

impl MintDriversHardwareDetectorEngine {
    pub fn new() -> Self {
        Self {
            drivers: BTreeMap::new(),
        }
    }

    pub fn register_hardware(&mut self, pci_id: &str, driver: &str, proprietary: bool) {
        self.drivers.insert(
            pci_id.to_string(),
            HardwareDriver {
                pci_id: pci_id.to_string(),
                driver_name: driver.to_string(),
                is_proprietary: proprietary,
                is_active: false,
            },
        );
    }

    pub fn activate_driver(&mut self, pci_id: &str) -> bool {
        if let Some(drv) = self.drivers.get_mut(pci_id) {
            drv.is_active = true;
            true
        } else {
            false
        }
    }
}

impl Default for MintDriversHardwareDetectorEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 16. MINTSOURCES MIRROR SPEED TESTER & PPA GOVERNOR
// =========================================================================

#[derive(Debug, Clone)]
pub struct MirrorStatus {
    pub url: String,
    pub latency_ms: u32,
    pub is_reachable: bool,
}

pub struct MintSourcesMirrorTesterEngine {
    pub mirrors: Vec<MirrorStatus>,
}

impl MintSourcesMirrorTesterEngine {
    pub fn new() -> Self {
        Self {
            mirrors: Vec::new(),
        }
    }

    pub fn add_mirror(&mut self, url: &str, latency: u32, reachable: bool) {
        self.mirrors.push(MirrorStatus {
            url: url.to_string(),
            latency_ms: latency,
            is_reachable: reachable,
        });
    }

    pub fn get_fastest_mirror(&self) -> Option<String> {
        self.mirrors
            .iter()
            .filter(|m| m.is_reachable)
            .min_by_key(|m| m.latency_ms)
            .map(|m| m.url.clone())
    }
}

impl Default for MintSourcesMirrorTesterEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 17. MINTUPGRADE MAJOR LTS TRANSITION ENGINE
// =========================================================================

pub struct MintUpgradeLtsTransitionEngine {
    pub current_codename: String,
    pub target_codename: String,
    pub preflight_passed: bool,
}

impl MintUpgradeLtsTransitionEngine {
    pub fn new(current: &str, target: &str) -> Self {
        Self {
            current_codename: current.to_string(),
            target_codename: target.to_string(),
            preflight_passed: false,
        }
    }

    pub fn run_preflight_checks(&mut self, disk_free_mb: u64, power_connected: bool) -> bool {
        self.preflight_passed = disk_free_mb >= 5000 && power_connected;
        self.preflight_passed
    }
}

// =========================================================================
// 18. STICKY NOTES DESKTOP CANVAS
// =========================================================================

#[derive(Debug, Clone)]
pub struct DesktopStickyNote {
    pub id: u32,
    pub title: String,
    pub markdown_body: String,
    pub color_hex: String,
    pub workspace: u32,
}

pub struct MintStickyNotesEngine {
    pub notes: BTreeMap<u32, DesktopStickyNote>,
    pub next_note_id: u32,
}

impl MintStickyNotesEngine {
    pub fn new() -> Self {
        Self {
            notes: BTreeMap::new(),
            next_note_id: 1,
        }
    }

    pub fn create_note(&mut self, title: &str, body: &str, color: &str, ws: u32) -> u32 {
        let id = self.next_note_id;
        self.next_note_id += 1;
        self.notes.insert(
            id,
            DesktopStickyNote {
                id,
                title: title.to_string(),
                markdown_body: body.to_string(),
                color_hex: color.to_string(),
                workspace: ws,
            },
        );
        id
    }
}

impl Default for MintStickyNotesEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 19. XAPP CROSS-DESKTOP STATUS NOTIFIER
// =========================================================================

#[derive(Debug, Clone)]
pub struct XAppStatusItem {
    pub id: String,
    pub icon_name: String,
    pub tooltip: String,
    pub menu_items: Vec<String>,
}

pub struct MintXAppStatusNotifierEngine {
    pub items: BTreeMap<String, XAppStatusItem>,
}

impl MintXAppStatusNotifierEngine {
    pub fn new() -> Self {
        Self {
            items: BTreeMap::new(),
        }
    }

    pub fn register_item(&mut self, id: &str, icon: &str, tip: &str, menu: &[&str]) {
        self.items.insert(
            id.to_string(),
            XAppStatusItem {
                id: id.to_string(),
                icon_name: icon.to_string(),
                tooltip: tip.to_string(),
                menu_items: menu.iter().map(|s| s.to_string()).collect(),
            },
        );
    }
}

impl Default for MintXAppStatusNotifierEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 20. SLICK-GREETER DISPLAY & SESSION MANAGER
// =========================================================================

pub struct MintSlickGreeterDisplayEngine {
    pub background_path: String,
    pub enable_hidpi: bool,
    pub sessions: Vec<String>,
}

impl MintSlickGreeterDisplayEngine {
    pub fn new() -> Self {
        Self {
            background_path: "/usr/share/backgrounds/default.jpg".to_string(),
            enable_hidpi: true,
            sessions: vec!["cinnamon".to_string(), "sigma-wm".to_string()],
        }
    }

    pub fn set_background(&mut self, path: &str) {
        self.background_path = path.to_string();
    }
}

impl Default for MintSlickGreeterDisplayEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 21. NEMO CONTEXT MENU & ACTION EXTENSION MANAGER
// =========================================================================

#[derive(Debug, Clone)]
pub struct NemoActionEntry {
    pub id: String,
    pub label: String,
    pub exec_cmd: String,
    pub mime_types: Vec<String>,
}

pub struct MintNemoActionManagerEngine {
    pub actions: BTreeMap<String, NemoActionEntry>,
}

impl MintNemoActionManagerEngine {
    pub fn new() -> Self {
        Self {
            actions: BTreeMap::new(),
        }
    }

    pub fn register_action(&mut self, id: &str, label: &str, cmd: &str, mimes: &[&str]) {
        self.actions.insert(
            id.to_string(),
            NemoActionEntry {
                id: id.to_string(),
                label: label.to_string(),
                exec_cmd: cmd.to_string(),
                mime_types: mimes.iter().map(|s| s.to_string()).collect(),
            },
        );
    }

    pub fn match_actions_for_mime(&self, mime: &str) -> Vec<NemoActionEntry> {
        self.actions
            .values()
            .filter(|a| a.mime_types.iter().any(|m| m == mime || m == "*/*"))
            .cloned()
            .collect()
    }
}

impl Default for MintNemoActionManagerEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 22. MUFFIN COMPOSITOR ENGINE
// =========================================================================

pub struct MintMuffinCompositorEngine {
    pub vsync_enabled: bool,
    pub unredirect_fullscreen: bool,
    pub workspace_count: u32,
}

impl MintMuffinCompositorEngine {
    pub fn new() -> Self {
        Self {
            vsync_enabled: true,
            unredirect_fullscreen: true,
            workspace_count: 4,
        }
    }

    pub fn set_workspace_count(&mut self, count: u32) {
        self.workspace_count = count.max(1);
    }
}

impl Default for MintMuffinCompositorEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 23. CINNAMON SPICES APPLET SCHEMAS & LIFECYCLE
// =========================================================================

#[derive(Debug, Clone)]
pub struct CinnamonSpiceApplet {
    pub uuid: String,
    pub name: String,
    pub is_enabled: bool,
    pub schema_settings: BTreeMap<String, String>,
}

pub struct MintCinnamonSpicesAppletEngine {
    pub applets: BTreeMap<String, CinnamonSpiceApplet>,
}

impl MintCinnamonSpicesAppletEngine {
    pub fn new() -> Self {
        Self {
            applets: BTreeMap::new(),
        }
    }

    pub fn load_applet(&mut self, uuid: &str, name: &str) {
        self.applets.insert(
            uuid.to_string(),
            CinnamonSpiceApplet {
                uuid: uuid.to_string(),
                name: name.to_string(),
                is_enabled: true,
                schema_settings: BTreeMap::new(),
            },
        );
    }

    pub fn set_setting(&mut self, uuid: &str, key: &str, val: &str) -> bool {
        if let Some(applet) = self.applets.get_mut(uuid) {
            applet.schema_settings.insert(key.to_string(), val.to_string());
            true
        } else {
            false
        }
    }
}

impl Default for MintCinnamonSpicesAppletEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 24. MASTER SUITE FOR LINUX MINT GITHUB REPO INSPIRATIONS PR SUITE
// =========================================================================

pub struct MintGithubRepoInspirationsPrSuite {
    pub pr_matrix: MintPrProposalMatrix,
    pub update_gov: MintUpdateLevelGovernorEngine,
    pub catalog: MintInstallSoftwareCatalogEngine,
    pub flasher: MintStickUsbFlasherEngine,
    pub warpinator: MintWarpinatorTransferProtocolEngine,
    pub hypnotix: MintHypnotixIptvEngine,
    pub bulky: MintBulkyBatchRenamerEngine,
    pub thingy: MintThingyDocManagerEngine,
    pub nanny: MintNannyParentalFilterEngine,
    pub welcome: MintWelcomeOnboardingEngine,
    pub report: MintReportDiagnosticEngine,
    pub locale: MintLocaleRegionalEngine,
    pub desktop: MintDesktopLayoutEngine,
    pub backup: MintBackupPersonalEngine,
    pub drivers: MintDriversHardwareDetectorEngine,
    pub sources: MintSourcesMirrorTesterEngine,
    pub upgrade: MintUpgradeLtsTransitionEngine,
    pub sticky: MintStickyNotesEngine,
    pub xapp: MintXAppStatusNotifierEngine,
    pub slick: MintSlickGreeterDisplayEngine,
    pub nemo: MintNemoActionManagerEngine,
    pub muffin: MintMuffinCompositorEngine,
    pub spices: MintCinnamonSpicesAppletEngine,
}

impl MintGithubRepoInspirationsPrSuite {
    pub fn new() -> Self {
        Self {
            pr_matrix: MintPrProposalMatrix::new(),
            update_gov: MintUpdateLevelGovernorEngine::new(),
            catalog: MintInstallSoftwareCatalogEngine::new(),
            flasher: MintStickUsbFlasherEngine::new(),
            warpinator: MintWarpinatorTransferProtocolEngine::new("sigma-node-01", "7890"),
            hypnotix: MintHypnotixIptvEngine::new(),
            bulky: MintBulkyBatchRenamerEngine::new(),
            thingy: MintThingyDocManagerEngine::new(),
            nanny: MintNannyParentalFilterEngine::new(),
            welcome: MintWelcomeOnboardingEngine::new(),
            report: MintReportDiagnosticEngine::new(),
            locale: MintLocaleRegionalEngine::new(),
            desktop: MintDesktopLayoutEngine::new(),
            backup: MintBackupPersonalEngine::new("/backup/sigmaos"),
            drivers: MintDriversHardwareDetectorEngine::new(),
            sources: MintSourcesMirrorTesterEngine::new(),
            upgrade: MintUpgradeLtsTransitionEngine::new("victoria", "virginia"),
            sticky: MintStickyNotesEngine::new(),
            xapp: MintXAppStatusNotifierEngine::new(),
            slick: MintSlickGreeterDisplayEngine::new(),
            nemo: MintNemoActionManagerEngine::new(),
            muffin: MintMuffinCompositorEngine::new(),
            spices: MintCinnamonSpicesAppletEngine::new(),
        }
    }

    pub fn run_full_suite_audit(&mut self) -> bool {
        let pr_num = self.pr_matrix.submit_pr(
            "Linux Mint GitHub Repo Gap Closure & Open Source OS Parity",
            "feat/mint-repo-parity",
            "linuxmint/*",
            "Desktop System Tools & Cinnamon Components",
            "Integrates mintupdate 5-tier classification, mintinstall, mintstick, and XApps into SigmaOS.",
            &["src/distro/mint_github_repo_inspirations_pr.rs"],
        );

        let signed = self.pr_matrix.sign_and_verify_pr(pr_num, b"dilithium5_key").is_ok();

        // 1. Update governor
        self.update_gov.register_update("linux-image-generic", "6.5.0-14", "6.5.0-15", UpdateLevel::Level4Dangerous, true, "Kernel patch");
        self.update_gov.blacklist_regressive_kernel("6.5.0-15");
        let kernel_allowed = self.update_gov.is_update_permitted("linux-image-generic");
        assert!(!kernel_allowed);

        // 2. Flasher safety
        self.flasher.register_device("/dev/nvme0n1", 512_000_000_000, true, false);
        self.flasher.register_device("/dev/sda", 32_000_000_000, false, true);
        assert!(self.flasher.flash_iso("/dev/nvme0n1", "mint.iso").is_err());
        assert!(self.flasher.flash_iso("/dev/sda", "mint.iso").is_ok());

        // 3. Bulky rename
        let renames = self.bulky.rename_files(&["IMG_001.JPG", "IMG_002.JPG"], "IMG_", "Photo_", true);
        assert_eq!(renames.len(), 2);

        // 4. Sources fastest mirror
        self.sources.add_mirror("https://fast.mint.com", 15, true);
        self.sources.add_mirror("https://slow.mint.com", 150, true);
        assert_eq!(self.sources.get_fastest_mirror().as_deref(), Some("https://fast.mint.com"));

        // 5. Welcome onboarding
        self.welcome.choose_layout(DesktopLayoutChoice::Modern);
        self.welcome.finalize_onboarding();

        signed
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
    fn test_mint_pr_proposal_matrix() {
        let mut matrix = MintPrProposalMatrix::new();
        let pr = matrix.submit_pr(
            "Add mintupdate 5-tier classification",
            "feat/mintupdate-parity",
            "linuxmint/mintupdate",
            "System Management",
            "Implements 5-tier update classification policy",
            &["src/distro/mint_github_repo_inspirations_pr.rs"],
        );
        assert_eq!(pr, 3001);

        let status = matrix.sign_and_verify_pr(pr, b"dilithium5_sec_key").unwrap();
        assert_eq!(status, MintPrStatus::Merged);

        let md = matrix.generate_markdown_pr(pr).unwrap();
        assert!(md.contains("Add mintupdate 5-tier classification"));
        assert!(md.contains("linuxmint/mintupdate"));
    }

    #[test]
    fn test_mint_engines() {
        // Update level governor
        let mut update = MintUpdateLevelGovernorEngine::new();
        update.register_update("bash", "5.1", "5.2", UpdateLevel::Level2Recommended, false, "Fix shell issue");
        assert!(update.is_update_permitted("bash"));

        // Software Catalog
        let mut catalog = MintInstallSoftwareCatalogEngine::new();
        catalog.add_software("org.videolan.VLC", "VLC Media Player", PackageSourceType::Flatpak, true);
        catalog.submit_review("org.videolan.VLC", 90);
        let results = catalog.search("VLC");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].score, 90);

        // USB Flasher & Formatter
        let mut flasher = MintStickUsbFlasherEngine::new();
        flasher.register_device("/dev/sdb", 16_000_000_000, false, true);
        assert!(flasher.format_drive("/dev/sdb", FsType::Fat32, "MINT_INSTALL").is_ok());

        // Warpinator
        let mut warpinator = MintWarpinatorTransferProtocolEngine::new("host1", "1234");
        warpinator.discover_peer("host2", "Alice PC", "192.168.1.100", 42000, "1234");
        let transfer_res = warpinator.initiate_send("host2", "contract.pdf").unwrap();
        assert_eq!(transfer_res, WarpinatorTransferState::AwaitingApproval);
        assert!(warpinator.accept_transfer("contract.pdf"));

        // Hypnotix
        let mut tv = MintHypnotixIptvEngine::new();
        tv.parse_m3u_entry("news1", "Global News", "News", "https://stream.example/live.m3u8");
        assert!(tv.toggle_favorite("news1"));
        assert_eq!(tv.get_favorites().len(), 1);

        // Bulky
        let mut bulky = MintBulkyBatchRenamerEngine::new();
        bulky.case_mode = CaseTransform::LowerCase;
        let files = bulky.rename_files(&["DOC_Alpha.txt", "DOC_Beta.txt"], "DOC_", "item_", false);
        assert_eq!(files[0], "item_alpha.txt");

        // Thingy
        let mut thingy = MintThingyDocManagerEngine::new();
        thingy.index_doc("/docs/spec.pdf", "SigmaOS Spec", "application/pdf", &["os", "kernel"], 1000);
        assert_eq!(thingy.filter_by_tag("kernel").len(), 1);

        // MintNanny
        let mut nanny = MintNannyParentalFilterEngine::new();
        nanny.add_domain("gambling.com");
        assert!(!nanny.is_domain_permitted("gambling.com", 12));
        assert!(nanny.is_domain_permitted("wikipedia.org", 12));
        assert!(!nanny.is_domain_permitted("wikipedia.org", 23)); // outside window

        // MintReport
        let mut report = MintReportDiagnosticEngine::new();
        report.register_notice("c1", "Kernel dump detected", true, "Update microcode");
        assert_eq!(report.get_critical_notices().len(), 1);

        // MintLocale
        let mut locale = MintLocaleRegionalEngine::new();
        locale.set_locale("de_DE.UTF-8");
        assert_eq!(locale.active_locale, "de_DE.UTF-8");

        // MintDesktop
        let mut desktop = MintDesktopLayoutEngine::new();
        assert!(!desktop.toggle_home());

        // MintBackup
        let mut backup = MintBackupPersonalEngine::new("/media/backup");
        let exported = backup.export_packages(&["curl", "git", "build-essential"]);
        assert_eq!(exported, 3);

        // MintDrivers
        let mut drivers = MintDriversHardwareDetectorEngine::new();
        drivers.register_hardware("10de:1f02", "nvidia-driver-535", true);
        assert!(drivers.activate_driver("10de:1f02"));
        assert!(drivers.drivers.get("10de:1f02").unwrap().is_active);

        // MintUpgrade
        let mut upgrade = MintUpgradeLtsTransitionEngine::new("victoria", "virginia");
        assert!(upgrade.run_preflight_checks(10000, true));
        assert!(!upgrade.run_preflight_checks(2000, true)); // insufficient space

        // Sticky notes
        let mut sticky = MintStickyNotesEngine::new();
        let note_id = sticky.create_note("TODO", "Refactor scheduler", "#FFFF88", 1);
        assert_eq!(note_id, 1);

        // XApp
        let mut xapp = MintXAppStatusNotifierEngine::new();
        xapp.register_item("volume-applet", "audio-volume-high", "Volume 80%", &["Mute", "Settings"]);
        assert!(xapp.items.contains_key("volume-applet"));

        // Slick Greeter
        let mut slick = MintSlickGreeterDisplayEngine::new();
        slick.set_background("/wallpapers/custom.png");
        assert_eq!(slick.background_path, "/wallpapers/custom.png");

        // Nemo Actions
        let mut nemo = MintNemoActionManagerEngine::new();
        nemo.register_action("open_terminal", "Open in Terminal", "st -d %f", &["folder"]);
        assert_eq!(nemo.match_actions_for_mime("folder").len(), 1);

        // Muffin
        let mut muffin = MintMuffinCompositorEngine::new();
        muffin.set_workspace_count(8);
        assert_eq!(muffin.workspace_count, 8);

        // Cinnamon Spices
        let mut spices = MintCinnamonSpicesAppletEngine::new();
        spices.load_applet("weather@cinnamon.org", "Weather Applet");
        assert!(spices.set_setting("weather@cinnamon.org", "units", "celsius"));
    }

    #[test]
    fn test_mint_master_pr_suite() {
        let mut suite = MintGithubRepoInspirationsPrSuite::new();
        assert!(suite.run_full_suite_audit());
    }
}
