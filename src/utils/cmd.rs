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
