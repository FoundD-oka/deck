---
owner: 開発生産性責任者
updated: 2026-02-14
watch_paths:
  - .ai/*.md
  - .ci/doc-review.sh
staleness_days: 90
breaks_build_if_stale: false
---

# Meta

## Review Schedule
月1回、25分のレビュー。
対象: staleness超過頻度、doc-update成功率、doc-reviewのfalse positive率。

## Expiry Condition
この体系の内容が実際のCI強制ルールと一致しなくなったとき:
- 文書を正にしない。CI強制ルールを正とする
- 文書は「superseded」として残し、新しい版を作る

## Emergency Override
緊急時に強制チェックを一時無効化する場合:
- 対象ドキュメントのfrontmatterに override_until, override_reason, override_owner を追加する
- 最大7日間
- 7日超過で以後のリリースを自動停止
- 解除にはownerの承認が必要

## Tuning Policy
- watch_pathsのチューニングログは .ci/tuning-log.tsv で管理する
- false positive率が20%以下になるまで breaks_build_if_stale: true に昇格しない
- 昇格順: BUILD.md -> ARCHITECTURE.md -> OBSERVABILITY.md
