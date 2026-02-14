#!/usr/bin/env bash
set -euo pipefail

# doc-review: ドキュメント整合性チェック（CI用）
# 実行時間上限: 2分
# 使い方: bash .ci/doc-review.sh [--warn-only]

WARN_ONLY="${1:-}"
TODAY=$(date +%Y-%m-%d)
EXIT_CODE=0
RESULTS=()
TUNING_LOG=".ci/tuning-log.tsv"

# tuning-logヘッダー作成（存在しなければ）
if [ ! -f "$TUNING_LOG" ]; then
  echo -e "date\tchanged_file\tmatched_doc\tstatus\tactual" > "$TUNING_LOG"
fi

# 差分ファイル取得
CHANGED_FILES=$(git diff --name-only HEAD 2>/dev/null || echo "")

check_staleness() {
  local doc="$1"
  local updated="$2"
  local staleness_days="$3"
  local breaks_build="$4"

  if [ -z "$updated" ] || [ -z "$staleness_days" ]; then
    return
  fi

  local updated_epoch=$(date -j -f "%Y-%m-%d" "$updated" "+%s" 2>/dev/null || date -d "$updated" "+%s" 2>/dev/null || echo "0")
  local threshold_epoch=$((updated_epoch + staleness_days * 86400))
  local today_epoch=$(date "+%s")

  if [ "$today_epoch" -gt "$threshold_epoch" ]; then
    local days_over=$(( (today_epoch - threshold_epoch) / 86400 ))
    if [ "$breaks_build" = "true" ] && [ "$WARN_ONLY" != "--warn-only" ]; then
      echo "[FAIL] $doc — stale (${days_over} days over, threshold ${staleness_days})"
      EXIT_CODE=1
    else
      echo "[WARN] $doc — stale (${days_over} days over, threshold ${staleness_days})"
    fi
  else
    echo "[PASS] $doc — staleness OK"
  fi
}

# .ai/ 配下のMarkdownファイルをチェック
for doc in .ai/*.md; do
  [ -f "$doc" ] || continue
  basename=$(basename "$doc")

  # frontmatterからフィールドを抽出（簡易YAML解析）
  updated=$(sed -n '/^---$/,/^---$/p' "$doc" | grep "^updated:" | head -1 | sed 's/updated: *//')
  staleness_days=$(sed -n '/^---$/,/^---$/p' "$doc" | grep "^staleness_days:" | head -1 | sed 's/staleness_days: *//')
  breaks_build=$(sed -n '/^---$/,/^---$/p' "$doc" | grep "^breaks_build_if_stale:" | head -1 | sed 's/breaks_build_if_stale: *//')

  check_staleness "$basename" "$updated" "$staleness_days" "$breaks_build"
done

echo ""
echo "--- doc-review complete ---"
exit $EXIT_CODE
