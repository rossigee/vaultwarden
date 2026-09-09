# Metrics Examples and Usage

This document provides practical examples of how to use Vaultwarden's metrics for monitoring and alerting.

## Basic Usage

### Accessing Metrics

```bash
# Using curl with header authentication
curl -H "Authorization: Bearer your-token" \
     http://vaultwarden:80/metrics

# Using query parameter
curl "http://vaultwarden:80/metrics?token=your-token"

# Save to file for analysis
curl -s "http://vaultwarden:80/metrics?token=token" > metrics.txt
```

### Docker Compose Setup

```yaml
version: '3.8'
services:
  vaultwarden:
    image: vaultwarden/server:latest
    environment:
      - METRICS_TOKEN=$argon2id$v=19$m=65536,t=3,p=4$abcdefghijklmnopqrstuvwx$yz0123456789
    ports:
      - "80:80"

  prometheus:
    image: prom/prometheus:latest
    volumes:
      - ./prometheus.yml:/etc/prometheus/prometheus.yml
    ports:
      - "9090:9090"

  grafana:
    image: grafana/grafana:latest
    environment:
      - GF_SECURITY_ADMIN_PASSWORD=admin
    ports:
      - "3000:3000"
```

## Prometheus Configuration

### Basic Configuration

```yaml
global:
  scrape_interval: 30s
  scrape_timeout: 10s

scrape_configs:
  - job_name: 'vaultwarden'
    static_configs:
      - targets: ['vaultwarden:80']
    metrics_path: '/metrics'
    params:
      token: ['your-metrics-token-here']
```

### Advanced Configuration

```yaml
scrape_configs:
  - job_name: 'vaultwarden'
    static_configs:
      - targets: ['vaultwarden:80']
    metrics_path: '/metrics'
    params:
      token: ['your-metrics-token-here']
    scrape_interval: 30s
    scrape_timeout: 10s

    # Relabeling
    relabel_configs:
      - source_labels: [__address__]
        target_label: instance
        replacement: 'production-vaultwarden'

    # Metric relabeling to reduce cardinality
    metric_relabel_configs:
      - source_labels: [path]
        regex: '/api/.*'
        target_label: endpoint_type
        replacement: 'api'
```

## Prometheus Queries

### Request Monitoring

```promql
# Total requests per minute
rate(vaultwarden_http_requests_total[5m])

# Requests by method
rate(vaultwarden_http_requests_total[5m]) by (method)

# Requests by status code
rate(vaultwarden_http_requests_total[5m]) by (status)

# Top 10 slowest endpoints (95th percentile)
topk(10, histogram_quantile(0.95, rate(vaultwarden_http_request_duration_seconds_bucket[5m])) by (path))
```

### Error Monitoring

```promql
# Error rate percentage
(rate(vaultwarden_http_requests_total{status=~"5.."}[5m]) /
 rate(vaultwarden_http_requests_total[5m])) * 100

# 4xx error rate
(rate(vaultwarden_http_requests_total{status=~"4.."}[5m]) /
 rate(vaultwarden_http_requests_total[5m])) * 100

# Errors by endpoint
rate(vaultwarden_http_requests_total{status=~"5.."}[5m]) by (path)
```

### Authentication Monitoring

```promql
# Authentication attempts per minute
rate(vaultwarden_auth_attempts_total[5m])

# Failed authentication rate
rate(vaultwarden_auth_attempts_total{status="failed"}[5m])

# Authentication success rate
rate(vaultwarden_auth_attempts_total{status="success"}[5m]) /
rate(vaultwarden_auth_attempts_total[5m])

# Authentication methods usage
rate(vaultwarden_auth_attempts_total[5m]) by (method)
```

### Database Monitoring

```promql
# Database connection utilization
vaultwarden_db_connections_active /
(vaultwarden_db_connections_active + vaultwarden_db_connections_idle)

# Query performance (95th percentile)
histogram_quantile(0.95, rate(vaultwarden_db_query_duration_seconds_bucket[5m]))

# Slow queries (>1 second)
rate(vaultwarden_db_query_duration_seconds_bucket{le="1"}[5m]) by (operation)
```

### Business Metrics

```promql
# User growth rate
rate(vaultwarden_users_total[7d])

# Organization growth
rate(vaultwarden_organizations_total[7d])

# Vault size growth
rate(vaultwarden_vault_items_total[7d])

# Items per organization
vaultwarden_vault_items_total / vaultwarden_organizations_total
```

## Grafana Dashboards

### Dashboard JSON Template

```json
{
  "dashboard": {
    "title": "Vaultwarden Overview",
    "tags": ["vaultwarden"],
    "timezone": "browser",
    "panels": [
      {
        "title": "HTTP Request Rate",
        "type": "graph",
        "targets": [
          {
            "expr": "rate(vaultwarden_http_requests_total[5m])",
            "legendFormat": "{{method}} {{path}}"
          }
        ]
      },
      {
        "title": "Error Rate",
        "type": "stat",
        "targets": [
          {
            "expr": "(rate(vaultwarden_http_requests_total{status=~\"5..\"}[5m]) / rate(vaultwarden_http_requests_total[5m])) * 100",
            "format": "percent"
          }
        ]
      }
    ]
  }
}
```

### Key Metrics Dashboard

```json
{
  "title": "Vaultwarden Key Metrics",
  "rows": [
    {
      "title": "HTTP Performance",
      "panels": [
        {
          "title": "Request Rate",
          "type": "graph",
          "targets": [{"expr": "rate(vaultwarden_http_requests_total[5m])"}]
        },
        {
          "title": "Response Time (95th percentile)",
          "type": "graph",
          "targets": [{"expr": "histogram_quantile(0.95, rate(vaultwarden_http_request_duration_seconds_bucket[5m]))"}]
        }
      ]
    },
    {
      "title": "Authentication",
      "panels": [
        {
          "title": "Active Sessions",
          "type": "stat",
          "targets": [{"expr": "vaultwarden_user_sessions_active"}]
        },
        {
          "title": "Failed Auth Attempts",
          "type": "graph",
          "targets": [{"expr": "rate(vaultwarden_auth_attempts_total{status=\"failed\"}[5m])"}]
        }
      ]
    }
  ]
}
```

## Alerting Rules

### Prometheus Alerting Rules

```yaml
groups:
  - name: vaultwarden
    rules:
      - alert: VaultwardenDown
        expr: up{job="vaultwarden"} == 0
        for: 5m
        labels:
          severity: critical
        annotations:
          summary: "Vaultwarden instance is down"
          description: "Vaultwarden has been down for more than 5 minutes."

      - alert: VaultwardenHighErrorRate
        expr: (rate(vaultwarden_http_requests_total{status=~"5.."}[5m]) / rate(vaultwarden_http_requests_total[5m])) * 100 > 5
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "High error rate detected"
          description: "Error rate is {{ $value }}% which is above 5%."

      - alert: VaultwardenSlowResponses
        expr: histogram_quantile(0.95, rate(vaultwarden_http_request_duration_seconds_bucket[5m])) > 2
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "Slow response times detected"
          description: "95th percentile response time is {{ $value }}s."

      - alert: VaultwardenAuthFailures
        expr: rate(vaultwarden_auth_attempts_total{status="failed"}[5m]) > 10
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "High authentication failure rate"
          description: "More than 10 authentication failures per minute."

      - alert: VaultwardenDbConnectionIssues
        expr: vaultwarden_db_connections_active / (vaultwarden_db_connections_active + vaultwarden_db_connections_idle) > 0.9
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "Database connection pool nearly exhausted"
          description: "Database connection utilization is {{ $value }}%."
```

## Monitoring Best Practices

### Key Metrics to Monitor

1. **Availability**: Instance uptime and response codes
2. **Performance**: Response times and database query times
3. **Security**: Authentication failures and unusual patterns
4. **Capacity**: User growth, vault size, connection pools
5. **Business**: Organization growth and feature adoption

### Alert Thresholds

- **Error Rate**: >5% for 5 minutes
- **Response Time**: >2 seconds (95th percentile) for 5 minutes
- **Auth Failures**: >10 per minute for 5 minutes
- **Connection Pool**: >90% utilization for 5 minutes
- **Uptime**: Any downtime >5 minutes

### Dashboard Organization

```
Vaultwarden Overview
├── HTTP Performance
│   ├── Request Rate
│   ├── Response Times
│   └── Error Rates
├── Authentication
│   ├── Active Sessions
│   ├── Auth Attempts
│   └── 2FA Usage
├── Database
│   ├── Connection Pools
│   ├── Query Performance
│   └── Connection Utilization
├── Business Metrics
│   ├── User Growth
│   ├── Organizations
│   └── Vault Items
└── System Health
    ├── Uptime
    ├── Build Info
    └── Resource Usage
```

## Troubleshooting with Metrics

### High Error Rates

```promql
# Find endpoints with high errors
rate(vaultwarden_http_requests_total{status=~"5.."}[5m]) by (path) /
rate(vaultwarden_http_requests_total[5m]) by (path) > 0.1
```

### Slow Performance

```promql
# Identify slow endpoints
histogram_quantile(0.95, rate(vaultwarden_http_request_duration_seconds_bucket[5m])) by (path) > 1
```

### Database Issues

```promql
# Check connection pool health
vaultwarden_db_connections_active /
(vaultwarden_db_connections_active + vaultwarden_db_connections_idle) > 0.8
```

### Security Monitoring

```promql
# Unusual authentication patterns
rate(vaultwarden_auth_attempts_total{status="failed"}[1h]) by (method) >
rate(vaultwarden_auth_attempts_total{status="failed"}[24h]) by (method) * 2
```

## Integration Examples

### ELK Stack

```bash
# Export metrics to Elasticsearch
curl -s "http://vaultwarden:80/metrics?token=token" |
prometheus-metrics-to-elasticsearch --elasticsearch-url http://elasticsearch:9200
```

### Custom Monitoring Scripts

```python
#!/usr/bin/env python3
import requests
import time

METRICS_URL = "http://vaultwarden:80/metrics"
TOKEN = "your-token"

def get_metrics():
    response = requests.get(METRICS_URL, params={"token": TOKEN})
    return response.text

def parse_metric_value(metric_name, metrics_text):
    for line in metrics_text.split('\n'):
        if line.startswith(metric_name):
            return float(line.split('} ')[1])
    return 0

# Monitor uptime
metrics = get_metrics()
uptime = parse_metric_value('vaultwarden_uptime_seconds', metrics)
print(f"Vaultwarden uptime: {uptime} seconds")
```