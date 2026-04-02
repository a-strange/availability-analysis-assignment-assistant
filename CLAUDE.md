# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

The Availability Analysis Assignment Assistant is a Rust GUI application built with eframe/egui that assigns team members to services based on availability and service importance. It uses a weighted random assignment algorithm to distribute services fairly among selected team members.

## Common Commands

### Build and Run
```bash
cargo build                    # Build debug version
cargo build --release          # Build release version
cargo run                      # Run debug version
cargo run --release            # Run release version
```

### Testing and Linting
```bash
cargo test                     # Run all tests
cargo clippy                   # Run linter
cargo fmt                      # Format code
cargo check                    # Fast compilation check without binary
```

### Cross-platform Build
The release workflow targets x86_64-apple-darwin:
```bash
cargo build --release --target x86_64-apple-darwin
```

## Architecture

### Single-File Structure
The entire application is contained in `src/main.rs`. This is a small GUI application with no module separation.

### Core Components

**Data Structures:**
- `AnalysisAssistant`: Main application state containing team members, services, and display text
- `Service`: Represents a service with name and importance (0-3 scale)
- Team members stored as `Vec<(String, bool)>` where bool indicates availability

**Assignment Algorithm (lines 117-182):**
The assignment algorithm has two phases:
1. **First pass**: Ensures each unique service with importance > 0 is assigned at least once
2. **Second pass**: Distributes remaining services (based on importance weighting) to remaining team members

Services with higher importance values appear multiple times in the service pool (importance = 2 means the service appears twice), increasing their probability of assignment.

### GUI Framework
Built on eframe/egui (v0.30.0):
- Uses `egui::Window` for the main interface
- Light theme is set in the update loop
- Image support via `egui_extras`
- Screenshot capability enabled via `__screenshot` feature flag

## Release Process

Releases are automated via GitHub Actions on push to main. The workflow:
1. Builds for macOS (x86_64-apple-darwin)
2. Generates timestamp-based tags (format: v%Y.%m.%d-%H%M)
3. Creates GitHub releases with binary artifacts
