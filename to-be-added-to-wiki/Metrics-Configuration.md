# Metrics Configuration Reference

This document details all configuration options related to Vaultwarden's metrics functionality.

## Feature Flags

### enable_metrics

**Type**: Build feature flag
**Default**: Disabled (but enabled by default in Docker builds)
**Description**: Enables the Prometheus metrics endpoint and metrics collection

**Build Command**:
```bash
cargo build --features sqlite,mysql,postgresql,enable_metrics
```

**Docker**: Metrics are enabled by default in official Docker builds.

## Environment Variables

### METRICS_TOKEN

**Type**: String
**Required**: Yes (when metrics are enabled)
**Description**: Authentication token required to access the `/metrics` endpoint

**Format Options**:
1. **Plain text** (not recommended for production):
   ```bash
   METRICS_TOKEN=my-secure-random-token-12345
   ```

2. **Argon2 PHC format** (recommended):
   ```bash
   METRICS_TOKEN=$argon2id$v=19$m=65536,t=3,p=4$abcdefghijklmnopqrstuvwx$yz0123456789
   ```

**Generation**:
```bash
# Generate plain token
openssl rand -hex 32

# Generate Argon2 hash (requires vaultwarden binary)
vaultwarden hash --help
```

### METRICS_BUSINESS_CACHE_SECONDS

**Type**: Integer
**Default**: 300 (5 minutes)
**Description**: How long to cache expensive business metrics (user counts, organization counts, etc.)

**Performance Impact**:
- Lower values: More accurate metrics, higher database load
- Higher values: Reduced database load, less accurate metrics

**Recommended Settings**:
- Development: 60 (1 minute)
- Production: 300 (5 minutes)
- High-traffic: 600 (10 minutes)

## Authentication

The metrics endpoint supports two authentication methods:

### Bearer Token (Header)

```bash
curl -H "Authorization: Bearer your-token-here" \
     http://vaultwarden:80/metrics
```

### Query Parameter

```bash
curl "http://vaultwarden:80/metrics?token=your-token-here"
```

**Security Notes**:
- Always use HTTPS in production
- Rotate tokens regularly
- Use Argon2 hashed tokens for additional security
- Never commit tokens to version control

## Docker Configuration

### Default Docker Build

The official Docker image includes metrics by default:

```dockerfile
ARG DB=sqlite,mysql,postgresql,enable_metrics
```

### Custom Docker Build

To explicitly control metrics in custom builds:

```dockerfile
# Enable metrics
ARG DB=sqlite,mysql,postgresql,enable_metrics

# Disable metrics
ARG DB=sqlite,mysql,postgresql
```

### Docker Compose Example

```yaml
version: '3.8'
services:
  vaultwarden:
    image: vaultwarden/server:latest
    environment:
      - METRICS_TOKEN=$argon2id$v=19$m=65536,t=3,p=4$abcdefghijklmnopqrstuvwx$yz0123456789
      - METRICS_BUSINESS_CACHE_SECONDS=300
    ports:
      - "80:80"
```

## Monitoring Integration

### Prometheus Configuration

```yaml
scrape_configs:
  - job_name: 'vaultwarden'
    static_configs:
      - targets: ['vaultwarden:80']
    metrics_path: '/metrics'
    params:
      token: ['your-metrics-token']
    scrape_interval: 30s
    scrape_timeout: 10s
```

### Metrics Collection Security

- **Network Security**: Only expose metrics endpoint on internal networks
- **Authentication**: Always require METRICS_TOKEN
- **Rate Limiting**: Consider implementing rate limiting for metrics endpoint
- **TLS**: Use HTTPS for all metric collection

## Troubleshooting Configuration

### Metrics Not Working

1. **Check build features**:
   ```bash
   # Verify metrics feature is enabled
   cargo tree | grep prometheus
   ```

2. **Verify token configuration**:
   ```bash
   # Test endpoint access
   curl -H "Authorization: Bearer $METRICS_TOKEN" http://localhost:80/metrics
   ```

3. **Check logs for errors**:
   ```bash
   # Look for metrics-related errors
   docker logs vaultwarden | grep -i metrics
   ```

### Performance Issues

1. **Adjust cache duration**:
   ```bash
   # Increase cache time for high-traffic instances
   METRICS_BUSINESS_CACHE_SECONDS=600
   ```

2. **Monitor resource usage**:
   ```promql
   # Check metrics collection performance
   rate(vaultwarden_http_requests_total{path="/metrics"}[5m])
   ```

## Advanced Configuration

### Custom Build Features

For custom builds with specific metrics features:

```bash
# Minimal metrics build
cargo build --features sqlite,enable_metrics

# Full featured build
cargo build --features sqlite,mysql,postgresql,enable_metrics,query_logger
```

### Development Configuration

```bash
# Development settings
export RUST_LOG=debug
export METRICS_TOKEN=test-token-123
export METRICS_BUSINESS_CACHE_SECONDS=60
```

### Production Configuration

```bash
# Production settings
export METRICS_TOKEN=$ARGON2_HASHED_TOKEN
export METRICS_BUSINESS_CACHE_SECONDS=300
export ROCKET_ENV=production
```