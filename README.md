# pkgtool - Advanced Universal Linux Package Manager

pkgtool is a high-performance, feature-rich package management interface designed to provide a unified experience across all major Linux distributions. Built with Rust for optimal performance and reliability, pkgtool offers an intuitive terminal user interface while maintaining powerful functionality under the hood.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Version](https://img.shields.io/badge/version-0.3.0-green.svg)
![Rust Version](https://img.shields.io/badge/rust-1.70%2B-orange.svg)

## Features

### Universal Package Management
- Seamless support for multiple package managers:
  - APT (Debian/Ubuntu)
  - Pacman (Arch Linux)
  - DNF (Fedora/RHEL)
  - Zypper (openSUSE)
  - XBPS (Void Linux)
  - Portage (Gentoo)
  - Nix (NixOS)
- Integration with Flatpak, Snap, and AppImage
- Smart detection of available package managers
- Unified command syntax across all distributions

### Advanced Features
- Real-time system monitoring and statistics
- Interactive dependency visualization
- System snapshots and rollbacks
- Security vulnerability scanning
- Package verification and integrity checking
- Smart update scheduling
- Configuration file management
- Repository management
- Batch operations with dependency resolution

### Modern Terminal Interface
- Intuitive tab-based navigation
- Interactive package browsing
- Real-time search with filtering
- Progress visualization
- Resource usage graphs
- Dependency trees
- Color-coded status indicators
- Mouse and keyboard support

## Installation

### Prerequisites
- Rust 1.70 or higher
- System package manager access (sudo privileges)
- SQLite 3.x
- libssl-dev
- pkg-config

### From Source
```bash
# Clone the repository
git clone https://github.com/0xb0urn3/pkgtool.git
cd pkgtool

# Build and install
cargo build --release
sudo cp target/release/pkgtool /usr/local/bin/
```

## Quick Start

Launch pkgtool with:
```bash
pkgtool
```

### Basic Operations

Search for packages:
```bash
pkgtool search <package-name>
```

Install packages:
```bash
pkgtool install <package-name> [package-name2 ...]
```

Remove packages:
```bash
pkgtool remove <package-name> [package-name2 ...]
```

Update system:
```bash
pkgtool update
```

Create system snapshot:
```bash
pkgtool snapshot create "pre-update-snapshot"
```

### Interactive Mode

The interactive mode provides a full terminal user interface:

1. Press `Tab` to switch between sections
2. Use arrow keys for navigation
3. Press `Enter` to select/confirm
4. Press `i` to enter input mode
5. Press `?` for help
6. Press `q` to quit

### Configuration

Configuration file location: `~/.config/pkgtool/config.json`

Example configuration:
```json
{
    "theme": {
        "name": "dark",
        "accent_color": "cyan"
    },
    "update_interval": 3600,
    "security_checks": {
        "enabled": true,
        "auto_fix": false
    },
    "notifications": {
        "enabled": true,
        "updates": true,
        "security": true
    }
}
```

## Advanced Usage

### System Snapshots
Create a snapshot before major changes:
```bash
pkgtool snapshot create "pre-upgrade-$(date +%Y%m%d)"
```

Restore from a snapshot:
```bash
pkgtool snapshot restore "pre-upgrade-20241222"
```

### Security Features
Scan system for vulnerabilities:
```bash
pkgtool security scan
```

Check specific package:
```bash
pkgtool security check firefox
```

### Dependency Analysis
View package dependencies:
```bash
pkgtool deps show nginx
```

Find reverse dependencies:
```bash
pkgtool deps reverse libssl
```

### Distribution-Specific Features

#### Debian/Ubuntu
```bash
# Manage APT pinning
pkgtool apt pin add <package> <version>

# Configure APT repositories
pkgtool apt repo add <repo-url>
```

#### Arch Linux
```bash
# AUR operations
pkgtool aur install <package>
pkgtool aur update
```

#### Fedora
```bash
# DNF module management
pkgtool dnf module enable <module>
pkgtool dnf module install <module>
```

## Troubleshooting

### Common Issues

1. Permission Denied
```bash
sudo chown root:root /usr/local/bin/pkgtool
sudo chmod 755 /usr/local/bin/pkgtool
```

2. Package Manager Not Detected
```bash
# Verify system package manager installation
pkgtool system verify

# Reconfigure package manager detection
pkgtool config --reconfigure
```

3. Database Errors
```bash
# Reset package database
pkgtool db reset

# Rebuild cache
pkgtool cache rebuild
```

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Setup
```bash
# Install development dependencies
cargo install cargo-watch cargo-audit

# Run tests
cargo test

# Run with hot reloading
cargo watch -x run

# Check code style
cargo fmt
cargo clippy
```

### Code Structure
- `src/package_managers/` - Package manager implementations
- `src/ui/` - Terminal user interface components
- `src/features/` - Core functionality implementations
- `src/config/` - Configuration management
- `src/security/` - Security-related features
- `src/utils/` - Utility functions and helpers

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Created by 0xb0urn3
- Inspired by various package managers and system tools
- Thanks to the Rust community for excellent crates

## Support

- GitHub Issues: [Report a bug](https://github.com/0xb0urn3/pkgtool/issues)
- Documentation: [Wiki](https://github.com/0xb0urn3/pkgtool/wiki)
- Discussions: [Community Forum](https://github.com/0xb0urn3/pkgtool/discussions)

## Roadmap

### Upcoming Features
- Container integration (Docker, Podman)
- Remote system management
- Package building tools
- Automated testing framework
- Web interface
- API for external tools

Stay tuned for updates and new features!
