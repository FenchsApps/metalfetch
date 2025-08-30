# 🚀 MetalFetch

A beautiful fastfetch alternative written in Rust that displays Linux distribution information with a beautiful ASCII-art Tux penguin.

## ✨ Features

- 🐧 Beautiful ASCII-art Tux penguin on the left
- 📊 Detailed system information on the right
- 🌈 Fully customizable colors and styles
- ⚙️ Configuration file for personalization
- 🔄 Customizable information display order
- 🎨 Support for various color schemes
- ⚡ Fast compilation and execution in Rust
- 🔍 Automatic distribution detection

## 📋 Displayed Information

- **OS**: Operating system name
- **Architecture**: System architecture
- **Kernel**: Linux kernel version
- **Shell**: Used shell
- **Desktop**: Desktop environment
- **Packages**: Number of installed packages
- **Uptime**: System uptime
- **WM**: Window manager
- **Theme**: Desktop theme
- **Icons**: Icon set
- **Terminal**: Terminal type

## 🛠️ Installation

### Requirements

- Rust (version 1.70 or higher)
- Cargo

### Compilation

```bash
# Clone the repository
git clone <your-repo-url>
cd metalfetch

# Compile in release mode
cargo build --release

# Run
./target/release/metalfetch
```

### System Installation

```bash
# Compile
cargo build --release

# Copy to /usr/local/bin (requires sudo)
sudo cp target/release/metalfetch /usr/local/bin/

# Now you can run from any directory
metalfetch
```

## 🎯 Usage

Simply run the command:

```bash
metalfetch
```

## ⚙️ Configuration

MetalFetch automatically creates a configuration file on first run at `~/.config/metalfetch/metalfetch.conf`.

### Configuration Settings:

- **font**: Font for display (currently only "default" is supported)
- **font_size**: Font size in points (default: 12)
- **spacing**: Distance between logo and information (default: 10)
- **info_order**: Order of information display
- **colors**: Color scheme
  - **label**: Color of labels (OS, Kernel, Shell, etc.)
  - **value**: Color of values
  - **logo**: Color of Tux penguin
- **show_logo**: Whether to show the Tux penguin

### Supported Colors:
- `blue`, `green`, `red`, `yellow`, `magenta`, `cyan`, `white`

### Configuration Example:
```json
{
  "font": "default",
  "font_size": 14,
  "spacing": 15,
  "info_order": [
    "OS",
    "Kernel",
    "Shell",
    "Desktop",
    "Uptime"
  ],
  "colors": {
    "label": "green",
    "value": "yellow",
    "logo": "magenta"
  },
  "show_logo": true
}
```

## 🎨 Interface Features

- **🐧 Tux Penguin** - beautiful ASCII-art Linux mascot on the left
- **📊 System Information** - detailed information on the right
- **🌈 Colorful Output** - beautiful interface with customizable colors
- **🔍 Universality** - works on any Linux distribution

## 🔧 Dependencies

- `colored` - for color output
- `owo-colors` - additional color capabilities
- `sysinfo` - for system information
- `anyhow` - for error handling
- `serde` - for configuration serialization/deserialization
- `serde_json` - for JSON configuration handling
- `dirs` - for standard directory detection
- `clap` - for CLI interface (ready for expansion)

## 🚀 Development

```bash
# Run in development mode
cargo run

# Run tests
cargo test

# Code check
cargo clippy
```

## 📝 License

MIT License

## 🤝 Contributing

Pull requests and suggestions for improvements are welcome!

## 📸 Screenshot

```
                                                              OS Arch Linux
                 .88888888:.                                  Architecture x86_64
                88888888.88888.                               Kernel 6.16.3-arch1-1
              .8888888888888888.                              Shell fish
              888888888888888888                              Desktop Hyprland
              88' _`88'_  `88888                              Packages 816 (pacman)
              88 88 88 88  88888                              Uptime up 4 hours, 43 minutes
              88_88_::_88_:88888                              WM Hyprland
              88:::,::,:::::8888                              Theme Graphite
              88`:::::::::'`8888                              Icons Adwaita
             .88  `::::'    8:88.                             Terminal xterm-256color
            8888            `8:888.
          .8888'             `888888.
         .8888:..  .::.  ...:'8888888:.
        .8888.'     :'     `'::`88:88888
       .8888        '         `.888:8888.
      888:8         .           888:88888
    .888:88        .:           888:88888:
    8888888.       ::           88:888888
    `.::.888.      ::          .88888888
   .::::::.888.    ::         :::`8888'.:.
  ::::::::::.888   '         .::::::::::::
  ::::::::::::.8    '      .:8::::::::::::.
 .::::::::::::::.        .:888:::::::::::::
 :::::::::::::::88:.__..:88888:::::::::::'
  `'.:::::::::::88888888888.88:::::::::'
        `':::_:' -- '' -'-' `':_::::'`
```

---

**MetalFetch** - A beautiful and fast way to learn about your Linux distribution! 🐧✨
