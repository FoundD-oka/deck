---
owner: テックリード
updated: 2026-02-14
watch_paths:
  - src/
  - lib/
  - packages/
staleness_days: 30
breaks_build_if_stale: false
---

# Architecture

## Module Map

| Module | Path | Responsibility | Owns (data/API) | Depends On |
|--------|------|----------------|------------------|------------|
| [module] | [path] | [責務] | [所有データ] | [依存先] |

## Boundary Rules
- モジュール間通信は公開インターフェースのみ
- 循環依存は禁止

## Entry Points
- Application: [エントリポイント]
- Tests: `make test` (-> BUILD.md参照)

## Concurrency Boundaries
- Safe to work in parallel: 各module内部の変更
- Conflict-prone: 共有型定義、ビルド設定
