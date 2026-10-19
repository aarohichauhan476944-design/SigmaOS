# Elementary OS Pantheon & Granite Ecosystem Integration Study

## Overview
This document studies the core architecture of elementary OS (Pantheon Desktop and Granite UI Toolkit) and details its adaptation for SigmaOS.

## Core Architectural Pillars

1. **Granite HIG Toolkit & Typography**
   - **HIG Grid:** Strict 4px / 8px grid alignment for padding, margins, and component dimensions.
   - **Accent Palette:** Consistent color palette (Blueberry, Strawberry, Orange, Banana, Lime, Mint, Grape, Slate, Cocoa).
   - **Dark Mode Scheduling:** System-wide dark style preference support and automated schedule support.

2. **Contractor Service (Desktop IPC Portal)**
   - Context-sensitive actions based on MIME type registry (`io.elementary.contractor`).
   - Dynamic action menu generation for files, text selections, and system entities.

3. **Gala Window Manager & Tiling**
   - Workspace isolation and multitasking view (Exposé / Overview).
   - Dynamic window snapping (Left/Right half tile, Quarter grid tile).

4. **Wingpanel & Plank**
   - **Wingpanel:** Modular top status bar supporting Left, Center (Clock/Date), and Right applet layouts.
   - **Plank:** Desktop application dock with live badge counts and hover zoom effects.

5. **AppCenter Pay-What-You-Want (PWYW)**
   - Curated application catalog supporting custom download prices (free or donation-based monetized installation).
   - Complete ledger and transaction history tracking.

6. **Switchboard Control Center & Onboarding**
   - Pluggable modular settings control center (`SwitchboardPlug`).
   - Step-by-step welcome wizard (`io.elementary.onboarding`) for initial desktop configuration.
