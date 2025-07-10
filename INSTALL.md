# 🚀 GDK Installation Guide

## Quick Installation

### Download Release Binary

```bash
# Download latest release
curl -L https://github.com/KooshaPari/GDK/releases/latest/download/gdk-cli-$(uname -s)-$(uname -m).tar.gz -o gdk.tar.gz

# Extract and install
tar -xzf gdk.tar.gz
sudo mv gdk-cli /usr/local/bin/
sudo chmod +x /usr/local/bin/gdk-cli

# Verify installation
gdk-cli --help
```

### Build from Source

```bash
# Clone repository
git clone https://github.com/KooshaPari/GDK.git
cd GDK

# Build production release
cargo build --release --locked

# Add to PATH (choose one method)
```

## Adding to PATH

### Method 1: System Installation (Recommended)
```bash
# Install system-wide (requires sudo)
sudo cp target/release/gdk-cli /usr/local/bin/
sudo chmod +x /usr/local/bin/gdk-cli
```

### Method 2: User PATH (Current Session)
```bash
# Add to current session
export PATH="$(pwd)/target/release:$PATH"
```

### Method 3: Permanent User PATH
```bash
# For Bash users
echo 'export PATH="'$(pwd)'/target/release:$PATH"' >> ~/.bashrc
source ~/.bashrc

# For Zsh users (macOS default)
echo 'export PATH="'$(pwd)'/target/release:$PATH"' >> ~/.zshrc
source ~/.zshrc

# For Fish users
echo 'set -gx PATH '$(pwd)'/target/release $PATH' >> ~/.config/fish/config.fish
```

### Method 4: Symlink (Alternative)
```bash
# Create symlink in user bin directory
mkdir -p ~/.local/bin
ln -sf $(pwd)/target/release/gdk-cli ~/.local/bin/gdk-cli

# Ensure ~/.local/bin is in PATH
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

## Verification

```bash
# Check installation
which gdk-cli
gdk-cli --help

# Test enterprise features
gdk-cli init --agent-id test-agent
gdk-cli status --agent-id test-agent
```

## Enterprise Installation

For enterprise deployments, see [ENTERPRISE.md](ENTERPRISE.md) for complete production installation guides including:

- Docker containerization
- Kubernetes deployment
- High-availability configuration
- Security hardening
- Monitoring setup

## Troubleshooting

### Command Not Found
```bash
# Check if binary exists
ls -la target/release/gdk-cli

# Check PATH
echo $PATH

# Reload shell configuration
source ~/.zshrc  # or ~/.bashrc
```

### Permission Issues
```bash
# Fix permissions
chmod +x target/release/gdk-cli

# Or use sudo for system installation
sudo cp target/release/gdk-cli /usr/local/bin/
```

### macOS Security Warning
```bash
# If macOS blocks execution
sudo xattr -rd com.apple.quarantine target/release/gdk-cli
```

## Development Setup

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install development dependencies
cargo install cargo-watch cargo-audit

# Run in development mode
cargo run --bin gdk-cli -- --help
```

## Enterprise Support

For enterprise installation support:
- **Email**: enterprise@gdk.dev
- **Documentation**: [ENTERPRISE.md](ENTERPRISE.md)
- **Security**: [SECURITY.md](SECURITY.md)