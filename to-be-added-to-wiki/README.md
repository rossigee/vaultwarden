# Vaultwarden Metrics Documentation

This folder contains comprehensive documentation for Vaultwarden's new Prometheus metrics functionality.

## Files Overview

### [Metrics.md](Metrics.md)
**Main documentation** - Complete guide to Vaultwarden's metrics feature including:
- Overview and benefits
- Configuration options
- Authentication methods
- Available metrics summary
- Monitoring setup examples
- Performance considerations

### [Metrics-Configuration.md](Metrics-Configuration.md)
**Configuration reference** - Detailed configuration guide covering:
- Build flags and feature enabling
- Environment variables
- Docker configuration
- Authentication setup
- Troubleshooting configuration issues

### [Metrics-Reference.md](Metrics-Reference.md)
**Technical reference** - Complete reference for all available metrics:
- Detailed metric descriptions
- Label definitions
- Example Prometheus output
- Usage patterns
- Cardinality control
- Performance characteristics

### [Metrics-Examples.md](Metrics-Examples.md)
**Usage examples** - Practical examples and implementations:
- Prometheus configuration
- Grafana dashboard examples
- Alerting rules
- Common queries and patterns
- Troubleshooting with metrics
- Integration examples

### [Metrics-Quick-Start.md](Metrics-Quick-Start.md)
**Quick start guide** - Get metrics running quickly:
- Prerequisites and setup
- Basic configuration
- Verification steps
- Common issues and solutions

## Wiki Integration

These documents are designed to be added to the Vaultwarden wiki. Each file can be:

1. **Copied directly** to the wiki as individual pages
2. **Combined** into a single comprehensive metrics section
3. **Split further** if the wiki has size limits

## Key Features Documented

- **HTTP Metrics**: Request rates, response times, error tracking
- **Database Metrics**: Connection pools, query performance
- **Authentication Metrics**: Login attempts, session tracking
- **Business Metrics**: User/organization counts, vault statistics
- **System Metrics**: Uptime, build information
- **Security**: Token authentication, Argon2 hashing
- **Performance**: Caching, cardinality control, resource usage

## Integration Points

The documentation covers integration with:
- **Prometheus**: Scraping configuration, query examples
- **Grafana**: Dashboard templates, visualization
- **Alerting**: Prometheus rules, thresholds
- **Docker**: Container setup, compose examples
- **Monitoring**: Best practices, troubleshooting

## Maintenance Notes

When updating metrics functionality:
1. Update relevant sections in all affected documents
2. Add new metrics to the reference document
3. Include configuration changes in the configuration guide
4. Add practical examples to the examples document
5. Update quick start if setup process changes

## Contributing

For documentation improvements:
- Ensure technical accuracy
- Include practical examples
- Test all commands and configurations
- Follow consistent formatting
- Update cross-references between documents