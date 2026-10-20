// SigmaOS Gentoo Portage Implementation
// Implements Gentoo-style source-based package management for SigmaOS
// Inspired by Gentoo's Portage for performance optimization and customization

extern crate alloc;

use alloc::collections::{BTreeMap, BTreeSet};
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

/// Portage error types
#[derive(Debug, Clone)]
pub enum PortageError {
    PackageNotFound,
    DependencyResolutionFailed,
    FetchFailed,
    CompilationFailed,
    InstallationFailed,
    ProfileNotFound,
    InvalidUseFlag,
}

/// USE flag type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UseFlagType {
    Global,
    Local,
    Expander,
    Architecture,
}

/// USE flag
#[derive(Debug, Clone)]
pub struct UseFlag {
    pub name: String,
    pub description: String,
    pub flag_type: UseFlagType,
    pub default: bool,
}

/// Ebuild metadata
#[derive(Debug, Clone)]
pub struct Ebuild {
    pub name: String,
    pub version: String,
    pub slot: String,
    pub description: String,
    pub homepage: String,
    pub license: String,
    pub iuse: Vec<String>,
    pub depends: Vec<String>,
    pub rdepends: Vec<String>,
    pub pdepends: Vec<String>,
    pub src_uri: Vec<String>,
}

/// Dependency specification
#[derive(Debug, Clone)]
pub struct DependencySpec {
    pub depends: Vec<String>,
    pub rdepends: Vec<String>,
    pub pdepends: Vec<String>,
}

/// USE flags manager
pub struct UseFlagManager {
    pub global_flags: BTreeMap<String, bool>,
    pub package_flags: BTreeMap<String, BTreeMap<String, bool>>,
    pub profile_flags: BTreeMap<String, bool>,
}

impl UseFlagManager {
    pub fn new() -> Self {
        Self {
            global_flags: BTreeMap::new(),
            package_flags: BTreeMap::new(),
            profile_flags: BTreeMap::new(),
        }
    }

    /// Enable global flag
    pub fn enable_global_flag(&mut self, flag: String) {
        self.global_flags.insert(flag, true);
    }

    /// Disable global flag
    pub fn disable_global_flag(&mut self, flag: String) {
        self.global_flags.insert(flag, false);
    }

    /// Enable package-specific flag
    pub fn enable_package_flag(&mut self, package: String, flag: String) {
        self.package_flags
            .entry(package)
            .or_insert_with(BTreeMap::new)
            .insert(flag, true);
    }

    /// Disable package-specific flag
    pub fn disable_package_flag(&mut self, package: String, flag: String) {
        self.package_flags
            .entry(package)
            .or_insert_with(BTreeMap::new)
            .insert(flag, false);
    }

    /// Resolve USE flags for package
    pub fn resolve_for_package(&self, ebuild: &Ebuild) -> Result<Vec<String>, PortageError> {
        let mut resolved = Vec::new();

        // Start with global flags
        for (flag, enabled) in &self.global_flags {
            if *enabled && ebuild.iuse.contains(flag) {
                resolved.push(flag.clone());
            }
        }

        // Add package-specific flags
        if let Some(pkg_flags) = self.package_flags.get(&ebuild.name) {
            for (flag, enabled) in pkg_flags {
                if *enabled && ebuild.iuse.contains(flag) {
                    if !resolved.contains(flag) {
                        resolved.push(flag.clone());
                    }
                }
            }
        }

        Ok(resolved)
    }
}

impl Default for UseFlagManager {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// GENTOO PORTAGE ADVANCEMENTS & PARITY ENGINES
// ==========================================

/// Gentoo Package Set Kind (@world, @system, @selected, @profile, or custom @set)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GentooPackageSetKind {
    World,
    System,
    Selected,
    Profile,
    Custom(String),
}

/// Gentoo Portage Package Sets Manager (`@world`, `@system`, `@selected`)
pub struct GentooPackageSetManager {
    pub sets: BTreeMap<String, BTreeSet<String>>,
}

impl GentooPackageSetManager {
    pub fn new() -> Self {
        let mut mgr = Self {
            sets: BTreeMap::new(),
        };
        // Seed standard Gentoo sets
        mgr.sets.insert("system".to_string(), BTreeSet::new());
        mgr.sets.insert("selected".to_string(), BTreeSet::new());
        mgr.sets.insert("world".to_string(), BTreeSet::new());

        // Populate standard system set defaults
        mgr.add_to_set("system", "sys-apps/baselayout");
        mgr.add_to_set("system", "sys-apps/portage");
        mgr.add_to_set("system", "sys-libs/glibc");
        mgr
    }

    pub fn add_to_set(&mut self, set_name: &str, package: &str) {
        let set = self.sets.entry(set_name.to_string()).or_default();
        set.insert(package.to_string());

        if set_name == "selected" || set_name == "system" {
            // World set includes both system and explicitly selected packages
            if let Some(world) = self.sets.get_mut("world") {
                world.insert(package.to_string());
            }
        }
    }

    pub fn remove_from_set(&mut self, set_name: &str, package: &str) -> bool {
        if let Some(set) = self.sets.get_mut(set_name) {
            let removed = set.remove(package);
            if removed && set_name == "selected" {
                // Rebuild world set from remaining selected + system
                let system_pkgs = self.sets.get("system").cloned().unwrap_or_default();
                let selected_pkgs = self.sets.get("selected").cloned().unwrap_or_default();
                if let Some(world) = self.sets.get_mut("world") {
                    *world = system_pkgs.union(&selected_pkgs).cloned().collect();
                }
            }
            removed
        } else {
            false
        }
    }

    pub fn get_set(&self, set_name: &str) -> Vec<String> {
        self.sets
            .get(set_name)
            .map(|s| s.iter().cloned().collect())
            .unwrap_or_default()
    }
}

impl Default for GentooPackageSetManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Gentoo `etc-update` / `dispatch-conf` Configuration Merge Action
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigMergeAction {
    UseNew,
    KeepCurrent,
    InteractiveMerge,
    DiffSummary,
}

/// Pending Configuration Update Entry (e.g. `._cfg0000_net.conf`)
#[derive(Debug, Clone)]
pub struct PendingConfigUpdate {
    pub target_file: String,
    pub pending_file: String,
    pub current_content: String,
    pub new_content: String,
}

/// Gentoo `etc-update` / `dispatch-conf` Engine
pub struct GentooConfigMergeEngine {
    pub pending_updates: Vec<PendingConfigUpdate>,
}

impl GentooConfigMergeEngine {
    pub fn new() -> Self {
        Self {
            pending_updates: Vec::new(),
        }
    }

    pub fn register_pending_update(
        &mut self,
        target_file: &str,
        current_content: &str,
        new_content: &str,
    ) {
        let pending_file = format!("._cfg0000_{}", target_file);
        self.pending_updates.push(PendingConfigUpdate {
            target_file: target_file.to_string(),
            pending_file,
            current_content: current_content.to_string(),
            new_content: new_content.to_string(),
        });
    }

    pub fn count_pending(&self) -> usize {
        self.pending_updates.len()
    }

    pub fn resolve_update(
        &mut self,
        target_file: &str,
        action: ConfigMergeAction,
    ) -> Result<String, PortageError> {
        let pos = self
            .pending_updates
            .iter()
            .position(|u| u.target_file == target_file)
            .ok_or(PortageError::PackageNotFound)?;

        let update = self.pending_updates.remove(pos);
        match action {
            ConfigMergeAction::UseNew => Ok(update.new_content),
            ConfigMergeAction::KeepCurrent => Ok(update.current_content),
            ConfigMergeAction::InteractiveMerge | ConfigMergeAction::DiffSummary => {
                let merged = format!(
                    "# --- MERGED CONFIG ({}) ---\n{}\n# --- END MERGED ---",
                    target_file, update.new_content
                );
                Ok(merged)
            }
        }
    }
}

impl Default for GentooConfigMergeEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Gentoo `eclean` Source Distfile & Binary Package Cleanup Engine
pub struct GentooEcleanEngine {
    pub distfiles: BTreeMap<String, (u64, bool)>, // filename -> (size_bytes, is_obsolete)
    pub binpkgs: BTreeMap<String, (u64, bool)>,  // pkg_tarball -> (size_bytes, is_obsolete)
}

impl GentooEcleanEngine {
    pub fn new() -> Self {
        Self {
            distfiles: BTreeMap::new(),
            binpkgs: BTreeMap::new(),
        }
    }

    pub fn register_distfile(&mut self, filename: &str, size_bytes: u64, is_obsolete: bool) {
        self.distfiles
            .insert(filename.to_string(), (size_bytes, is_obsolete));
    }

    pub fn register_binpkg(&mut self, pkg_name: &str, size_bytes: u64, is_obsolete: bool) {
        self.binpkgs
            .insert(pkg_name.to_string(), (size_bytes, is_obsolete));
    }

    pub fn eclean_distfiles(&mut self) -> (usize, u64) {
        let obsolete_keys: Vec<String> = self
            .distfiles
            .iter()
            .filter(|(_, (_, obs))| *obs)
            .map(|(k, _)| k.clone())
            .collect();

        let mut freed_bytes = 0u64;
        let freed_count = obsolete_keys.len();
        for k in obsolete_keys {
            if let Some((sz, _)) = self.distfiles.remove(&k) {
                freed_bytes += sz;
            }
        }
        (freed_count, freed_bytes)
    }

    pub fn eclean_pkg(&mut self) -> (usize, u64) {
        let obsolete_keys: Vec<String> = self
            .binpkgs
            .iter()
            .filter(|(_, (_, obs))| *obs)
            .map(|(k, _)| k.clone())
            .collect();

        let mut freed_bytes = 0u64;
        let freed_count = obsolete_keys.len();
        for k in obsolete_keys {
            if let Some((sz, _)) = self.binpkgs.remove(&k) {
                freed_bytes += sz;
            }
        }
        (freed_count, freed_bytes)
    }
}

impl Default for GentooEcleanEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Gentoo Binary Package Host (binhost) Client Engine
#[derive(Debug, Clone)]
pub struct GentooBinpkgMetadata {
    pub package_atom: String,
    pub version: String,
    pub use_flags: Vec<String>,
    pub sha256_checksum: String,
    pub size_bytes: u64,
}

pub struct GentooBinhostEngine {
    pub binhost_url: String,
    pub available_binpkgs: BTreeMap<String, GentooBinpkgMetadata>,
}

impl GentooBinhostEngine {
    pub fn new(binhost_url: &str) -> Self {
        Self {
            binhost_url: binhost_url.to_string(),
            available_binpkgs: BTreeMap::new(),
        }
    }

    pub fn register_binpkg(&mut self, meta: GentooBinpkgMetadata) {
        self.available_binpkgs
            .insert(meta.package_atom.clone(), meta);
    }

    pub fn verify_and_fetch(&self, package_atom: &str, expected_hash: &str) -> Result<&GentooBinpkgMetadata, PortageError> {
        let meta = self
            .available_binpkgs
            .get(package_atom)
            .ok_or(PortageError::PackageNotFound)?;

        if meta.sha256_checksum == expected_hash {
            Ok(meta)
        } else {
            Err(PortageError::FetchFailed)
        }
    }
}

/// Portage tree
pub struct PortageTree {
    pub ebuilds: BTreeMap<String, Ebuild>,
    pub categories: Vec<String>,
}

impl PortageTree {
    pub fn new() -> Self {
        Self {
            ebuilds: BTreeMap::new(),
            categories: Vec::new(),
        }
    }

    /// Add ebuild
    pub fn add_ebuild(&mut self, ebuild: Ebuild) {
        self.ebuilds.insert(ebuild.name.clone(), ebuild);
    }

    /// Find ebuild
    pub fn find_ebuild(&self, name: &str) -> Result<&Ebuild, PortageError> {
        self.ebuilds.get(name).ok_or(PortageError::PackageNotFound)
    }

    /// Search ebuilds
    pub fn search(&self, query: &str) -> Vec<&Ebuild> {
        let query_lower = query.to_lowercase();
        self.ebuilds
            .values()
            .filter(|e| {
                e.name.to_lowercase().contains(&query_lower)
                    || e.description.to_lowercase().contains(&query_lower)
            })
            .collect()
    }
}

impl Default for PortageTree {
    fn default() -> Self {
        Self::new()
    }
}

/// Package database
pub struct PackageDatabase {
    pub installed: BTreeMap<String, String>,
}

impl PackageDatabase {
    pub fn new() -> Self {
        Self {
            installed: BTreeMap::new(),
        }
    }

    /// Add installed package
    pub fn add_installed(&mut self, ebuild: &Ebuild) -> Result<(), PortageError> {
        self.installed
            .insert(ebuild.name.clone(), ebuild.version.clone());
        Ok(())
    }

    /// Check if package is installed
    pub fn is_installed(&self, name: &str) -> bool {
        self.installed.contains_key(name)
    }

    /// Get installed version
    pub fn get_version(&self, name: &str) -> Option<&String> {
        self.installed.get(name)
    }
}

impl Default for PackageDatabase {
    fn default() -> Self {
        Self::new()
    }
}

/// Profile manager
pub struct ProfileManager {
    pub current_profile: Option<String>,
    pub available_profiles: Vec<String>,
}

impl ProfileManager {
    pub fn new() -> Self {
        Self {
            current_profile: None,
            available_profiles: vec![
                "default/linux/amd64/17.0".to_string(),
                "default/linux/amd64/17.0/desktop".to_string(),
                "default/linux/amd64/17.0/desktop/gnome".to_string(),
            ],
        }
    }

    /// Set profile
    pub fn set_profile(&mut self, profile: &str) -> Result<(), PortageError> {
        if self.available_profiles.contains(&profile.to_string()) {
            self.current_profile = Some(profile.to_string());
            Ok(())
        } else {
            Err(PortageError::ProfileNotFound)
        }
    }

    /// Get current profile
    pub fn get_profile(&self) -> Option<&String> {
        self.current_profile.as_ref()
    }
}

impl Default for ProfileManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Sigma Portage package manager
pub struct SigmaPortage {
    pub tree: PortageTree,
    pub database: PackageDatabase,
    pub profiles: ProfileManager,
    pub use_flags: UseFlagManager,
}

impl SigmaPortage {
    pub fn new() -> Self {
        Self {
            tree: PortageTree::new(),
            database: PackageDatabase::new(),
            profiles: ProfileManager::new(),
            use_flags: UseFlagManager::new(),
        }
    }

    /// Emerge package
    pub fn emerge(&mut self, package: &str) -> Result<(), PortageError> {
        let ebuild = self.tree.find_ebuild(package)?.clone();

        let ebuild_clone = ebuild.clone();

        // Resolve USE flags
        let use_flags = self.use_flags.resolve_for_package(&ebuild_clone)?;

        // Calculate dependencies
        let dependencies = self.calculate_dependencies(&ebuild_clone)?;

        // Emerge dependencies first
        for dep in &dependencies {
            if !self.database.is_installed(dep) {
                self.emerge(dep)?;
            }
        }

        // Simulate build process
        println!("Emerging {} ({})", package, ebuild.version);
        println!("USE flags: {:?}", use_flags);
        println!("Dependencies: {:?}", dependencies);

        // Update database
        self.database.add_installed(&ebuild_clone)?;

        Ok(())
    }

    /// Calculate dependencies
    fn calculate_dependencies(&self, ebuild: &Ebuild) -> Result<Vec<String>, PortageError> {
        let mut deps = Vec::new();

        for dep in &ebuild.depends {
            if !deps.contains(dep) {
                deps.push(dep.clone());
            }
        }

        Ok(deps)
    }

    /// Update USE flags for package
    pub fn update_use_flags(
        &mut self,
        package: &str,
        flags: Vec<String>,
    ) -> Result<(), PortageError> {
        for flag in flags {
            self.use_flags
                .enable_package_flag(package.to_string(), flag);
        }

        // Rebuild package
        if self.database.is_installed(package) {
            self.emerge(package)?;
        }

        Ok(())
    }

    /// Set profile
    pub fn set_profile(&mut self, profile: &str) -> Result<(), PortageError> {
        self.profiles.set_profile(profile)
    }

    /// Search packages
    pub fn search(&self, query: &str) -> Vec<&Ebuild> {
        self.tree.search(query)
    }

    /// Get installed packages
    pub fn get_installed(&self) -> Vec<(&String, &String)> {
        self.database.installed.iter().collect()
    }
}

impl Default for SigmaPortage {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_portage_emerge() {
        let mut portage = SigmaPortage::new();

        let ebuild = Ebuild {
            name: "example-pkg".to_string(),
            version: "1.0.0".to_string(),
            slot: "0".to_string(),
            description: "An example package".to_string(),
            homepage: "https://example.com".to_string(),
            license: "MIT".to_string(),
            iuse: vec!["X".to_string(), "gtk".to_string()],
            depends: vec![],
            rdepends: vec![],
            pdepends: vec![],
            src_uri: vec![],
        };

        portage.tree.add_ebuild(ebuild);
        let result = portage.emerge("example-pkg");
        assert!(result.is_ok());
    }

    #[test]
    fn test_use_flags() {
        let mut manager = UseFlagManager::new();
        manager.enable_global_flag("X".to_string());

        let ebuild = Ebuild {
            name: "example-pkg".to_string(),
            version: "1.0.0".to_string(),
            slot: "0".to_string(),
            description: "An example package".to_string(),
            homepage: "https://example.com".to_string(),
            license: "MIT".to_string(),
            iuse: vec!["X".to_string(), "gtk".to_string()],
            depends: vec![],
            rdepends: vec![],
            pdepends: vec![],
            src_uri: vec![],
        };

        let flags = manager.resolve_for_package(&ebuild).unwrap();
        assert!(flags.contains(&"X".to_string()));
    }

    #[test]
    fn test_gentoo_package_sets() {
        let mut sets = GentooPackageSetManager::new();
        assert!(sets.get_set("system").contains(&"sys-apps/portage".to_string()));

        sets.add_to_set("selected", "app-editors/neovim");
        let world = sets.get_set("world");
        assert!(world.contains(&"app-editors/neovim".to_string()));
        assert!(world.contains(&"sys-apps/portage".to_string()));

        assert!(sets.remove_from_set("selected", "app-editors/neovim"));
        assert!(!sets.get_set("world").contains(&"app-editors/neovim".to_string()));
    }

    #[test]
    fn test_gentoo_config_merge() {
        let mut merger = GentooConfigMergeEngine::new();
        merger.register_pending_update("/etc/portage/make.conf", "CFLAGS=\"-O2\"", "CFLAGS=\"-O3 -march=native\"");
        assert_eq!(merger.count_pending(), 1);

        let resolved = merger.resolve_update("/etc/portage/make.conf", ConfigMergeAction::UseNew).unwrap();
        assert_eq!(resolved, "CFLAGS=\"-O3 -march=native\"");
        assert_eq!(merger.count_pending(), 0);
    }

    #[test]
    fn test_gentoo_eclean_engine() {
        let mut eclean = GentooEcleanEngine::new();
        eclean.register_distfile("gcc-13.2.0.tar.xz", 150_000_000, true);
        eclean.register_distfile("linux-6.6.tar.xz", 140_000_000, false);
        eclean.register_binpkg("sys-apps/bash-5.2.tbz2", 5_000_000, true);

        let (freed_dist_count, freed_dist_bytes) = eclean.eclean_distfiles();
        assert_eq!(freed_dist_count, 1);
        assert_eq!(freed_dist_bytes, 150_000_000);

        let (freed_pkg_count, freed_pkg_bytes) = eclean.eclean_pkg();
        assert_eq!(freed_pkg_count, 1);
        assert_eq!(freed_pkg_bytes, 5_000_000);
    }

    #[test]
    fn test_gentoo_binhost_engine() {
        let mut binhost = GentooBinhostEngine::new("https://binhost.sigmaos.org");
        binhost.register_binpkg(GentooBinpkgMetadata {
            package_atom: "sys-apps/coreutils-9.4".to_string(),
            version: "9.4".to_string(),
            use_flags: vec!["nls".to_string()],
            sha256_checksum: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".to_string(),
            size_bytes: 12_000_000,
        });

        let pkg = binhost.verify_and_fetch(
            "sys-apps/coreutils-9.4",
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        );
        assert!(pkg.is_ok());
        assert_eq!(pkg.unwrap().size_bytes, 12_000_000);

        let bad_hash = binhost.verify_and_fetch("sys-apps/coreutils-9.4", "invalid_hash");
        assert!(bad_hash.is_err());
    }
}
