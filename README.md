# PKGTool

PKGTool is an advanced package management utility that provides a modern, feature-rich interface for managing system packages across multiple Linux distributions and macOS. It combines powerful package management capabilities with system monitoring and analytics to give users comprehensive control over their system's software.

## Features

PKGTool offers an extensive set of features designed to enhance your package management experience:

### Package Management
- Multi-platform support (apt, pacman, dnf, and Homebrew)
- Smart package search and discovery
- Batch package installation and removal
- Package version control and rollback capabilities
- Dependency tracking and analysis
- Package hold/unhold functionality
- Automated system updates
- Cache cleaning and orphan package removal

### System Monitoring
- Real-time CPU, memory, and disk usage tracking
- Network traffic monitoring
- Historical performance metrics
- Package operation logging
- System resource visualization
- Custom alert thresholds

### User Interface
- Modern command-line interface with rich formatting
- Interactive menus and dashboards
- Progress tracking for long-running operations
- Detailed operation history
- Customizable themes
- Comprehensive system statistics

## Installation

### Prerequisites
- Python 3.7 or higher
- pip (Python package installer)
- System package manager (apt, pacman, dnf, or Homebrew)
- Administrative privileges for package operations

### Dependencies
```bash
pip install rich psutil aiohttp asyncio
```

### Basic Installation
```bash
git clone https://github.com/0xb0urn3/pkg-tool.git
cd pkg-tool
pip install -e .
```

## Usage

### Starting PKGTool
```bash
pkg-tool
```

### Main Menu Options
1. 🔍 Search Packages - Search for packages in repositories
2. 📦 Install Packages - Install new packages
3. 🗑️ Remove Packages - Remove installed packages
4. 🔄 Update System - Update system and packages
5. 🧹 Clean System - Clean package cache and orphans
6. ℹ️ Package Info - Display package information
7. 📋 List Packages - List installed packages
8. 🔒 Hold Package - Prevent package updates
9. 🔓 Unhold Package - Allow package updates
10. 🔗 Dependencies - Show package dependencies
11. 📊 Package Size - Show package disk usage
12. 📜 History - View operation history
13. 💾 Backup - Create system backup
14. 📈 Monitor - System monitoring dashboard
15. ⚙️ Settings - Configure tool settings

### Examples

Search for a package:
```bash
# From the main menu, select option 1
Enter package name: python3
```

Install multiple packages:
```bash
# From the main menu, select option 2
Enter package name(s): git vim tmux
```

View system metrics:
```bash
# From the main menu, select option 14
# Interactive dashboard will appear with real-time metrics
```

## Configuration

PKGTool stores its configuration in `~/.pkg_tool/config.json`. You can modify these settings through the Settings menu (option 15) or by directly editing the configuration file.

### Key Configuration Options
- Monitoring interval
- Alert thresholds
- UI theme preferences
- Backup settings
- History retention period

## Database

PKGTool maintains several SQLite databases in the `~/.pkg_tool/` directory:
- `monitoring.db`: System metrics and performance data
- `packages.db`: Package information and status
- `history.json`: Operation history

## Troubleshooting

### Common Issues

1. Permission Errors
```bash
sudo chown -R $USER:$USER ~/.pkg_tool
```

2. Database Errors
```bash
pkg-tool --repair-db
```

3. Package Manager Detection
```bash
pkg-tool --detect-manager
```

## Contributing

We welcome contributions to PKGTool! Please follow these steps:

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to your branch
5. Create a Pull Request

### Development Setup
```bash
git clone https://github.com/yourusername/pkg-tool.git
cd pkg-tool
pip install -e ".[dev]"
pytest
```

## License

PKGTool is released under the MIT License. See the LICENSE file for details.

## Credits

- Created by 0xb0urn3
- Version 0.1.3-ALPHA
- Built with Python and love for the Linux/Unix community

## Support

For bug reports and feature requests, please use the GitHub issue tracker. For general questions and discussions, join our community Discord server.
