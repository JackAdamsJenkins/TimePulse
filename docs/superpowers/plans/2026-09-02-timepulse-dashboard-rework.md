# TimePulse Dashboard Rework Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make reminders visible as a native foreground modal and make dashboard, history, statistics, and projects use real stored data.

**Architecture:** Keep the existing React/Tauri app, add a dedicated Tauri activity window, and expose small SQLite commands for projects, activities, totals, and history. React renders each view from those commands; no mock metrics remain.

**Tech Stack:** Tauri 2, Rust, rusqlite, React, TypeScript, native Windows sound.

**Spec:** User-approved design in conversation on 2026-09-02.

**Global Constraints**

- The reminder window must show only the modal, without the dashboard behind it.
- Productif is green, Neutre is orange, and Temps perdu is red everywhere.
- The application remains local-first and uses the existing SQLite database.
- The Windows build must be tested before publishing the GitHub Release.

---

### Task 1: Native reminder window and sound

**Files:**
- Modify: `src-tauri/src/lib.rs`, `src-tauri/tauri.conf.json`, `src-tauri/Cargo.toml`
- Modify: `src/App.tsx`

- [ ] Add a Tauri activity window and commands to show, focus, keep it always-on-top, and play the native Windows pop sound.
- [ ] Route reminder actions through the activity window and hide it after save, ignore, or snooze.
- [ ] Run Rust checks and the frontend build.

### Task 2: Data commands and activity model

**Files:**
- Modify: `src-tauri/src/lib.rs`
- Create: `src/activityData.mjs`, `src/activityData.test.mjs`

- [ ] Add tested pure helpers for durations and productivity colors.
- [ ] Add SQLite commands for projects CRUD, activity history, and period totals.
- [ ] Return project/category fields separately instead of embedding them in descriptions.

### Task 3: React views and visual cleanup

**Files:**
- Modify: `src/App.tsx`, `src/App.css`, `src/responsive.css`
- Modify: `package.json`, `src-tauri/tauri.conf.json`

- [ ] Render readable French timestamps and real durations in the timeline and history.
- [ ] Render real metrics and charts in statistics.
- [ ] Add project management controls.
- [ ] Add explicit Projet/Catégorie labels and consistent productivity colors.
- [ ] Run all tests, build the Windows installers, commit, push, and create the GitHub Release.
