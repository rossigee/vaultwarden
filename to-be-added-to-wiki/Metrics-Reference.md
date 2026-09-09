# Metrics Reference

This document provides detailed information about all Prometheus metrics exposed by Vaultwarden.

## Metric Types

- **Counter**: Monotonically increasing value (e.g., total requests)
- **Gauge**: Value that can increase or decrease (e.g., active connections)
- **Histogram**: Distribution of values with buckets (e.g., request durations)

## HTTP Request Metrics

### vaultwarden_http_requests_total

**Type**: Counter
**Description**: Total number of HTTP requests processed by Vaultwarden

**Labels**:
- `method`: HTTP method (GET, POST, PUT, DELETE, etc.)
- `path`: Normalized request path
- `status`: HTTP status code (200, 401, 500, etc.)

**Example**:
```prometheus
vaultwarden_http_requests_total{method="GET",path="/api/sync",status="200"} 1500
vaultwarden_http_requests_total{method="POST",path="/api/accounts/login",status="401"} 45
```

**Usage**:
- Monitor API usage patterns
- Track error rates by endpoint
- Identify performance bottlenecks

### vaultwarden_http_request_duration_seconds

**Type**: Histogram
**Description**: Time taken to process HTTP requests

**Labels**:
- `method`: HTTP method
- `path`: Normalized request path

**Buckets**: 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0 seconds

**Example**:
```prometheus
vaultwarden_http_request_duration_seconds_bucket{method="GET",path="/api/sync",le="0.1"} 1200
vaultwarden_http_request_duration_seconds_bucket{method="GET",path="/api/sync",le="0.25"} 1480
vaultwarden_http_request_duration_seconds_sum{method="GET",path="/api/sync"} 250.5
vaultwarden_http_request_duration_seconds_count{method="GET",path="/api/sync"} 1500
```

**Usage**:
- Monitor API response times
- Set SLOs for request latency
- Identify slow endpoints

## Database Metrics

### vaultwarden_db_connections_active

**Type**: Gauge
**Description**: Number of active database connections

**Labels**:
- `database`: Database type (sqlite, postgresql, mysql)

**Example**:
```prometheus
vaultwarden_db_connections_active{database="sqlite"} 5
vaultwarden_db_connections_active{database="postgresql"} 8
```

**Usage**:
- Monitor database connection pool usage
- Alert when approaching connection limits
- Optimize connection pool size

### vaultwarden_db_connections_idle

**Type**: Gauge
**Description**: Number of idle database connections

**Labels**:
- `database`: Database type

**Example**:
```prometheus
vaultwarden_db_connections_idle{database="sqlite"} 10
vaultwarden_db_connections_idle{database="postgresql"} 2
```

**Usage**:
- Monitor connection pool efficiency
- Identify connection leaks
- Tune connection pool configuration

### vaultwarden_db_query_duration_seconds

**Type**: Histogram
**Description**: Time taken to execute database queries

**Labels**:
- `operation`: Query operation type (select, insert, update, delete)

**Buckets**: 0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0 seconds

**Example**:
```prometheus
vaultwarden_db_query_duration_seconds_bucket{operation="select",le="0.01"} 850
vaultwarden_db_query_duration_seconds_sum{operation="select"} 45.2
vaultwarden_db_query_duration_seconds_count{operation="select"} 900
```

**Usage**:
- Monitor database performance
- Identify slow queries
- Track query optimization impact

## Authentication Metrics

### vaultwarden_auth_attempts_total

**Type**: Counter
**Description**: Total number of authentication attempts

**Labels**:
- `method`: Authentication method (password, webauthn, totp, duo, etc.)
- `status`: Authentication result (success, failed)

**Example**:
```prometheus
vaultwarden_auth_attempts_total{method="password",status="success"} 1200
vaultwarden_auth_attempts_total{method="password",status="failed"} 45
vaultwarden_auth_attempts_total{method="webauthn",status="success"} 300
```

**Usage**:
- Monitor authentication patterns
- Detect brute force attempts
- Track 2FA adoption

### vaultwarden_user_sessions_active

**Type**: Gauge
**Description**: Number of currently active user sessions

**Labels**:
- `user_type`: Type of user (authenticated, anonymous)

**Example**:
```prometheus
vaultwarden_user_sessions_active{user_type="authenticated"} 150
vaultwarden_user_sessions_active{user_type="anonymous"} 5
```

**Usage**:
- Monitor concurrent user load
- Plan capacity requirements
- Track user engagement

## Business Metrics

### vaultwarden_users_total

**Type**: Gauge
**Description**: Total number of users in the system

**Labels**:
- `status`: User status (enabled, disabled)

**Example**:
```prometheus
vaultwarden_users_total{status="enabled"} 1200
vaultwarden_users_total{status="disabled"} 45
```

**Usage**:
- Track user growth
- Monitor account lifecycle
- Plan resource allocation

### vaultwarden_organizations_total

**Type**: Gauge
**Description**: Total number of organizations

**Labels**:
- `status`: Organization status (active)

**Example**:
```prometheus
vaultwarden_organizations_total{status="active"} 25
```

**Usage**:
- Monitor business adoption
- Track organizational growth
- Plan organizational features

### vaultwarden_vault_items_total

**Type**: Gauge
**Description**: Total number of items in user vaults

**Labels**:
- `type`: Item type (login, secure_note, card, identity)
- `organization`: Organization UUID (empty for personal items)

**Example**:
```prometheus
vaultwarden_vault_items_total{type="login",organization=""} 5000
vaultwarden_vault_items_total{type="login",organization="org-uuid-123"} 1200
vaultwarden_vault_items_total{type="card",organization=""} 150
```

**Usage**:
- Monitor vault size growth
- Track item type distribution
- Identify storage requirements

### vaultwarden_collections_total

**Type**: Gauge
**Description**: Total number of collections per organization

**Labels**:
- `organization`: Organization UUID

**Example**:
```prometheus
vaultwarden_collections_total{organization="org-uuid-123"} 15
vaultwarden_collections_total{organization="org-uuid-456"} 8
```

**Usage**:
- Monitor organizational structure
- Track collection management
- Plan organizational scaling

## System Metrics

### vaultwarden_uptime_seconds

**Type**: Gauge
**Description**: Instance uptime in seconds since start

**Labels**:
- `version`: Vaultwarden version

**Example**:
```prometheus
vaultwarden_uptime_seconds{version="1.0.0"} 86400.5
```

**Usage**:
- Monitor instance availability
- Track restart frequency
- Calculate uptime percentages

### vaultwarden_build_info

**Type**: Gauge (always 1)
**Description**: Build information and metadata

**Labels**:
- `version`: Version number
- `revision`: Git commit hash
- `branch`: Git branch

**Example**:
```prometheus
vaultwarden_build_info{version="1.0.0",revision="abcd1234",branch="main"} 1
```

**Usage**:
- Track deployment versions
- Monitor update status
- Correlate metrics with code changes

## Metric Collection Details

### Update Frequency

- **HTTP Metrics**: Updated per request
- **Database Metrics**: Updated on connection pool changes
- **Authentication Metrics**: Updated per auth attempt
- **Business Metrics**: Cached for 5 minutes (configurable)
- **System Metrics**: Updated periodically

### Cardinality Control

- Request paths are normalized to prevent high cardinality
- Organization UUIDs are used instead of names
- Sensitive information is not exposed in labels

### Performance Impact

- Metrics collection is designed to be lightweight
- Business metrics use caching to reduce database load
- Histogram buckets are pre-defined to minimize memory usage

## Prometheus Queries

### Common Queries

```promql
# Request rate per minute
rate(vaultwarden_http_requests_total[5m])

# Error rate percentage
rate(vaultwarden_http_requests_total{status=~"5.."}[5m]) /
rate(vaultwarden_http_requests_total[5m]) * 100

# P95 response time
histogram_quantile(0.95, rate(vaultwarden_http_request_duration_seconds_bucket[5m]))

# Active users
vaultwarden_user_sessions_active{user_type="authenticated"}

# Database connection utilization
vaultwarden_db_connections_active / (vaultwarden_db_connections_active + vaultwarden_db_connections_idle)
```

### Alerting Queries

```promql
# High error rate
rate(vaultwarden_http_requests_total{status=~"5.."}[5m]) /
rate(vaultwarden_http_requests_total[5m]) > 0.05

# Slow requests
histogram_quantile(0.95, rate(vaultwarden_http_request_duration_seconds_bucket[5m])) > 2

# Authentication failures
rate(vaultwarden_auth_attempts_total{status="failed"}[5m]) > 10

# Instance down
up{job="vaultwarden"} == 0
```

## Troubleshooting

### Missing Metrics

- Verify `enable_metrics` feature is enabled
- Check `METRICS_TOKEN` configuration
- Ensure endpoint is accessible

### Inaccurate Counts

- Business metrics are cached (default 5 minutes)
- Check `METRICS_BUSINESS_CACHE_SECONDS` setting
- Restart instance to force cache refresh

### High Cardinality

- Metrics automatically normalize paths
- Monitor for unexpected label values
- Check for application configuration issues