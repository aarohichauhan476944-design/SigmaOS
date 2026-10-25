// SPDX-License-Identifier: MIT
// SigmaOS Omarchy Linux Subsystem Enhancements PR Gateway Engine

#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::{
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};

#[cfg(any(feature = "standalone_test", test))]
use std::{
    format,
    string::{String, ToString},
    vec::Vec,
};

#[cfg(not(any(feature = "standalone_test", test)))]
use crate::klib::HashMap;
#[cfg(any(feature = "standalone_test", test))]
use std::collections::HashMap;

/// Capability Token for Omarchy Privileged Component Verification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapabilityToken(pub u64);

/// Omarchy Preset Color Schemes
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OmarchyThemePreset {
    CatppuccinMocha,
    NordicFrost,
    TokyoNight,
    GruvboxDark,
    CustomHex(String),
}

/// Dynamic Palette Colors for Quickshell QML Bindings
#[derive(Debug, Clone)]
pub struct OmarchyColorPalette {
    pub preset: OmarchyThemePreset,
    pub background: String,
    pub foreground: String,
    pub primary_accent: String,
    pub secondary_accent: String,
    pub urgent_alert: String,
    pub surface_variant: String,
}

impl OmarchyColorPalette {
    pub fn from_preset(preset: OmarchyThemePreset) -> Self {
        match preset.clone() {
            OmarchyThemePreset::CatppuccinMocha => Self {
                preset,
                background: String::from("#1e1e2e"),
                foreground: String::from("#cdd6f4"),
                primary_accent: String::from("#cba6f7"),
                secondary_accent: String::from("#89b4fa"),
                urgent_alert: String::from("#f38ba8"),
                surface_variant: String::from("#313244"),
            },
            OmarchyThemePreset::NordicFrost => Self {
                preset,
                background: String::from("#2e3440"),
                foreground: String::from("#eceff4"),
                primary_accent: String::from("#88c0d0"),
                secondary_accent: String::from("#81a1c1"),
                urgent_alert: String::from("#bf616a"),
                surface_variant: String::from("#3b4252"),
            },
            OmarchyThemePreset::TokyoNight => Self {
                preset,
                background: String::from("#1a1b26"),
                foreground: String::from("#a9b1d6"),
                primary_accent: String::from("#7aa2f7"),
                secondary_accent: String::from("#bb9af7"),
                urgent_alert: String::from("#f7768e"),
                surface_variant: String::from("#24283b"),
            },
            OmarchyThemePreset::GruvboxDark => Self {
                preset,
                background: String::from("#282828"),
                foreground: String::from("#ebdbb2"),
                primary_accent: String::from("#fe8019"),
                secondary_accent: String::from("#fabd2f"),
                urgent_alert: String::from("#fb4934"),
                surface_variant: String::from("#3c3836"),
            },
            OmarchyThemePreset::CustomHex(hex) => Self {
                preset,
                background: String::from("#101010"),
                foreground: String::from("#f0f0f0"),
                primary_accent: hex,
                secondary_accent: String::from("#808080"),
                urgent_alert: String::from("#ff0000"),
                surface_variant: String::from("#202020"),
            },
        }
    }

    /// Exports colors as Quickshell QML property definitions
    pub fn export_qml_properties(&self) -> String {
        format!(
            "pragma Singleton\nimport QtQuick 2.15\n\nQtObject {{\n    property color background: \"{}\"\n    property color foreground: \"{}\"\n    property color primaryAccent: \"{}\"\n    property color secondaryAccent: \"{}\"\n    property color urgentAlert: \"{}\"\n    property color surfaceVariant: \"{}\"\n}}\n",
            self.background,
            self.foreground,
            self.primary_accent,
            self.secondary_accent,
            self.urgent_alert,
            self.surface_variant
        )
    }
}

/// Overlay Interaction Mode
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OverlayInteractionMode {
    FullscreenModal,
    FloatingDock,
    TopBarBanner,
    RadialMenu,
}

/// Quick-Action Overlay Definition
#[derive(Debug, Clone)]
pub struct QuickActionOverlay {
    pub id: String,
    pub title: String,
    pub trigger_hotkey: String,
    pub mode: OverlayInteractionMode,
    pub active: bool,
    pub invocation_count: u64,
}

/// Interactive Quickshell Overlay Manager
#[derive(Debug, Clone)]
pub struct OmarchyQuickLauncherOverlayManager {
    pub overlays: HashMap<String, QuickActionOverlay>,
    pub hotkey_bindings: HashMap<String, String>,
    pub active_overlay_id: Option<String>,
}

impl OmarchyQuickLauncherOverlayManager {
    pub fn new() -> Self {
        Self {
            overlays: HashMap::new(),
            hotkey_bindings: HashMap::new(),
            active_overlay_id: None,
        }
    }

    /// Registers a new quick-action overlay
    pub fn register_overlay(&mut self, overlay: QuickActionOverlay) {
        let hotkey = overlay.trigger_hotkey.clone();
        let id = overlay.id.clone();
        self.hotkey_bindings.insert(hotkey, id.clone());
        self.overlays.insert(id, overlay);
    }

    /// Handles incoming hotkey press and summons corresponding overlay
    pub fn handle_hotkey_press(&mut self, hotkey: &str) -> Result<String, &'static str> {
        let id = match self.hotkey_bindings.get(hotkey) {
            Some(i) => i.clone(),
            None => return Err("Hotkey not bound to any overlay"),
        };

        if let Some(ref current_id) = self.active_overlay_id {
            if current_id == &id {
                // Toggle off
                if let Some(overlay) = self.overlays.get_mut(&id) {
                    overlay.active = false;
                }
                self.active_overlay_id = None;
                return Ok(format!("Dismissed overlay {}", id));
            }
        }

        if let Some(overlay) = self.overlays.get_mut(&id) {
            overlay.active = true;
            overlay.invocation_count += 1;
            self.active_overlay_id = Some(id.clone());
            Ok(format!("Summoned overlay {}", id))
        } else {
            Err("Overlay target missing")
        }
    }

    /// Dismisses any active overlay
    pub fn dismiss_active(&mut self) -> bool {
        if let Some(ref id) = self.active_overlay_id {
            if let Some(overlay) = self.overlays.get_mut(id) {
                overlay.active = false;
            }
            self.active_overlay_id = None;
            true
        } else {
            false
        }
    }
}

impl Default for OmarchyQuickLauncherOverlayManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Subsystem Component Security Status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComponentSecurityStatus {
    ValidCapabilityToken,
    InvalidCapabilityToken,
    AccessDenied,
    Sandboxed,
}

/// Omarchy Subsystem Capability & Health Validator
pub struct OmarchySubsystemSecurityValidator {
    pub trusted_tokens: Vec<CapabilityToken>,
}

impl OmarchySubsystemSecurityValidator {
    pub fn new() -> Self {
        Self {
            trusted_tokens: Vec::new(),
        }
    }

    pub fn register_trusted_token(&mut self, token: CapabilityToken) {
        self.trusted_tokens.push(token);
    }

    /// Validates capability token for requested privileged Omarchy action
    pub fn validate_capability(&self, token: &CapabilityToken) -> ComponentSecurityStatus {
        if self.trusted_tokens.contains(token) {
            ComponentSecurityStatus::ValidCapabilityToken
        } else {
            ComponentSecurityStatus::InvalidCapabilityToken
        }
    }
}

impl Default for OmarchySubsystemSecurityValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Enhancement PR Format Type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OmarchyEnhancementType {
    DynamicPaletteEngine,
    QuickActionOverlay,
    SecurityCapabilityValidator,
    BarWidgetExtension,
    PluginArchitectureUpgrade,
}

/// PR Status Lifecycle
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OmarchyPrLifecycle {
    Draft,
    UnderReview,
    VerifiedInCi,
    ApprovedAndMerged,
    Rejected,
}

/// Omarchy Subsystem Enhancement PR Submission
#[derive(Debug, Clone)]
pub struct OmarchyEnhancementPr {
    pub pr_id: u32,
    pub title: String,
    pub author: String,
    pub enhancement_type: OmarchyEnhancementType,
    pub code_diff: String,
    pub status: OmarchyPrLifecycle,
    pub timestamp: u64,
}

/// Sovereign Omarchy Subsystem Enhancements PR Gateway Engine
pub struct SovereignOmarchySubsystemEnhancementsPrGateway {
    pub prs: HashMap<u32, OmarchyEnhancementPr>,
    pub next_pr_id: u32,
    pub palette_engine: OmarchyColorPalette,
    pub overlay_manager: OmarchyQuickLauncherOverlayManager,
    pub security_validator: OmarchySubsystemSecurityValidator,
}

impl SovereignOmarchySubsystemEnhancementsPrGateway {
    pub fn new() -> Self {
        Self {
            prs: HashMap::new(),
            next_pr_id: 100,
            palette_engine: OmarchyColorPalette::from_preset(OmarchyThemePreset::CatppuccinMocha),
            overlay_manager: OmarchyQuickLauncherOverlayManager::new(),
            security_validator: OmarchySubsystemSecurityValidator::new(),
        }
    }

    /// Submit a new Omarchy enhancement PR
    pub fn submit_pr(
        &mut self,
        title: &str,
        author: &str,
        enhancement_type: OmarchyEnhancementType,
        code_diff: &str,
    ) -> u32 {
        let id = self.next_pr_id;
        self.next_pr_id += 1;

        let pr = OmarchyEnhancementPr {
            pr_id: id,
            title: title.to_string(),
            author: author.to_string(),
            enhancement_type,
            code_diff: code_diff.to_string(),
            status: OmarchyPrLifecycle::Draft,
            timestamp: 20250101 + id as u64,
        };

        self.prs.insert(id, pr);
        id
    }

    /// Run automated CI gate verification on submitted PR
    pub fn run_ci_gate(&mut self, pr_id: u32) -> bool {
        if let Some(pr) = self.prs.get_mut(&pr_id) {
            if pr.code_diff.is_empty() {
                pr.status = OmarchyPrLifecycle::Rejected;
                return false;
            }
            pr.status = OmarchyPrLifecycle::VerifiedInCi;
            true
        } else {
            false
        }
    }

    /// Merge verified PR into SigmaOS runtime
    pub fn merge_pr(&mut self, pr_id: u32) -> bool {
        if let Some(pr) = self.prs.get_mut(&pr_id) {
            if pr.status == OmarchyPrLifecycle::VerifiedInCi {
                pr.status = OmarchyPrLifecycle::ApprovedAndMerged;
                return true;
            }
        }
        false
    }
}

impl Default for SovereignOmarchySubsystemEnhancementsPrGateway {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_omarchy_color_palette_qml_export() {
        let palette = OmarchyColorPalette::from_preset(OmarchyThemePreset::CatppuccinMocha);
        let qml = palette.export_qml_properties();
        assert!(qml.contains("#1e1e2e"));
        assert!(qml.contains("#cba6f7"));
        assert!(qml.contains("property color primaryAccent"));
    }

    #[test]
    fn test_overlay_manager_hotkey_flow() {
        let mut manager = OmarchyQuickLauncherOverlayManager::new();
        let overlay = QuickActionOverlay {
            id: String::from("emoji-picker"),
            title: String::from("Emoji Selection Tool"),
            trigger_hotkey: String::from("Super+Period"),
            mode: OverlayInteractionMode::FullscreenModal,
            active: false,
            invocation_count: 0,
        };
        manager.register_overlay(overlay);

        let res = manager.handle_hotkey_press("Super+Period");
        assert!(res.is_ok());
        assert_eq!(manager.active_overlay_id.as_deref(), Some("emoji-picker"));
        assert_eq!(manager.overlays.get("emoji-picker").unwrap().invocation_count, 1);

        // Toggle off
        let res2 = manager.handle_hotkey_press("Super+Period");
        assert!(res2.is_ok());
        assert_eq!(manager.active_overlay_id, None);
    }

    #[test]
    fn test_security_validator_capability_token() {
        let mut validator = OmarchySubsystemSecurityValidator::new();
        let token = CapabilityToken(0x9988776655443322);
        validator.register_trusted_token(token);

        assert_eq!(
            validator.validate_capability(&token),
            ComponentSecurityStatus::ValidCapabilityToken
        );

        let untrusted = CapabilityToken(0x1111111111111111);
        assert_eq!(
            validator.validate_capability(&untrusted),
            ComponentSecurityStatus::InvalidCapabilityToken
        );
    }

    #[test]
    fn test_enhancement_pr_gateway_lifecycle() {
        let mut gateway = SovereignOmarchySubsystemEnhancementsPrGateway::new();

        let pr_id = gateway.submit_pr(
            "Add Nord palette and Quickshell Overlay Manager",
            "developer@sigmaos.org",
            OmarchyEnhancementType::DynamicPaletteEngine,
            "diff --git a/palette b/palette",
        );

        assert_eq!(pr_id, 100);
        assert!(gateway.run_ci_gate(pr_id));
        assert!(gateway.merge_pr(pr_id));

        assert_eq!(
            gateway.prs.get(&pr_id).unwrap().status,
            OmarchyPrLifecycle::ApprovedAndMerged
        );
    }
}
