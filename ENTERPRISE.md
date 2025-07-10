# 🏢 GDK Enterprise Deployment Guide

## Enterprise Production Deployment

This guide provides comprehensive instructions for deploying GDK in enterprise production environments with optimal security, performance, and reliability.

## 🚀 Quick Enterprise Setup

### Prerequisites
- Rust 1.75+ (recommended: latest stable)
- Git 2.30+
- Linux/macOS/Windows (production tested)
- Minimum 4GB RAM, 8+ cores recommended
- Network access for git operations

### Production Installation

```bash
# Clone and build production release
git clone https://github.com/KooshaPari/GDK.git
cd GDK
cargo build --release --locked

# Install to system path
sudo cp target/release/gdk-cli /usr/local/bin/
sudo chmod +x /usr/local/bin/gdk-cli

# Verify installation
gdk-cli --version
```

## 🔧 Enterprise Configuration

### Environment Variables

```bash
# Production Quality Settings
export GDK_CONVERGENCE_THRESHOLD=0.95          # High quality threshold
export GDK_MAX_ITERATIONS=100                  # Maximum convergence attempts
export GDK_ENTERPRISE_MODE=true                # Enable enterprise features

# Performance Optimization
export GDK_THREAD_POOL_SIZE=16                 # CPU cores * 2
export GDK_BATCH_SIZE=1000                     # Batch processing size
export GDK_CACHE_SIZE=10000                    # LRU cache entries
export GDK_MEMORY_LIMIT=8192                   # Memory limit in MB

# Security Settings
export GDK_AUDIT_MODE=true                     # Enable audit logging
export GDK_VALIDATE_INPUTS=true                # Strict input validation
export GDK_SECURE_MODE=true                    # Enhanced security checks

# Monitoring & Observability
export GDK_METRICS_ENABLED=true                # Enable metrics collection
export GDK_METRICS_PORT=9090                   # Prometheus metrics port
export GDK_LOG_LEVEL=info                      # Logging level
export GDK_TRACE_ENABLED=true                  # Distributed tracing
```

### Configuration File

Create `/etc/gdk/config.toml`:

```toml
[enterprise]
mode = true
audit_enabled = true
compliance_mode = "soc2"
security_level = "high"

[performance]
thread_pool_size = 16
batch_size = 1000
cache_size = 10000
memory_limit = 8192
numa_aware = true

[quality]
convergence_threshold = 0.95
max_iterations = 100
strict_validation = true
quality_gates = [
    { name = "lint", threshold = 0.9 },
    { name = "tests", threshold = 0.95 },
    { name = "security", threshold = 1.0 }
]

[monitoring]
metrics_enabled = true
metrics_port = 9090
health_check_port = 8080
prometheus_endpoint = "/metrics"
grafana_dashboard = true

[logging]
level = "info"
format = "json"
audit_file = "/var/log/gdk/audit.log"
error_file = "/var/log/gdk/error.log"
rotation = "daily"
retention_days = 90

[security]
tls_enabled = true
certificate_path = "/etc/gdk/certs/server.crt"
private_key_path = "/etc/gdk/certs/server.key"
allowed_hosts = ["*.company.com"]
rate_limiting = true
max_requests_per_minute = 1000
```

## 🏗️ Production Architecture

### Single Node Deployment

```bash
# Production server setup
sudo mkdir -p /opt/gdk/{bin,config,logs,data}
sudo cp target/release/gdk-cli /opt/gdk/bin/
sudo cp config.toml /opt/gdk/config/

# Systemd service
sudo tee /etc/systemd/system/gdk.service > /dev/null <<EOF
[Unit]
Description=GDK Enterprise Service
After=network.target

[Service]
Type=simple
User=gdk
Group=gdk
WorkingDirectory=/opt/gdk
Environment=GDK_CONFIG_PATH=/opt/gdk/config/config.toml
ExecStart=/opt/gdk/bin/gdk-cli server --enterprise
Restart=always
RestartSec=10
LimitNOFILE=65536

[Install]
WantedBy=multi-user.target
EOF

sudo systemctl enable gdk
sudo systemctl start gdk
```

### High Availability Cluster

```yaml
# docker-compose.yml for HA deployment
version: '3.8'
services:
  gdk-node-1:
    image: gdk:enterprise
    environment:
      - GDK_NODE_ID=node-1
      - GDK_CLUSTER_MODE=true
      - GDK_REDIS_URL=redis://redis:6379
    volumes:
      - gdk-data-1:/data
    ports:
      - "8080:8080"
      - "9090:9090"
    
  gdk-node-2:
    image: gdk:enterprise
    environment:
      - GDK_NODE_ID=node-2
      - GDK_CLUSTER_MODE=true
      - GDK_REDIS_URL=redis://redis:6379
    volumes:
      - gdk-data-2:/data
    ports:
      - "8081:8080"
      - "9091:9090"
    
  redis:
    image: redis:7-alpine
    volumes:
      - redis-data:/data
    
  nginx:
    image: nginx:alpine
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf
      - ./certs:/etc/nginx/certs

volumes:
  gdk-data-1:
  gdk-data-2:
  redis-data:
```

## 🔒 Enterprise Security

### TLS/SSL Configuration

```bash
# Generate enterprise certificates
openssl req -x509 -newkey rsa:4096 -keyout server.key -out server.crt -days 365 -nodes
sudo mkdir -p /etc/gdk/certs
sudo cp server.* /etc/gdk/certs/
sudo chmod 600 /etc/gdk/certs/server.key
sudo chmod 644 /etc/gdk/certs/server.crt
```

### Security Hardening

```bash
# Create dedicated user
sudo useradd -r -s /bin/false gdk
sudo chown -R gdk:gdk /opt/gdk
sudo chmod 750 /opt/gdk

# Firewall configuration
sudo ufw allow 8080/tcp  # Health checks
sudo ufw allow 9090/tcp  # Metrics
sudo ufw allow 443/tcp   # HTTPS
sudo ufw enable

# SELinux policies (RHEL/CentOS)
sudo setsebool -P httpd_can_network_connect 1
sudo semanage port -a -t http_port_t -p tcp 8080
sudo semanage port -a -t http_port_t -p tcp 9090
```

### Authentication & Authorization

```toml
# Add to config.toml
[auth]
enabled = true
provider = "oauth2"
oauth2_endpoint = "https://auth.company.com"
client_id = "gdk-enterprise"
scopes = ["gdk:read", "gdk:write", "gdk:admin"]

[rbac]
enabled = true
roles = [
    { name = "viewer", permissions = ["read"] },
    { name = "operator", permissions = ["read", "write"] },
    { name = "admin", permissions = ["read", "write", "admin"] }
]
```

## 📊 Monitoring & Observability

### Prometheus Integration

```yaml
# prometheus.yml
global:
  scrape_interval: 15s

scrape_configs:
  - job_name: 'gdk'
    static_configs:
      - targets: ['localhost:9090']
    metrics_path: /metrics
    scrape_interval: 5s
```

### Grafana Dashboard

```json
{
  "dashboard": {
    "title": "GDK Enterprise Dashboard",
    "panels": [
      {
        "title": "Agent Success Rate",
        "type": "stat",
        "targets": [{
          "expr": "gdk_agent_success_rate"
        }]
      },
      {
        "title": "Convergence Time",
        "type": "graph",
        "targets": [{
          "expr": "gdk_convergence_duration_seconds"
        }]
      },
      {
        "title": "Quality Score Trend",
        "type": "graph",
        "targets": [{
          "expr": "gdk_quality_score"
        }]
      }
    ]
  }
}
```

### Health Checks

```bash
# Kubernetes health checks
apiVersion: v1
kind: Pod
spec:
  containers:
  - name: gdk
    image: gdk:enterprise
    livenessProbe:
      httpGet:
        path: /health
        port: 8080
      initialDelaySeconds: 30
      periodSeconds: 10
    readinessProbe:
      httpGet:
        path: /ready
        port: 8080
      initialDelaySeconds: 5
      periodSeconds: 5
```

## 🔄 Backup & Recovery

### Data Backup Strategy

```bash
#!/bin/bash
# Enterprise backup script
BACKUP_DIR="/backup/gdk/$(date +%Y%m%d_%H%M%S)"
sudo mkdir -p "$BACKUP_DIR"

# Backup configuration
sudo cp -r /opt/gdk/config "$BACKUP_DIR/"

# Backup data and logs
sudo cp -r /opt/gdk/data "$BACKUP_DIR/"
sudo cp -r /opt/gdk/logs "$BACKUP_DIR/"

# Create archive
sudo tar -czf "$BACKUP_DIR.tar.gz" "$BACKUP_DIR"
sudo rm -rf "$BACKUP_DIR"

# Retention (keep 30 days)
find /backup/gdk -name "*.tar.gz" -mtime +30 -delete
```

### Disaster Recovery

```bash
#!/bin/bash
# Disaster recovery script
BACKUP_FILE="$1"

if [ -z "$BACKUP_FILE" ]; then
    echo "Usage: $0 <backup_file.tar.gz>"
    exit 1
fi

# Stop services
sudo systemctl stop gdk

# Restore from backup
sudo tar -xzf "$BACKUP_FILE" -C /tmp/
sudo cp -r /tmp/gdk_backup_*/config/* /opt/gdk/config/
sudo cp -r /tmp/gdk_backup_*/data/* /opt/gdk/data/

# Fix permissions
sudo chown -R gdk:gdk /opt/gdk

# Restart services
sudo systemctl start gdk
```

## 🚀 Performance Tuning

### System Optimization

```bash
# Kernel parameters for high performance
echo 'net.core.somaxconn = 1024' | sudo tee -a /etc/sysctl.conf
echo 'net.core.netdev_max_backlog = 5000' | sudo tee -a /etc/sysctl.conf
echo 'net.ipv4.tcp_max_syn_backlog = 1024' | sudo tee -a /etc/sysctl.conf
sudo sysctl -p

# File descriptor limits
echo 'gdk soft nofile 65536' | sudo tee -a /etc/security/limits.conf
echo 'gdk hard nofile 65536' | sudo tee -a /etc/security/limits.conf
```

### JVM-style Memory Management

```bash
# Memory optimization
export GDK_MEMORY_LIMIT=8192
export GDK_HEAP_SIZE=6144
export GDK_STACK_SIZE=2048
export GDK_GC_THREADS=4
```

## 📋 Maintenance & Operations

### Log Management

```bash
# Logrotate configuration
sudo tee /etc/logrotate.d/gdk > /dev/null <<EOF
/var/log/gdk/*.log {
    daily
    rotate 30
    compress
    delaycompress
    missingok
    notifempty
    create 644 gdk gdk
    postrotate
        systemctl reload gdk
    endscript
}
EOF
```

### Update Procedure

```bash
#!/bin/bash
# Enterprise update script
set -e

echo "Starting GDK enterprise update..."

# Backup current installation
./backup.sh

# Download new version
curl -L https://github.com/KooshaPari/GDK/releases/latest/download/gdk-linux-x86_64.tar.gz -o gdk-latest.tar.gz

# Verify checksum
curl -L https://github.com/KooshaPari/GDK/releases/latest/download/checksums.txt -o checksums.txt
sha256sum -c checksums.txt

# Stop service
sudo systemctl stop gdk

# Update binary
sudo tar -xzf gdk-latest.tar.gz -C /opt/gdk/bin/

# Test configuration
sudo -u gdk /opt/gdk/bin/gdk-cli validate-config

# Start service
sudo systemctl start gdk

# Verify health
sleep 10
curl -f http://localhost:8080/health

echo "Update completed successfully"
```

## 🔍 Troubleshooting

### Common Issues

1. **High Memory Usage**
   ```bash
   # Check memory usage
   gdk-cli metrics memory
   
   # Reduce cache size
   export GDK_CACHE_SIZE=5000
   ```

2. **Slow Convergence**
   ```bash
   # Check performance metrics
   gdk-cli metrics performance
   
   # Increase thread pool
   export GDK_THREAD_POOL_SIZE=32
   ```

3. **Certificate Issues**
   ```bash
   # Verify certificates
   openssl x509 -in /etc/gdk/certs/server.crt -text -noout
   
   # Regenerate if expired
   ./generate-certs.sh
   ```

### Support Channels

- **Enterprise Support**: enterprise@gdk.dev
- **Emergency Hotline**: +1-555-GDK-HELP
- **Status Page**: https://status.gdk.dev
- **Documentation**: https://docs.gdk.dev

---

## ✅ Production Checklist

- [ ] Hardware requirements met (8+ cores, 16GB+ RAM)
- [ ] TLS certificates configured and valid
- [ ] Authentication and authorization configured
- [ ] Monitoring and alerting setup
- [ ] Backup and recovery procedures tested
- [ ] Security hardening applied
- [ ] Performance tuning completed
- [ ] Health checks configured
- [ ] Log rotation setup
- [ ] Update procedures documented
- [ ] Team training completed
- [ ] Disaster recovery plan tested

**For enterprise support and consulting, contact: enterprise@gdk.dev**