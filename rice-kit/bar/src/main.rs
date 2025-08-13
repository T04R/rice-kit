
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::thread;
use std::time::Duration;
use get_if_addrs::{get_if_addrs, IfAddr};
use libc::{time, time_t, localtime};
use alsa::mixer::SelemChannelId;
use std::ptr;
use alsa::Mixer;
use alsa::mixer::SelemId;

struct ModuleConfig {
    network: bool,
    ip: bool,
    dns: bool,
    ram: bool,
    cpu: bool,
    volume: bool,
    brightness: bool,
    datetime: bool,
    usb: bool,
    battery: bool,
    mp: bool,
    uptime: bool,
}

impl Default for ModuleConfig {
    fn default() -> Self {
        ModuleConfig {
            network: true,
            ip: true,
            dns: true,
            ram: true,
            cpu: true,
            volume: true,
            brightness: true,
            datetime: true,
            usb: true,
            battery: true,
            mp: true,
            uptime: true,
        }
    }
}


fn main() {
    println!("{{\"version\":1}}");
    println!("[");
    println!("[]");

    let config = ModuleConfig::default();
    let mut stdout = io::stdout();
    let mut blink = false;
    let mut prev_cpu = read_cpu();
    let mut prev_net = read_network_stats("wlan0");

    let mut counter: u8 = 0;
    let mut cached_battery_block = String::new();
    let mut cached_ip_blocks = (String::new(), String::new());
    let mut cached_dns_block = String::new();
    let mut cached_usb_block = String::new();
    let mut cached_time_block = String::new();
    let mut cached_uptime_block = String::new();
    let mut cached_brightness_block = String::new();
    let mut cached_volume_block = String::new();
    let mut cached_mp_block = String::new();

    let mut cached_ram_block = String::new();
    let mut cached_cpu_block = String::new();

    loop {
        blink = !blink;
        let mut blocks = Vec::new();

        if counter % 4 == 0 { // Every 2 seconds (4*500ms)
            if config.battery {
                cached_battery_block = get_battery(blink);
            }
            if config.ip {
                cached_ip_blocks = get_ip_addresses("wlan0");
            }
            if config.dns {
                cached_dns_block = get_dns();
            }
            if config.usb {
                cached_usb_block = get_usb();
            }
        }

        if counter % 2 == 0 { // Every 1 second (2*500ms)
            if config.datetime {
                cached_time_block = get_time();
            }
            if config.uptime {
                cached_uptime_block = get_uptime();
            }
            if config.brightness {
                cached_brightness_block = get_brightness();
            }
            if config.volume {
                cached_volume_block = get_volume();
            }
            if config.mp {
                cached_mp_block = get_mp_status();
            }

            if config.ram {
                cached_ram_block = get_ram(blink);
            }
            if config.cpu {
                cached_cpu_block = get_cpu_and_temp(&mut prev_cpu, blink);
            }
        }

        // Always updated modules (every 500ms)
        if config.network {
            blocks.push(get_network(&mut prev_net, "wlan0"));
        }

        if config.ip {
            blocks.push(cached_ip_blocks.0.clone());
            blocks.push(cached_ip_blocks.1.clone());
        }
        if config.dns {
            blocks.push(cached_dns_block.clone());
        }
        if config.ram {
            blocks.push(cached_ram_block.clone());
        }
        if config.cpu {
            blocks.push(cached_cpu_block.clone());
        }
        if config.volume {
            blocks.push(cached_volume_block.clone());
        }
        if config.brightness {
            blocks.push(cached_brightness_block.clone());
        }
        if config.datetime {
            blocks.push(cached_time_block.clone());
        }
        if config.uptime {
            blocks.push(cached_uptime_block.clone());
        }
        if config.battery {
            blocks.push(cached_battery_block.clone());
        }
        if config.mp {
            blocks.push(cached_mp_block.clone());
        }
        if config.usb {
            blocks.push(cached_usb_block.clone());
        }

        // Generate output
        let output = format!(",[\n{}]", blocks.join(",\n"));
        writeln!(stdout, "{}", output).unwrap();
        stdout.flush().unwrap();

        // Update counter for refresh rates
        counter = (counter + 1) % 4;
        thread::sleep(Duration::from_millis(500));
    }
}


fn make_block(text: &str, color: &str, border: Option<&str>, separator: bool) -> String {
    format!(
        r#"{{"full_text":"{}", "color":"{}", "background":"\u0023000000", "border_width":1{}, "separator":{}}}"#,
        text,
        color,
        border.map_or(String::new(), |b| format!(r#", "border":"{}""#, b)),
        separator
    )
}

// Network
fn get_network(prev: &mut (u64, u64), interface: &str) -> String {
    let curr = read_network_stats(interface);
    let (prev_rx, prev_tx) = *prev;
    let (curr_rx, curr_tx) = curr;
    *prev = curr;

    let rx_kb = (curr_rx - prev_rx) as f64 / 1024.0;
    let tx_kb = (curr_tx - prev_tx) as f64 / 1024.0;

    let format_speed = |speed: f64| {
        if speed >= 1000.0 {
            format!("{:.1}M", speed / 1024.0)
        } else {
            format!("{:.1}K", speed)
        }
    };

    let down_block = make_block(&format!("↓{}", format_speed(rx_kb)), "#0A79FF", None, false);
    let up_block = make_block(&format!("↑{}", format_speed(tx_kb)), "#FF0000", None, true);
    
    format!("{},\n{}", down_block, up_block)
}

// IP
fn get_ip_addresses(interface: &str) -> (String, String) {
    let (mut ipv4, mut ipv6) = ("ipv4".to_string(), "ipv6".to_string());
    let mut ipv4_color = "#FF0000";
    let mut ipv6_color = "#FF0000";

    if let Ok(ifaces) = get_if_addrs() {
        for iface in ifaces {
            if iface.name == interface {
                match iface.addr {
                    IfAddr::V4(addr) if !addr.ip.is_loopback() => {
                        ipv4 = addr.ip.to_string();
                        ipv4_color = "#00FF00";
                    }
                    IfAddr::V6(addr) if !addr.ip.is_loopback() => {
                        ipv6 = addr.ip.to_string();
                        ipv6_color = "#00FF00";
                    }
                    _ => (),
                }
            }
        }
    }

    let ipv4_block = make_block(&ipv4, ipv4_color, None, false);
    let ipv6_block = make_block(&ipv6, ipv6_color, None, true);
    
    (ipv4_block, ipv6_block)
}

// DNS
fn get_dns() -> String {
    let (text, color) = match fs::read_to_string("/etc/resolv.conf") {
        Ok(content) => {
            let servers: Vec<_> = content.lines()
                .filter(|l| l.starts_with("nameserver"))
                .filter_map(|l| l.split_whitespace().nth(1))
                .collect();

            if !servers.is_empty() {
                (servers[0].to_string(), "#0075B6")
            } else {
                ("dns".to_string(), "#FF0000")
            }
        }
        Err(_) => ("error".to_string(), "#FF0000"),
    };
    make_block(&text, color, None, true)
}


// RAM
fn get_ram(blink: bool) -> String {
    let meminfo = fs::read_to_string("/proc/meminfo").unwrap_or_default();
    let (total, available) = meminfo.lines().fold((0, 0), |(mut t, mut a), line| {
        if line.starts_with("MemTotal:") { t = parse_kb(line); }
        if line.starts_with("MemAvailable:") { a = parse_kb(line); }
        (t, a)
    });

    if total == 0 || available == 0 {
        make_block("RAM: N/A", "#FF0000", Some("#FF0000"), false)
    } else {
        let used = (total - available) as f64 / 1_048_576.0;
        let total_gb = total as f64 / 1_048_576.0;
        let text = format!("{:.1}/{:.1}G", used, total_gb);
        let usage_ratio = used / total_gb;

        if usage_ratio > 0.9 {
            let border_color = if blink { "#FF0000" } else { "#000000" };
            make_block(&text, "#FF0000", Some(border_color), false)
        } else {
            make_block(&text, "#FFA500", Some("#000000"), false)
        }
    }
}

//CPU
fn get_cpu_and_temp(prev: &mut (u64, u64), blink: bool) -> String {
    let curr = read_cpu();
    let total = curr.0 - prev.0;
    let idle = curr.1 - prev.1;
    *prev = curr;

    let usage = if total > 0 {
        100.0 * (1.0 - idle as f64 / total as f64)
    } else {
        0.0
    };

    let temp = read_cpu_temp().unwrap_or(0.0);
    let text = format!("{:.1}% {:.1}°C", usage, temp);

    if temp >= 70.0 {
        let border_color = if blink { "#FF0000" } else { "#000000" };
        make_block(&text, "#FF0000", Some(border_color), true)
    } else if usage >= 80.0 {
        make_block(&text, "#FF0000", Some("#000000"), true)
    } else {
        make_block(&text, "#FFFFFF", Some("#000000"), true)
    }
}


fn get_volume() -> String {
    let (text, color) = match Mixer::new("default", false) {
        Ok(mixer) => {
            let sid = SelemId::new("Master", 0);
            if let Some(selem) = mixer.find_selem(&sid) {
                let muted = selem.get_playback_switch(SelemChannelId::FrontLeft)
                    .map(|switch| switch == 0)
                    .unwrap_or(false);

                if muted {
                    ("MUTE".to_string(), "#FF0000")
                } else {
                    let (min, max) = selem.get_playback_volume_range();
                    match selem.get_playback_volume(SelemChannelId::FrontLeft) {
                        Ok(vol) => {
                            let percent = ((vol - min) as f64 / (max - min) as f64) * 100.0;
                            let value = percent.round() as i32;
                            let color = match value {
                                0 => "#FF0000",
                                _ if value > 80 => "#FFFF00",
                                _ => "#00FF00",
                            };
                            (format!("V{}%", value), color)
                        }
                        Err(_) => ("V?%".to_string(), "#FF0000"),
                    }
                }
            } else {
                ("V?%".to_string(), "#FF0000")
            }
        }
        Err(_) => ("V?%".to_string(), "#FF0000"),
    };
    make_block(&text, color, None, false)
}



// Brightness
fn get_brightness() -> String {
    let backlight_dir = if Path::new("/sys/class/backlight/intel_backlight").exists() {
        "/sys/class/backlight/intel_backlight"
    } else {
        "/sys/class/backlight/acpi_video0"
    };

    let (brightness, max) = (
        fs::read_to_string(format!("{}/brightness", backlight_dir)).unwrap_or_else(|_| "1".into()),
        fs::read_to_string(format!("{}/max_brightness", backlight_dir)).unwrap_or_else(|_| "100".into()),
    );

    let percent = brightness.trim().parse::<f32>().unwrap_or(1.0) /
                 max.trim().parse::<f32>().unwrap_or(100.0) * 100.0;

    let color = match percent {
        p if p < 5.0 => "#FF0000",
        p if p > 80.0 => "#FFFF00",
        _ => "#00FF00",
    };

    make_block(&format!("B{:.0}%", percent), color, None, true)
}

// DateTime
fn get_time() -> String {
    unsafe {
        let mut t: time_t = time(ptr::null_mut());
        let tm = *localtime(&mut t);

        let (h, meridian) = match tm.tm_hour {
            0 => (12, "AM"),
            1..=11 => (tm.tm_hour, "AM"),
            12 => (12, "PM"),
            _ => (tm.tm_hour - 12, "PM"),
        };

        let text = format!(
            "{}/{}/{} {}:{:02}:{:02} {}",
            tm.tm_year + 1900,
            tm.tm_mon + 1,
            tm.tm_mday,
            h,
            tm.tm_min,
            tm.tm_sec,
            meridian
        );
        make_block(&text, "#FFFFFF", None, true)
    }
}

// Uptime
fn get_uptime() -> String {
    let content = fs::read_to_string("/proc/uptime").unwrap_or_else(|_| "0 0".into());
    let uptime_seconds: u64 = content.split_whitespace()
        .next()
        .unwrap_or("0")
        .parse::<f64>()
        .unwrap_or(0.0) as u64;

    let (hours, minutes, seconds) = (
        uptime_seconds / 3600,
        (uptime_seconds % 3600) / 60,
        uptime_seconds % 60
    );

    let text = format!("{:02}:{:02}:{:02}", hours, minutes, seconds);
    let color = if hours > 24 { "#FF0000" } else { "#C0C0C0" };
    make_block(&text, color, None, false)
}

// Battery
fn get_battery(blink: bool) -> String {
    let (cap, status) = (
        fs::read_to_string("/sys/class/power_supply/BAT1/capacity").unwrap_or("0".into()),
        fs::read_to_string("/sys/class/power_supply/ACAD/online").unwrap_or("0".into()),
    );

    let cap_val = cap.trim().parse::<u8>().unwrap_or(0);
    let plugged = status.trim() == "1";
    let state = if plugged { "ch" } else { "dis" };
    let text = format!("{}%{}", cap_val, state);

    if !plugged && cap_val < 30 {
        let border = if blink { "#FF0000" } else { "#000000" };
        make_block(&text, "#FF0000", Some(border), false)
    } else {
        make_block(&text, "#C0C0C0", Some("#000000"), false)
    }
}

// Media recorder
fn get_mp_status() -> String {
    let (text, color) = match fs::read_to_string("/tmp/ffmpeg-state") {
        Ok(content) => {
            if content.trim() == "1" {
                ("● REC".to_string(), "#FF0000")
            } else {
                ("".to_string(), "#FFFFFF")
            }
        }
        Err(_) => ("".to_string(), "#FFFFFF"),
    };
    make_block(&text, color, None, false)
}

// USB
fn get_usb() -> String {
    let text = if Path::new("/dev/sdb").exists() || Path::new("/dev/sdb2").exists() {
        "▮"
    } else {
        ""
    };
    make_block(text, "#FF0000", None, false)
}

// ===== Utility Functions =====

fn parse_kb(s: &str) -> u64 {
    s.split_whitespace().nth(1).unwrap_or("0").parse().unwrap_or(0)
}

fn read_cpu() -> (u64, u64) {
    let content = fs::read_to_string("/proc/stat").unwrap_or_default();
    let line = content.lines().next().unwrap_or("");
    let values: Vec<u64> = line.split_whitespace().skip(1).filter_map(|s| s.parse().ok()).collect();
    if values.len() >= 5 {
        (values.iter().sum(), values[3] + values[4])
    } else {
        (0, 0)
    }
}

fn read_cpu_temp() -> Option<f32> {
    let paths = [
        "/sys/class/thermal/thermal_zone0/temp",
        "/sys/class/hwmon/hwmon0/temp1_input",
        "/sys/class/hwmon/hwmon1/temp1_input",
    ];
    for path in paths {
        if let Ok(content) = fs::read_to_string(path) {
            if let Ok(temp) = content.trim().parse::<f32>() {
                return Some(temp / 1000.0);
            }
        }
    }
    None
}

fn read_network_stats(interface: &str) -> (u64, u64) {
    let content = fs::read_to_string("/proc/net/dev").unwrap_or_default();
    for line in content.lines() {
        if line.contains(interface) {
            let parts: Vec<u64> = line.split_whitespace()
                .skip(1)
                .filter_map(|s| s.parse().ok())
                .collect();
            if parts.len() >= 16 {
                return (parts[0], parts[8]);
            }
        }
    }
    (0, 0)
}

