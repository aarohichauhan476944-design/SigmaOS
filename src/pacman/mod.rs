// SPDX-License-Identifier: MIT
// SigmaOS Pacman-inspired Package Manager
// Arch Linux pacman-inspired package management system

use std::collections::{BTreeMap, BTreeSet};
use std::string::String;
use std::vec::Vec;

/// Pacman transaction hook timing type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HookType {
    PreTransaction,
    PostTransaction,
}

/// Pacman transaction hook trigger condition
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HookTrigger {
    Package(String),
    File(String),
    Any,
}

/// Pacman transaction hook definition (ALPM / Arch Linux hook format)
#[derive(Debug, Clone, PartialEq)]
pub struct PacmanHook {
    pub name: String,
    pub hook_type: HookType,
    pub trigger: HookTrigger,
    pub exec_command: String,
}

impl PacmanHook {
    pub fn new(name: String, hook_type: HookType, trigger: HookTrigger, exec_command: String) -> Self {
        Self {
            name,
            hook_type,
            trigger,
            exec_command,
        }
    }

    /// Check if hook matches a package or file event
    pub fn matches(&self, target_pkg: &str, files: &[String]) -> bool {
        match &self.trigger {
            HookTrigger::Any => true,
            HookTrigger::Package(pkg) => pkg == target_pkg || pkg == "*",
            HookTrigger::File(file_pattern) => files.iter().any(|f| f.contains(file_pattern)),
        }
    }
}

/// Pacman-inspired package information
#[derive(Debug, Clone, PartialEq)]
pub struct PacmanPackage {
    pub name: String,
    pub version: String,
    pub description: String,
    pub url: String,
    pub license: Vec<String>,
    pub groups: Vec<String>,
    pub depends: Vec<String>,
    pub optdepends: Vec<String>,
    pub provides: Vec<String>,
    pub conflicts: Vec<String>,
    pub replaces: Vec<String>,
    pub size: u64,
    pub installed_size: u64,
    pub architecture: String,
    pub build_date: String,
    pub install_date: String,
    pub packager: String,
    pub files: Vec<String>,
    pub backup: Vec<String>,
    pub install_reason: String,
    pub validation: String,
}

impl PacmanPackage {
    pub fn new(name: String, version: String) -> Self {
        Self {
            name,
            version,
            description: String::new(),
            url: String::new(),
            license: Vec::new(),
            groups: Vec::new(),
            depends: Vec::new(),
            optdepends: Vec::new(),
            provides: Vec::new(),
            conflicts: Vec::new(),
            replaces: Vec::new(),
            size: 0,
            installed_size: 0,
            architecture: String::from("x86_64"),
            build_date: String::new(),
            install_date: String::new(),
            packager: String::new(),
            files: Vec::new(),
            backup: Vec::new(),
            install_reason: String::from("explicit"),
            validation: String::from("none"),
        }
    }

    /// Add dependency
    pub fn add_dependency(&mut self, dep: String) {
        self.depends.push(dep);
    }

    /// Add optional dependency
    pub fn add_opt_dependency(&mut self, dep: String) {
        self.optdepends.push(dep);
    }

    /// Add conflict
    pub fn add_conflict(&mut self, conflict: String) {
        self.conflicts.push(conflict);
    }

    /// Add file
    pub fn add_file(&mut self, file: String) {
        self.files.push(file);
    }
}

/// Pacman-inspired package database
#[derive(Debug, Clone)]
pub struct PacmanDatabase {
    pub packages: BTreeMap<String, PacmanPackage>,
    pub local_packages: BTreeMap<String, PacmanPackage>,
    pub sync_databases: BTreeMap<String, Vec<String>>,
    pub cache_dir: String,
    pub config_dir: String,
    pub database_dir: String,
}

impl PacmanDatabase {
    pub fn new() -> Self {
        Self {
            packages: BTreeMap::new(),
            local_packages: BTreeMap::new(),
            sync_databases: BTreeMap::new(),
            cache_dir: String::from("/var/cache/pacman/pkg"),
            config_dir: String::from("/etc/pacman.conf"),
            database_dir: String::from("/var/lib/pacman"),
        }
    }

    /// Add package to database
    pub fn add_package(&mut self, package: PacmanPackage) {
        let name = package.name.clone();
        self.packages.insert(name, package);
    }

    /// Add local package
    pub fn add_local_package(&mut self, package: PacmanPackage) {
        let name = package.name.clone();
        self.local_packages.insert(name, package);
    }

    /// Add sync database
    pub fn add_sync_database(&mut self, name: String, packages: Vec<String>) {
        self.sync_databases.insert(name, packages);
    }

    /// Get package by name
    pub fn get_package(&self, name: &str) -> Option<&PacmanPackage> {
        self.packages.get(name)
    }

    /// Get local package by name
    pub fn get_local_package(&self, name: &str) -> Option<&PacmanPackage> {
        self.local_packages.get(name)
    }

    /// Search packages by pattern
    pub fn search_packages(&self, pattern: &str) -> Vec<String> {
        self.packages
            .keys()
            .filter(|name| name.contains(pattern))
            .cloned()
            .collect()
    }

    /// Get package dependencies
    pub fn get_dependencies(&self, name: &str) -> Vec<String> {
        if let Some(pkg) = self.packages.get(name) {
            pkg.depends.clone()
        } else {
            Vec::new()
        }
    }

    /// Check if package is installed
    pub fn is_installed(&self, name: &str) -> bool {
        self.local_packages.contains_key(name)
    }

    /// List all packages
    pub fn list_packages(&self) -> Vec<String> {
        self.packages.keys().cloned().collect()
    }

    /// List installed packages
    pub fn list_installed_packages(&self) -> Vec<String> {
        self.local_packages.keys().cloned().collect()
    }

    /// Get package size
    pub fn get_package_size(&self, name: &str) -> Option<u64> {
        self.packages.get(name).map(|p| p.size)
    }

    /// Get installed package size
    pub fn get_installed_size(&self, name: &str) -> Option<u64> {
        self.local_packages.get(name).map(|p| p.installed_size)
    }

    /// Query orphan packages (pacman -Qtdq)
    /// Packages installed as dependency that are no longer required by any installed package
    pub fn get_orphans(&self) -> Vec<String> {
        let mut required_deps = BTreeSet::new();

        for (_name, pkg) in &self.local_packages {
            for dep in &pkg.depends {
                required_deps.insert(dep.as_str());
            }
        }

        let mut orphans = Vec::new();
        for (name, pkg) in &self.local_packages {
            if pkg.install_reason == "dependency" && !required_deps.contains(name.as_str()) {
                orphans.push(name.clone());
            }
        }
        orphans
    }
}

impl Default for PacmanDatabase {
    fn default() -> Self {
        Self::new()
    }
}

/// Pacman-inspired package manager
#[derive(Debug, Clone)]
pub struct PacmanPackageManager {
    pub database: PacmanDatabase,
    pub hooks: Vec<PacmanHook>,
    pub executed_hooks: Vec<String>,
    pub operation: String,
    pub noconfirm: bool,
    pub needed: bool,
    pub overwrite: bool,
    pub asdeps: bool,
    pub asexplicit: bool,
    pub quiet: bool,
    pub verbose: bool,
}

impl PacmanPackageManager {
    pub fn new() -> Self {
        Self {
            database: PacmanDatabase::new(),
            hooks: Vec::new(),
            executed_hooks: Vec::new(),
            operation: String::new(),
            noconfirm: false,
            needed: false,
            overwrite: false,
            asdeps: false,
            asexplicit: false,
            quiet: false,
            verbose: false,
        }
    }

    /// Register a pacman hook
    pub fn add_hook(&mut self, hook: PacmanHook) {
        self.hooks.push(hook);
    }

    /// Execute matching transaction hooks
    pub fn run_hooks(&mut self, hook_type: HookType, target_pkg: &str, files: &[String]) {
        for hook in &self.hooks {
            if hook.hook_type == hook_type && hook.matches(target_pkg, files) {
                self.executed_hooks.push(format!("{}: {}", hook.name, hook.exec_command));
            }
        }
    }

    /// Install package
    pub fn install(&mut self, package_name: &str) -> Result<(), &'static str> {
        if self.database.is_installed(package_name) && self.needed {
            return Err("Package already installed and --needed flag set");
        }

        let pkg = self.database.get_package(package_name).cloned();
        if let Some(pkg) = pkg {
            let files = pkg.files.clone();
            self.run_hooks(HookType::PreTransaction, package_name, &files);

            let mut local_pkg = pkg;
            local_pkg.install_date = format!(
                "{}",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs()
            );
            local_pkg.install_reason = if self.asdeps {
                String::from("dependency")
            } else {
                String::from("explicit")
            };

            self.database.add_local_package(local_pkg);

            self.run_hooks(HookType::PostTransaction, package_name, &files);
            Ok(())
        } else {
            Err("Package not found in database")
        }
    }

    /// Remove package
    pub fn remove(&mut self, package_name: &str) -> Result<(), &'static str> {
        if !self.database.is_installed(package_name) {
            return Err("Package not installed");
        }

        let files = self.database.get_local_package(package_name).map(|p| p.files.clone()).unwrap_or_default();
        self.run_hooks(HookType::PreTransaction, package_name, &files);

        self.database.local_packages.remove(package_name);

        self.run_hooks(HookType::PostTransaction, package_name, &files);
        Ok(())
    }

    /// Cascaded recursive package removal (pacman -Rns style)
    pub fn remove_recursive(&mut self, package_name: &str) -> Result<Vec<String>, &'static str> {
        if !self.database.is_installed(package_name) {
            return Err("Package not installed");
        }

        let mut removed = Vec::new();
        let target_deps = self.database.get_local_package(package_name).map(|p| p.depends.clone()).unwrap_or_default();

        self.remove(package_name)?;
        removed.push(String::from(package_name));

        // Check each dependency to see if it became orphaned
        for dep in target_deps {
            if self.database.is_installed(&dep) {
                let orphans = self.database.get_orphans();
                if orphans.contains(&dep) {
                    let _ = self.remove(&dep);
                    removed.push(dep);
                }
            }
        }

        Ok(removed)
    }

    /// Update package
    pub fn update(&mut self, package_name: &str) -> Result<(), &'static str> {
        if let Some(pkg) = self.database.get_package(package_name) {
            if let Some(local_pkg) = self.database.get_local_package(package_name) {
                if pkg.version == local_pkg.version {
                    return Err("Package already up to date");
                }
            }

            self.remove(package_name)?;
            self.install(package_name)?;
            Ok(())
        } else {
            Err("Package not found in database")
        }
    }

    /// Sync database
    pub fn sync(&mut self) -> Result<(), &'static str> {
        // In real implementation, would download package databases
        Ok(())
    }

    /// Upgrade all packages
    pub fn upgrade(&mut self) -> Result<Vec<String>, &'static str> {
        let mut upgraded = Vec::new();

        for package_name in self.database.list_installed_packages() {
            if let Ok(_) = self.update(&package_name) {
                upgraded.push(package_name);
            }
        }

        Ok(upgraded)
    }

    /// Query package information
    pub fn query(&self, package_name: &str) -> Option<&PacmanPackage> {
        self.database.get_package(package_name)
    }

    /// Search packages
    pub fn search(&self, pattern: &str) -> Vec<String> {
        self.database.search_packages(pattern)
    }

    /// Get package dependencies
    pub fn get_deps(&self, package_name: &str) -> Vec<String> {
        self.database.get_dependencies(package_name)
    }

    /// Check package files
    pub fn check_files(&self, package_name: &str) -> Vec<String> {
        if let Some(pkg) = self.database.get_local_package(package_name) {
            pkg.files.clone()
        } else {
            Vec::new()
        }
    }

    /// Set noconfirm flag
    pub fn set_noconfirm(&mut self, noconfirm: bool) {
        self.noconfirm = noconfirm;
    }

    /// Set needed flag
    pub fn set_needed(&mut self, needed: bool) {
        self.needed = needed;
    }

    /// Set asdeps flag
    pub fn set_asdeps(&mut self, asdeps: bool) {
        self.asdeps = asdeps;
    }

    /// Set asexplicit flag
    pub fn set_asexplicit(&mut self, asexplicit: bool) {
        self.asexplicit = asexplicit;
    }
}

impl Default for PacmanPackageManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pacman_package() {
        let mut pkg = PacmanPackage::new(String::from("test"), String::from("1.0.0"));
        pkg.description = String::from("Test package");
        pkg.add_dependency(String::from("libc"));

        assert_eq!(pkg.name, String::from("test"));
        assert!(pkg.depends.contains(&String::from("libc")));
    }

    #[test]
    fn test_pacman_database() {
        let mut db = PacmanDatabase::new();
        let pkg = PacmanPackage::new(String::from("nginx"), String::from("1.24.0"));
        db.add_package(pkg);

        assert!(db.get_package("nginx").is_some());
        assert!(db.search_packages("ngi").contains(&String::from("nginx")));
    }

    #[test]
    fn test_pacman_install() {
        let mut manager = PacmanPackageManager::new();
        let pkg = PacmanPackage::new(String::from("vim"), String::from("9.0"));
        manager.database.add_package(pkg);

        assert!(manager.install("vim").is_ok());
        assert!(manager.database.is_installed("vim"));
    }

    #[test]
    fn test_pacman_remove() {
        let mut manager = PacmanPackageManager::new();
        let pkg = PacmanPackage::new(String::from("nano"), String::from("7.0"));
        manager.database.add_package(pkg.clone());
        manager.database.add_local_package(pkg);

        assert!(manager.remove("nano").is_ok());
        assert!(!manager.database.is_installed("nano"));
    }

    #[test]
    fn test_pacman_update() {
        let mut manager = PacmanPackageManager::new();
        let old_pkg = PacmanPackage::new(String::from("git"), String::from("2.39"));
        let new_pkg = PacmanPackage::new(String::from("git"), String::from("2.40"));
        manager.database.add_package(new_pkg);
        manager.database.add_local_package(old_pkg);

        assert!(manager.update("git").is_ok());
    }

    #[test]
    fn test_pacman_search() {
        let mut manager = PacmanPackageManager::new();
        let pkg = PacmanPackage::new(String::from("docker"), String::from("24.0"));
        manager.database.add_package(pkg);

        let results = manager.search("doc");
        assert!(results.contains(&String::from("docker")));
    }

    #[test]
    fn test_pacman_hooks() {
        let mut manager = PacmanPackageManager::new();
        let hook = PacmanHook::new(
            String::from("glib-compile-schemas"),
            HookType::PostTransaction,
            HookTrigger::Package(String::from("glib2")),
            String::from("/usr/bin/glib-compile-schemas /usr/share/glib-2.0/schemas"),
        );
        manager.add_hook(hook);

        let mut pkg = PacmanPackage::new(String::from("glib2"), String::from("2.76.0"));
        pkg.add_file(String::from("/usr/share/glib-2.0/schemas/gschemas.compiled"));
        manager.database.add_package(pkg);

        assert!(manager.install("glib2").is_ok());
        assert_eq!(manager.executed_hooks.len(), 1);
        assert!(manager.executed_hooks[0].contains("glib-compile-schemas"));
    }

    #[test]
    fn test_pacman_orphans_and_recursive_remove() {
        let mut manager = PacmanPackageManager::new();

        let mut dep_pkg = PacmanPackage::new(String::from("libfoo"), String::from("1.0.0"));
        dep_pkg.install_reason = String::from("dependency");
        manager.database.add_package(dep_pkg.clone());

        let mut main_pkg = PacmanPackage::new(String::from("foo-app"), String::from("2.0.0"));
        main_pkg.add_dependency(String::from("libfoo"));
        main_pkg.install_reason = String::from("explicit");
        manager.database.add_package(main_pkg.clone());

        // Install dependency as dependency, main as explicit
        manager.set_asdeps(true);
        assert!(manager.install("libfoo").is_ok());
        manager.set_asdeps(false);
        assert!(manager.install("foo-app").is_ok());

        // Initially libfoo is required by foo-app, so no orphans
        assert!(manager.database.get_orphans().is_empty());

        // Recursively remove foo-app, should cascade remove libfoo
        let removed = manager.remove_recursive("foo-app").unwrap();
        assert!(removed.contains(&String::from("foo-app")));
        assert!(removed.contains(&String::from("libfoo")));
        assert!(!manager.database.is_installed("foo-app"));
        assert!(!manager.database.is_installed("libfoo"));
    }
}
