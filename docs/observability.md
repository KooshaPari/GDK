# gdk Observability

`gdk` exports OpenTelemetry-compatible traces via `pheno-tracing` (ADR-012, ADR-036).

## Quickstart

```bash
# 1. Start a collector (any OTLP/gRPC compatible receiver)
docker run --rm -p 4317:4317 otel/opentelemetry-collector-contrib:0.96.0

# 2. Run gdk
export OTEL_EXPORTER_OTLP_ENDPOINT=http://localhost:4317
export OTEL_SERVICE_NAME=gdk
gdk ...
```

Spans and metrics flow: `gdk` → `pheno-tracing` (TracePort) → OTLP/gRPC → collector → backend.

## API surface

- `gdk::observability::otlp_endpoint()` — reads `OTEL_EXPORTER_OTLP_ENDPOINT` or returns the local default.
- `gdk::observability::emit_span(name, attrs)` — builds + submits a `TraceOperation` to the configured backend.
- `gdk::observability::info!`, `warn!`, `error!`, `instrument` — re-exported from `pheno_tracing::compat` so existing `tracing` call sites work unchanged.
- `gdk::observability::SERVICE_NAME` — `"gdk"`, used as the OpenTelemetry `service.name` resource attribute.

## What is exported

- **Spans**: every `observability::emit_span` call plus the upstream `tracing` macros.
- **Metrics**: `requests_total` (Counter<u64>) and `request_duration_seconds` (Histogram<f64>) — wired through the upstream `tracing-subscriber` registry.

## CI smoke

`.github/workflows/observability-smoke.yml` spins up `otel/opentelemetry-collector-contrib:0.96.0` and asserts OTLP gRPC port 4317 is reachable.
