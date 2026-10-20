# Vaultwarden Metrics

Vaultwarden provides comprehensive Prometheus metrics to monitor your password manager instance. This feature allows you to track performance, usage patterns, and system health through standardized metrics.

## Overview

The metrics feature exposes a `/metrics` endpoint that provides Prometheus-compatible metrics. These metrics help administrators monitor:

- **HTTP Performance**: Request rates, response times, and error rates
- **Database Usage**: Connection pools, query performance
- **Authentication Activity**: Login attempts and session management
- **Business Metrics**: User counts, organizations, and vault items
- **System Health**: Uptime, build information, and resource usage

## Configuration

### Enabling Metrics

Metrics are **enabled by default** in Docker builds. For manual builds, add the `enable_metrics` feature flag:

```bash
cargo build --features sqlite,mysql,postgresql,enable_metrics
```

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `METRICS_TOKEN` | Authentication token for accessing metrics (required) | None |
| `METRICS_TOKEN` | Can use Argon2 PHC format for hashed tokens | None |
| `METRICS_BUSINESS_CACHE_SECONDS` | How long to cache expensive business metrics | 300 (5 minutes) |

### Security Considerations

The metrics endpoint requires authentication. Configure `METRICS_TOKEN` with a strong, random token:

```bash
# Plain text token (less secure)
export METRICS_TOKEN="your-secure-random-token-here"

# Argon2 hashed token (recommended)
export METRICS_TOKEN="$argon2id$v=19$m=65536,t=3,p=4$..."
```

**Important**: Never expose the metrics endpoint publicly without authentication. The metrics contain sensitive information about your instance.

## Accessing Metrics

### Endpoint

The metrics are available at the `/metrics` endpoint:

```
GET /metrics
```

### Authentication

Include the token in the `Authorization` header:

```bash
curl -H "Authorization: Bearer your-metrics-token" \
     http://your-vaultwarden-instance/metrics
```

Or as a query parameter:

```bash
curl "http://your-vaultwarden-instance/metrics?token=your-metrics-token"
```

### Response Format

Metrics are returned in Prometheus text format:

```prometheus
# HELP vaultwarden_http_requests_total Total number of HTTP requests processed
# TYPE vaultwarden_http_requests_total counter
vaultwarden_http_requests_total{method="GET",path="/api/sync",status="200"} 1500
vaultwarden_http_requests_total{method="POST",path="/api/accounts/login",status="200"} 45

# HELP vaultwarden_uptime_seconds Uptime in seconds
# TYPE vaultwarden_uptime_seconds gauge
vaultwarden_uptime_seconds{version="1.0.0"} 86400.5
```

## Available Metrics

### HTTP Request Metrics

| Metric | Type | Description | Labels |
|--------|------|-------------|--------|
| `vaultwarden_http_requests_total` | Counter | Total HTTP requests processed | `method`, `path`, `status` |
| `vaultwarden_http_request_duration_seconds` | Histogram | HTTP request duration | `method`, `path` |

### Database Metrics

| Metric | Type | Description | Labels |
|--------|------|-------------|--------|
| `vaultwarden_db_connections_active` | Gauge | Active database connections | `database` |
| `vaultwarden_db_connections_idle` | Gauge | Idle database connections | `database` |
| `vaultwarden_db_query_duration_seconds` | Histogram | Database query duration | `operation` |

### Authentication Metrics

| Metric | Type | Description | Labels |
|--------|------|-------------|--------|
| `vaultwarden_auth_attempts_total` | Counter | Authentication attempts | `method`, `status` |
| `vaultwarden_user_sessions_active` | Gauge | Active user sessions | `user_type` |

### Business Metrics

| Metric | Type | Description | Labels |
|--------|------|-------------|--------|
| `vaultwarden_users_total` | Gauge | Total users | `status` |
| `vaultwarden_organizations_total` | Gauge | Total organizations | `status` |
| `vaultwarden_vault_items_total` | Gauge | Total vault items | `type`, `organization` |
| `vaultwarden_collections_total` | Gauge | Total collections | `organization` |

### System Metrics

| Metric | Type | Description | Labels |
|--------|------|-------------|--------|
| `vaultwarden_uptime_seconds` | Gauge | Instance uptime | `version` |
| `vaultwarden_build_info` | Gauge | Build information | `version`, `revision`, `branch` |

## Monitoring Setup

### Prometheus Configuration

Add to your `prometheus.yml`:

```yaml
scrape_configs:
  - job_name: 'vaultwarden'
    static_configs:
      - targets: ['vaultwarden:80']
    metrics_path: '/metrics'
    params:
      token: ['your-metrics-token']
```

### Grafana Dashboards

Use the metrics to create dashboards showing:

- **Request Rate**: Track API usage patterns
- **Error Rates**: Monitor failed requests and authentication attempts
- **Performance**: Database query times and HTTP response times
- **User Growth**: Monitor user and organization counts
- **System Health**: Uptime and resource usage

### Alerting

Set up alerts for:

- High error rates (>5% of requests)
- Database connection pool exhaustion
- Authentication failures spikes
- Instance downtime

## Performance Considerations

- **Business Metrics Caching**: Expensive metrics (user/organization counts) are cached for 5 minutes by default
- **Cardinality**: Path-based metrics are normalized to prevent high cardinality issues
- **Resource Usage**: Metrics collection has minimal performance impact

## Troubleshooting

### Metrics Not Available

- Ensure `enable_metrics` feature is enabled during build
- Check `METRICS_TOKEN` is configured
- Verify endpoint is accessible: `curl -H "Authorization: Bearer <token>" http://localhost:80/metrics`

### High Cardinality

If you see performance issues, the metrics system automatically normalizes high-cardinality labels like request paths.

### Authentication Issues

- Verify token format (plain text or Argon2 PHC)
- Check token encoding (URL-safe for query parameters)
- Ensure proper header format: `Authorization: Bearer <token>`

## Examples

### Basic Metrics Query

```bash
# Get all metrics
curl -s "http://localhost:80/metrics?token=mytoken" | head -20
```

### Prometheus Query Examples

```promql
# Request rate per minute
rate(vaultwarden_http_requests_total[5m])

# Error rate percentage
rate(vaultwarden_http_requests_total{status=~"5.."}[5m]) / rate(vaultwarden_http_requests_total[5m]) * 100

# Active users
vaultwarden_user_sessions_active

# Database performance
histogram_quantile(0.95, rate(vaultwarden_db_query_duration_seconds_bucket[5m]))
```

### Grafana Dashboard JSON

See the Vaultwarden repository for example Grafana dashboard configurations that visualize these metrics.