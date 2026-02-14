---
owner: DevOps / Platform
updated: 2026-02-14
watch_paths:
  - Makefile
  - package.json
  - Cargo.toml
  - Dockerfile
  - devcontainer.json
  - .github/workflows/
  - .ci/
staleness_days: 14
breaks_build_if_stale: false
---

# Build

## Single Source of Truth
CI定義がすべての正。ローカルコマンドはCIの薄いラッパーである。

## Commands

| Action | Command | Timeout | Expected Exit |
|--------|---------|---------|---------------|
| Build | `make build` | 5min | 0 |
| Test (unit) | `make test` | 10min | 0 |
| Lint | `make lint` | 3min | 0 |

## Environment Requirements
- Container: Dockerfile (or devcontainer.json)
- Required tools: [コンテナ定義を参照]
- Environment variables: .env.example を参照

## Verification After Fix
1. `make build` が exit 0
2. `make test` が exit 0
3. `make lint` が exit 0
4. 変更ファイルが ARCHITECTURE.md の Module Map と整合
