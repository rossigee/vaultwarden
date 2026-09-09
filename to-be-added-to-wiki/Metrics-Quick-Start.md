# Metrics Quick Start Guide

Get Vaultwarden metrics up and running in minutes.

## Prerequisites

- Vaultwarden instance (Docker or manual build with `enable_metrics` feature)
- Prometheus or monitoring system
- Basic understanding of Prometheus metrics

## Step 1: Configure Metrics Token

### For Docker

Set the `METRICS_TOKEN` environment variable:

```bash
export METRICS_TOKEN="your-secure-random-token"
```

Or generate an Argon2 hash:

```bash
# Using openssl for a random token
TOKEN=$(openssl rand -hex 32)
echo "METRICS_TOKEN=$TOKEN"
```

### For Docker Compose

Add to your `docker-compose.yml`:

```yaml
environment:
  - METRICS_TOKEN=your-secure-random-token-here
```

## Step 2: Verify Metrics are Working

Test the metrics endpoint:

```bash
curl -H "Authorization: Bearer your-token" \
     http://your-vaultwarden-instance:80/metrics
```

You should see output like:

```prometheus
# HELP vaultwarden_http_requests_total Total number of HTTP requests processed
# TYPE vaultwarden_http_requests_total counter
vaultwarden_http_requests_total{method="GET",path="/",status="200"} 1
# HELP vaultwarden_uptime_seconds Uptime in seconds
# TYPE vaultwarden_uptime_seconds gauge
vaultwarden_uptime_seconds{version="1.0.0"} 45.2
```

## Step 3: Set Up Prometheus

Create a `prometheus.yml` file:

```yaml
global:
  scrape_interval: 30s

scrape_configs:
  - job_name: 'vaultwarden'
    static_configs:
      - targets: ['vaultwarden:80']
    metrics_path: '/metrics'
    params:
      token: ['your-metrics-token-here']
```

Start Prometheus:

```bash
docker run -d \
  -p 9090:9090 \
  -v $(pwd)/prometheus.yml:/etc/prometheus/prometheus.yml \
  prom/prometheus
```

## Step 4: Access Prometheus

Open http://localhost:9090 and verify Vaultwarden metrics are being collected.

Try these queries:

- `vaultwarden_http_requests_total` - Total requests
- `rate(vaultwarden_http_requests_total[5m])` - Request rate
- `vaultwarden_uptime_seconds` - Instance uptime

## Step 5: Set Up Grafana (Optional)

Start Grafana:

```bash
docker run -d \
  -p 3000:3000 \
  -e GF_SECURITY_ADMIN_PASSWORD=admin \
  grafana/grafana
```

1. Open http://localhost:3000 (admin/admin)
2. Add Prometheus as a data source (URL: http://prometheus:9090)
3. Create a dashboard with Vaultwarden metrics

## Common Issues

### No Metrics Available

- Ensure `enable_metrics` feature is enabled in your build
- Check that `METRICS_TOKEN` is set
- Verify Vaultwarden is running and accessible

### Authentication Failed

- Check token is correct
- Ensure proper header format: `Authorization: Bearer <token>`
- Try query parameter: `?token=<token>`

### Prometheus Can't Scrape

- Verify Vaultwarden is accessible from Prometheus container
- Check network connectivity between containers
- Confirm metrics endpoint URL is correct

## Next Steps

- Set up alerting rules for critical metrics
- Create comprehensive Grafana dashboards
- Configure metrics retention and storage
- Integrate with your existing monitoring stack

## Example Queries

```promql
# Request rate
rate(vaultwarden_http_requests_total[5m])

# Error rate %
(rate(vaultwarden_http_requests_total{status=~"5.."}[5m]) /
 rate(vaultwarden_http_requests_total[5m])) * 100

# Active users
vaultwarden_user_sessions_active

# Database connections
vaultwarden_db_connections_active
```

## Security Reminder

- Never expose metrics endpoint publicly
- Use strong, random tokens
- Consider Argon2 hashed tokens for production
- Rotate tokens regularly