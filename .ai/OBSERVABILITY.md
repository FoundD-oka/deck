---
owner: SRE / Platform
updated: 2026-02-14
watch_paths:
  - "**/*log*"
  - "**/*metric*"
staleness_days: 30
breaks_build_if_stale: false
---

# Observability

## Logging Standards
- Format: structured JSON
- Required fields: timestamp, level, service, trace_id, message
- Error logs must include: error_code, context, stack_trace

## Agent Activity Logging
- Record: tool calls, inputs, outputs, errors, handoffs
- Replay: セッション再生が可能な形式で保存

## Key Metrics

| Metric | Type | Alert Threshold |
|--------|------|-----------------|
| Request latency p99 | histogram | > 200ms |
| Error rate | counter | > 1% |
| Agent fix success rate | gauge | < 80% |
| Agent fix -> revert rate | gauge | > 10% |
