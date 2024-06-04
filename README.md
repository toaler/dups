# Turbo Tasker (tt)
[![Build status](https://github.com/toaler/turbo-tasker/actions/workflows/rust.yml/badge.svg)](https://github.com/toaler/turbo-tasker/actions)
[![Build status](https://github.com/toaler/turbo-tasker/actions/workflows/rust-clippy.yml/badge.svg)](https://github.com/toaler/turbo-tasker/actions)
[![Coverage Status](https://coveralls.io/repos/github/toaler/turbo-tasker/badge.svg?branch=main)](https://coveralls.io/github/toaler/turbo-tasker?branch=main)

Turbo Tasker is a platform agnostic PC maintenance assistant that strives to simplify common housekeeping tasks efficiently via speedy workflows.  Turbo Tasker encodes and optimizes various tasks related to system maintenance.

Workflows Supported:
1. Storage management - Provides storage management workflow allowing users to scan, inspect, stage and commit resource management actions.

2. Coming soon

## Features

### Storage Management 

Scanning

- Initial Scan - must perform intrusive scan once so the metadata of the transitive resource graph can be uncovered and cached. 
- Fast Scans (N+1 Scans) - persisting known resource hierarchy (files/dirs) metadata, specifically last modified allows for directory change detection required for fast resource analysis. 
- Scanning stats - various metrics to evalute scanning speed
- Inspection - Provides inspection analyzer for selecting operations on resources

Inspection

- top-k list by file size
- Big files - Identifies top-k files by size with the option to delete or compress
- recommended remediation action, either delete or compress
- Space Saver via Compression - identifies large infrequently updated files that are candidates for compression. Uses common cross platform "zip" compression 
- ability ot override recommendation


Staging

- review pending remediation actions by resource

Todo

4. Duplicate detection - Ability to identify duplicates
5. Broken symlink detection - identifies dangling symbolic links
7. Junk file cleaning
8. Supports OSX, Linux and Windows
9. Resource staging that allows per resource level actions (delete, compress) to be scheduled


## Build

This project uses the Tauri tech stack which utilizes Javascript for the frontend and Rust for the backend. The reason this technology was useful is its cross platform support.

### Tauri

<p>❯ npm install 
<p>❯ npm run tauri init
<p>❯ npm run build
<p>❯ npm run tauri dev
<p>❯ npm run tauri build