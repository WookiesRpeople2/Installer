use crate::utils::cmd::{CmdResult, run};

pub fn wifi_device() -> Result<String, String> {
    let out = run("iwctl", &["device", "list"])?;
    for line in out.lines() {
        let line = strip_ansi(line).trim().to_string();
        if line.is_empty()
            || line.contains("Devices")
            || line.contains("---")
            || line.contains("Address")
        {
            continue;
        }
        let mut parts = line.split_whitespace();
        let name = parts.next().unwrap_or("");
        let mode = parts.last().unwrap_or("");
        if !name.is_empty() && mode == "station" {
            return Ok(name.to_string());
        }
    }
    Err("no Wi-Fi device found".into())
}

pub fn wifi_scan(device: &str) -> Result<(), String> {
    run("iwctl", &["station", device, "scan"])?;
    std::thread::sleep(std::time::Duration::from_secs(2));
    Ok(())
}

pub fn wifi_networks(device: &str) -> Vec<String> {
    let Ok(out) = run("iwctl", &["station", device, "get-networks"]) else {
        return Vec::new();
    };
    let mut nets = Vec::new();
    for line in out.lines() {
        let line = strip_ansi(line);
        let line = line.trim();
        if line.is_empty()
            || line.contains("Available networks")
            || line.contains("Network name")
            || line.contains("---")
        {
            continue;
        }
        let mut parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 {
            continue;
        }
        if parts.len() >= 3 {
            parts.pop();
            parts.pop();
        }
        let ssid = parts.join(" ");
        if !ssid.is_empty() {
            nets.push(ssid);
        }
    }
    nets.sort();
    nets.dedup();
    nets
}

pub fn wifi_connect(device: &str, ssid: &str, passphrase: &str) -> CmdResult {
    let ssid = ssid.trim().trim_start_matches('>').trim();

    let args: Vec<String> = if passphrase.is_empty() {
        vec![
            "--dont-ask".into(),
            "station".into(),
            device.into(),
            "connect".into(),
            ssid.into(),
        ]
    } else {
        vec![
            "--dont-ask".into(),
            format!("--passphrase={passphrase}"),
            "station".into(),
            device.into(),
            "connect".into(),
            ssid.into(),
        ]
    };

    // Prefer showing stdout+stderr when iwctl fails
    let output = std::process::Command::new("iwctl")
        .args(&args)
        .output()
        .map_err(|e| format!("failed to start iwctl: {e}"))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        Err(format!("iwctl connect failed: {} {}", stderr, stdout)
            .trim()
            .to_string())
    }
}

fn strip_ansi(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\u{1b}' {
            if chars.peek() == Some(&'[') {
                chars.next();
                while let Some(n) = chars.next() {
                    if n.is_ascii_alphabetic() {
                        break;
                    }
                }
            }
        } else {
            out.push(c);
        }
    }
    out
}
