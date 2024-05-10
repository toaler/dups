# Turbo Tasker (tt)

Turbo Tasker is a cross-platform PC task manager that strives to complete task execution efficiently.  Turbo Tasker encodes and optimizes various tasks related to system maintenance.


[![Build status](https://github.com/toaler/turbo-tasker/actions/workflows/rust.yml/badge.svg)](https://github.com/toaler/turbo-tasker/actions)
[![Build status](https://github.com/toaler/turbo-tasker/actions/workflows/rust-clippy.yml/badge.svg)](https://github.com/toaler/turbo-tasker/actions)
[![Coverage Status](https://coveralls.io/repos/github/toaler/turbo-tasker/badge.svg?branch=main)](https://coveralls.io/github/toaler/turbo-tasker?branch=main)

## Features

### Storage Management 

Provides storage management workflow allowing users to scan, inspect, staging and commit resource management actions.

1. Fast capture - persisting known resource hierarchy (files/dirs) metadata, specifically last modified allows for fast reconciliation.
2. Fast analysis - In-memory resource hierarchy metadata allows for fast analysis. 
3. Inspection View
- Big files - Identifies top-k files by size with the option to delete or compress
4. Duplicate detection - Ability to identify duplicates
5. Space Saver via Compression - identifies large infrequently updated files that are candidates for compression. Uses common cross platform "zip" compression 
6. Broken symlink detection - identifies dangling symbolic links
7. Junk file cleaning
8. Supports OSX, Linux and Windows
9. Resource staging that allows per resource level actions (delete, compress) to be scheduled
10. 


## Build

### Tauri

<p>❯ npm install 
<p>❯ npm run tauri init
<p>❯ npm run build
<p>❯ npm run tauri dev
<p>❯ npm run tauri build