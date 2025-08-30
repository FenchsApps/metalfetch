use colored::*;
use owo_colors::OwoColorize;
use std::process::Command;
use std::io::BufRead;
use std::fs;
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    #[serde(default = "default_font")]
    font: String,
    #[serde(default = "default_font_size")]
    font_size: u8,
    #[serde(default = "default_spacing")]
    spacing: u8,
    #[serde(default = "default_info_order")]
    info_order: Vec<String>,
    #[serde(default = "default_colors")]
    colors: Colors,
    #[serde(default = "default_logo")]
    show_logo: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Colors {
    #[serde(default = "default_label_color")]
    label: String,
    #[serde(default = "default_value_color")]
    value: String,
    #[serde(default = "default_logo_color")]
    logo: String,
}

#[derive(Debug)]
struct DistroInfo {
    name: String,
    architecture: String,
    kernel: String,
    shell: String,
    desktop: String,
    packages: String,
    uptime: String,
    wm: String,
    theme: String,
    icons: String,
    terminal: String,
}

fn default_font() -> String {
    "default".to_string()
}

fn default_font_size() -> u8 {
    12
}

fn default_spacing() -> u8 {
    10
}

fn default_info_order() -> Vec<String> {
    vec![
        "OS".to_string(),
        "Architecture".to_string(),
        "Kernel".to_string(),
        "Shell".to_string(),
        "Desktop".to_string(),
        "Packages".to_string(),
        "Uptime".to_string(),
        "WM".to_string(),
        "Theme".to_string(),
        "Icons".to_string(),
        "Terminal".to_string(),
    ]
}

fn default_colors() -> Colors {
    Colors {
        label: "blue".to_string(),
        value: "white".to_string(),
        logo: "cyan".to_string(),
    }
}

fn default_label_color() -> String {
    "blue".to_string()
}

fn default_value_color() -> String {
    "white".to_string()
}

fn default_logo_color() -> String {
    "cyan".to_string()
}

fn default_logo() -> bool {
    true
}

impl Config {
    fn load() -> Result<Self> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?
            .join("metalfetch");
        
        let config_file = config_dir.join("metalfetch.conf");
        
        if config_file.exists() {
            let content = fs::read_to_string(&config_file)?;
            let config: Config = serde_json::from_str(&content)?;
            Ok(config)
        } else {
            fs::create_dir_all(&config_dir)?;
            let default_config = Config::default();
            let config_json = serde_json::to_string_pretty(&default_config)?;
            fs::write(&config_file, config_json)?;
            println!("Created default config at: {:?}", config_file);
            Ok(default_config)
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            font: default_font(),
            font_size: default_font_size(),
            spacing: default_spacing(),
            info_order: default_info_order(),
            colors: default_colors(),
            show_logo: default_logo(),
        }
    }
}

impl DistroInfo {
    fn new() -> Result<Self> {
        let name = Self::get_distro_name()?;
        let architecture = Self::get_architecture()?;
        let kernel = Self::get_kernel()?;
        let shell = Self::get_shell()?;
        let desktop = Self::get_desktop()?;
        let packages = Self::get_packages()?;
        let uptime = Self::get_uptime()?;
        let wm = Self::get_wm()?;
        let theme = Self::get_theme()?;
        let icons = Self::get_icons()?;
        let terminal = Self::get_terminal()?;

        Ok(Self {
            name,
            architecture,
            kernel,
            shell,
            desktop,
            packages,
            uptime,
            wm,
            theme,
            icons,
            terminal,
        })
    }

    fn get_distro_name() -> Result<String> {
        let output = Command::new("cat")
            .arg("/etc/os-release")
            .output()?;
        
        let content = String::from_utf8(output.stdout)?;
        for line in content.lines() {
            if line.starts_with("PRETTY_NAME=") {
                let name = line.split('=').nth(1)
                    .unwrap_or("Unknown")
                    .trim_matches('"');
                return Ok(name.to_string());
            }
        }
        Ok("Unknown".to_string())
    }

    fn get_uptime() -> Result<String> {
        let output = Command::new("uptime").arg("-p").output()?;
        let uptime = String::from_utf8(output.stdout)?.trim().to_string();
        if uptime.is_empty() {
            Ok("Unknown".to_string())
        } else {
            Ok(uptime)
        }
    }

    fn get_wm() -> Result<String> {
        let wm = std::env::var("XDG_CURRENT_DESKTOP")
            .unwrap_or_else(|_| std::env::var("DESKTOP_SESSION")
                .unwrap_or_else(|_| "Unknown".to_string()));
        Ok(wm)
    }

    fn get_theme() -> Result<String> {
        let output = Command::new("gsettings")
            .args(&["get", "org.gnome.desktop.interface", "gtk-theme"])
            .output();
        
        if let Ok(output) = output {
            if let Ok(theme) = String::from_utf8(output.stdout) {
                let theme = theme.trim().trim_matches('\'');
                if !theme.is_empty() && theme != "Unknown" {
                    return Ok(theme.to_string());
                }
            }
        }
        Ok("Unknown".to_string())
    }

    fn get_icons() -> Result<String> {
        let output = Command::new("gsettings")
            .args(&["get", "org.gnome.desktop.interface", "icon-theme"])
            .output();
        
        if let Ok(output) = output {
            if let Ok(icons) = String::from_utf8(output.stdout) {
                let icons = icons.trim().trim_matches('\'');
                if !icons.is_empty() && icons != "Unknown" {
                    return Ok(icons.to_string());
                }
            }
        }
        Ok("Unknown".to_string())
    }

    fn get_terminal() -> Result<String> {
        let terminal = std::env::var("TERM").unwrap_or_else(|_| "Unknown".to_string());
        Ok(terminal)
    }

    fn get_architecture() -> Result<String> {
        let output = Command::new("uname").arg("-m").output()?;
        Ok(String::from_utf8(output.stdout)?.trim().to_string())
    }

    fn get_kernel() -> Result<String> {
        let output = Command::new("uname").arg("-r").output()?;
        Ok(String::from_utf8(output.stdout)?.trim().to_string())
    }

    fn get_shell() -> Result<String> {
        let shell = std::env::var("SHELL").unwrap_or_else(|_| "Unknown".to_string());
        let shell_name = shell.split('/').last().unwrap_or("Unknown");
        Ok(shell_name.to_string())
    }

    fn get_desktop() -> Result<String> {
        let desktop = std::env::var("XDG_CURRENT_DESKTOP")
            .unwrap_or_else(|_| std::env::var("DESKTOP_SESSION")
                .unwrap_or_else(|_| "Unknown".to_string()));
        Ok(desktop)
    }

    fn get_packages() -> Result<String> {
        let package_managers = [
            ("pacman", "-Qq"),
            ("apt", "list --installed"),
            ("dnf", "list installed"),
            ("yum", "list installed"),
            ("zypper", "se --installed-only"),
            ("emerge", "-Q"),
            ("xbps-query", "-l"),
        ];

        for (manager, arg) in package_managers.iter() {
            if Command::new("which").arg(manager).output().is_ok() {
                if let Ok(output) = Command::new(manager).args(arg.split_whitespace()).output() {
                    let count = output.stdout.lines().count();
                    return Ok(format!("{} ({})", count, manager));
                }
            }
        }
        Ok("Unknown".to_string())
    }
}

fn get_distro_logo() -> &'static str {
    r#"
                 .88888888:.
                88888888.88888.
              .8888888888888888.
              888888888888888888
              88' _`88'_  `88888
              88 88 88 88  88888
              88_88_::_88_:88888
              88:::,::,:::::8888
              88`:::::::::'`8888
             .88  `::::'    8:88.
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
"#
}

fn print_info(distro_info: &DistroInfo, config: &Config) {
    let logo = get_distro_logo();
    let logo_lines: Vec<&str> = logo.lines().collect();
    
    let mut info_lines = Vec::new();
    
    for field in &config.info_order {
        let (label, value) = match field.as_str() {
            "OS" => ("OS", &distro_info.name),
            "Architecture" => ("Architecture", &distro_info.architecture),
            "Kernel" => ("Kernel", &distro_info.kernel),
            "Shell" => ("Shell", &distro_info.shell),
            "Desktop" => ("Desktop", &distro_info.desktop),
            "Packages" => ("Packages", &distro_info.packages),
            "Uptime" => ("Uptime", &distro_info.uptime),
            "WM" => ("WM", &distro_info.wm),
            "Theme" => ("Theme", &distro_info.theme),
            "Icons" => ("Icons", &distro_info.icons),
            "Terminal" => ("Terminal", &distro_info.terminal),
            _ => continue,
        };
        
        let colored_label = match config.colors.label.as_str() {
            "blue" => label.bright_blue().bold(),
            "green" => label.bright_green().bold(),
            "red" => label.bright_red().bold(),
            "yellow" => label.bright_yellow().bold(),
            "magenta" => label.bright_magenta().bold(),
            "cyan" => label.bright_cyan().bold(),
            _ => label.bright_blue().bold(),
        };
        
        let colored_value = match config.colors.value.as_str() {
            "white" => value.bright_white().to_string(),
            "blue" => value.bright_blue().to_string(),
            "green" => value.bright_green().to_string(),
            "red" => value.bright_red().to_string(),
            "yellow" => value.bright_yellow().to_string(),
            "magenta" => value.bright_magenta().to_string(),
            "cyan" => value.bright_cyan().to_string(),
            _ => value.bright_white().to_string(),
        };
        
        info_lines.push(format!("{} {}", colored_label, colored_value));
    }

    let max_logo_height = if config.show_logo { logo_lines.len() } else { 0 };
    let max_info_height = info_lines.len();
    let max_height = std::cmp::max(max_logo_height, max_info_height);

    for i in 0..max_height {
        let logo_line = if config.show_logo && i < logo_lines.len() { 
            let colored_logo = match config.colors.logo.as_str() {
                "cyan" => logo_lines[i].bright_cyan().to_string(),
                "blue" => logo_lines[i].bright_blue().to_string(),
                "green" => logo_lines[i].bright_green().to_string(),
                "red" => logo_lines[i].bright_red().to_string(),
                "yellow" => logo_lines[i].bright_yellow().to_string(),
                "magenta" => logo_lines[i].bright_magenta().to_string(),
                "white" => logo_lines[i].bright_white().to_string(),
                _ => logo_lines[i].bright_cyan().to_string(),
            };
            colored_logo
        } else { "".to_string() };
        
        let info_line = if i < info_lines.len() { &info_lines[i] } else { "" };
        
        if !logo_line.is_empty() && !info_line.is_empty() {
            let spacing = config.spacing as usize;
            let logo_width = 60 + spacing;
            println!("{:<width$} {}", logo_line, info_line, width = logo_width);
        } else if !logo_line.is_empty() {
            println!("{}", logo_line);
        } else if !info_line.is_empty() {
            let spacing = config.spacing as usize;
            let logo_width = 60 + spacing;
            println!("{:<width$} {}", "", info_line, width = logo_width);
        }
    }
}

fn main() -> Result<()> {
    let config = Config::load()?;
    
    match DistroInfo::new() {
        Ok(info) => {
            print_info(&info, &config);
        }
        Err(e) => {
            eprintln!("{} Error getting distro info: {}", "❌".red(), e);
            std::process::exit(1);
        }
    }
    
    Ok(())
}
