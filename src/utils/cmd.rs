use std::{
    ffi::OsStr,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

pub type CmdResult = Result<String, String>;

pub fn get_disks() -> Vec<String> {
    match run("lsblk", &["-dn", "-o", "NAME,SIZE,TYPE,MODEL"]) {
        Ok(out) => out
            .lines()
            .filter_map(|line| {
                let mut parts = line.split_whitespace();
                let name = parts.next()?;
                let size = parts.next().unwrap_or("-");
                let typ = parts.next().unwrap_or("");
                if typ != "disk" {
                    return None;
                }
                let model = parts.collect::<Vec<_>>().join(" ");
                Some(format!("/dev/{name}  {size}  {model}"))
            })
            .collect(),
        _ => vec!["/dev/sda  128G  (example)".into()],
    }
}

pub fn get_swap_disks(exclude_disk: &Path) -> Vec<String> {
    get_disks()
        .into_iter()
        .filter(|line| {
            let name = line.split_whitespace().next().unwrap_or("");
            Path::new(name) != exclude_disk
        })
        .collect()
}

pub fn get_swap_candidates(exclude_disk: &Path) -> Vec<String> {
    let exclude = exclude_disk
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("");

    let mut live_names: Vec<String> = Vec::new();
    for mount in ["/", "/run/archiso/bootmnt", "/run/archiso/copytoram"] {
        if let Ok(out) = run("findmnt", &["-n", "-o", "SOURCE", mount]) {
            // SOURCE may be /dev/sda1 or /dev/mapper/...
            if let Some(name) = out.trim().strip_prefix("/dev/") {
                let base = name.trim_end_matches(|c: char| c.is_ascii_digit());
                let base = base.strip_suffix('p').unwrap_or(base);
                live_names.push(name.to_string());
                live_names.push(base.to_string());
            }
        }
    }

    let Ok(out) = run(
        "lsblk",
        &[
            "-dn",
            "-b",
            "-o",
            "NAME,SIZE,TYPE,RO,MOUNTPOINT,FSTYPE,PKNAME,MODEL",
        ],
    ) else {
        return Vec::new();
    };

    out.lines()
        .filter_map(|line| {
            let mut p = line.split_whitespace();
            let name = p.next()?;
            let size_b: u64 = p.next()?.parse().ok()?;
            let typ = p.next()?;
            let ro = p.next()?;
            let mount = p.next().unwrap_or("-");
            let _fstype = p.next().unwrap_or("-");
            let pkname = p.next().unwrap_or("-");
            let model = p.collect::<Vec<_>>().join(" ");

            let is_mounted = mount != "-" && !mount.is_empty();
            let on_install_disk = pkname == exclude || name.starts_with(exclude);
            let is_live_media = live_names
                .iter()
                .any(|d| name == d || pkname == d || name.starts_with(d.as_str()));
            let is_tiny = size_b < 256 * 1024 * 1024;

            if typ != "part"
                || ro != "0"
                || is_mounted
                || on_install_disk
                || is_live_media
                || is_tiny
            {
                return None;
            }

            let size = if size_b >= 1024 * 1024 * 1024 {
                format!("{:.1}G", size_b as f64 / (1024.0 * 1024.0 * 1024.0))
            } else if size_b >= 1024 * 1024 {
                format!("{:.0}M", size_b as f64 / (1024.0 * 1024.0))
            } else {
                format!("{:.0}K", size_b as f64 / 1024.0)
            };

            Some(format!("/dev/{name}  {size}  {model}"))
        })
        .collect()
}

pub fn run<I, S>(program: &str, args: I) -> Result<String, String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let output = Command::new(program)
        .args(args)
        .output()
        .map_err(|e| format!("failed to start `{program}`: {e}"))?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        Err(format!("`{program}` failed: {stderr}"))
    }
}

pub fn run_with_stdin<I, S>(program: &str, args: I, stdin: &str) -> CmdResult
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let args: Vec<String> = args
        .into_iter()
        .map(|arg| arg.as_ref().to_string_lossy().into_owned())
        .collect();

    let mut child = Command::new(program)
        .args(&args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("failed to start `{program}`: {e}"))?;
    if let Some(mut pipe) = child.stdin.take() {
        pipe.write_all(stdin.as_bytes())
            .map_err(|e| format!("failed writing stdin to `{program}`: {e}"))?;
    }
    let output = child
        .wait_with_output()
        .map_err(|e| format!("failed waiting for `{program}`: {e}"))?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        Err(format!("`{program} {}` failed: {stderr}", args.join(" ")))
    }
}

pub fn chroot<I, S>(args: I) -> CmdResult
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let args: Vec<String> = args
        .into_iter()
        .map(|arg| arg.as_ref().to_string_lossy().into_owned())
        .collect();
    let args: Vec<&str> = args.iter().map(String::as_str).collect();

    let mut full = vec!["/mnt"];
    full.extend_from_slice(&args);

    run("arch-chroot", &full)
}

pub fn chroot_with_stdin<I, S>(args: I, stdin: &str) -> CmdResult
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let args: Vec<String> = args
        .into_iter()
        .map(|arg| arg.as_ref().to_string_lossy().into_owned())
        .collect();
    let args: Vec<&str> = args.iter().map(String::as_str).collect();

    let mut full = vec!["/mnt"];
    full.extend_from_slice(&args);

    run_with_stdin("arch-chroot", &full, stdin)
}

pub fn reboot() -> CmdResult {
    run("shutdown", &["-r", "now"])
}

pub fn part_name(disk: &Path, n: u32) -> PathBuf {
    let s = disk.to_string_lossy();
    if s.contains("nvme") || s.contains("mmcblk") {
        PathBuf::from(format!("{s}p{n}"))
    } else {
        PathBuf::from(format!("{s}{n}"))
    }
}
