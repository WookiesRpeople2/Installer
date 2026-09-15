use crate::steps::step_manager::State;
use crate::utils::cmd::{CmdResult, chroot, chroot_with_stdin, part_name, run};
use std::fs::{create_dir_all, write};
use std::path::{Path, PathBuf};

pub enum InstallEvent {
    Progress { ratio: f64, label: String },
    Done,
    Failed(String),
}

struct Partitions {
    efi: PathBuf,
    root: PathBuf,
}

pub fn install(state: &State, mut on_progress: impl FnMut(f64, &str)) -> CmdResult {
    let disk = state.disk.as_path();
    let boot_manager = state.boot_managers.selected();

    on_progress(0.05, "Partitioning");
    let parts = partition(disk)?;

    on_progress(0.15, "Formatting");
    format_partitions(&parts)?;

    on_progress(0.25, "Mounting");
    mount_boot(&parts)?;
    if state.swap.as_os_str().len() > 0 {
        setup_swap(state.swap.as_path())?;
    }
    gen_fstab()?;

    on_progress(0.40, "Installing packages");
    pacstrap(boot_manager == "grub".into())?;

    on_progress(0.70, "Configuring system");
    hostname(state.hostname.value())?;
    timezone(state.timezone.selected().unwrap_or("UTC"))?;
    locale(state.locale.selected().unwrap_or("en_US.UTF-8"))?;
    keymap(state.keymap.selected().unwrap_or("us"))?;
    user(state.username.value(), state.user_password.value())?;
    root(state.root_password.value())?;
    sudoers()?;

    on_progress(0.90, "Bootloader");
    bootloader(
        state.boot_managers.selected().unwrap_or("efi"),
        disk,
        &parts,
    )?;
    enable_services()?;

    on_progress(1.0, "Done");
    unmount()?;
    Ok("install complete".into())
}

fn gen_fstab() -> Result<(), String> {
    let fstab = run("genfstab", ["-U", "/mnt"])?;
    write("/mnt/etc/fstab", format!("{fstab}\n")).map_err(|e| e.to_string())
}

fn sudoers() -> Result<(), String> {
    write("/mnt/etc/sudoers.d/wheel", "%wheel ALL=(ALL:ALL) ALL\n").map_err(|e| e.to_string())
}

fn user(user: &str, user_password: &str) -> CmdResult {
    let _ = chroot(&["useradd", "-m", "-G", "wheel", "-s", "/bin/bash", user]);
    chroot_with_stdin(&["chpasswd"], &format!("{}:{}", user, user_password))
}

fn root(root_password: &str) -> CmdResult {
    chroot_with_stdin(&["chpasswd"], &format!("root:{root_password}\n"))
}

fn hostname(hostname: &str) -> Result<(), String> {
    write("/mnt/etc/hostname", format!("{}\n", hostname)).map_err(|e| e.to_string())?;
    Ok(())
}

fn format_partitions(parts: &Partitions) -> CmdResult {
    run("mkfs.fat", ["-F32", parts.efi.to_str().unwrap()])?;
    run("mkfs.ext4", ["-F", parts.root.to_str().unwrap()])?;
    Ok(String::new())
}

fn partition(disk: &Path) -> Result<Partitions, String> {
    let d = disk.to_string_lossy();

    run("sgdisk", ["--zap-all", d.as_ref()])?;
    run("sgdisk", ["-n", "1:0:+512M", "-t", "1:EF00", d.as_ref()])?;
    run("sgdisk", ["-n", "2:0:0", "-t", "2:8300", d.as_ref()])?;
    run("partprobe", [d.as_ref()])?;
    Ok(Partitions {
        efi: part_name(disk, 1),
        root: part_name(disk, 2),
    })
}

fn mount_boot(parts: &Partitions) -> CmdResult {
    run("mount", [parts.root.to_str().unwrap(), "/mnt"])?;
    create_dir_all("/mnt/boot").map_err(|e| e.to_string())?;
    run("mount", [parts.efi.to_str().unwrap(), "/mnt/boot"])?;
    Ok(String::new())
}

fn setup_swap(swap: &Path) -> CmdResult {
    run("mkswap", [swap.to_str().unwrap()])?;
    run("swapon", [swap.to_str().unwrap()])?;
    Ok(String::new())
}

fn pacstrap(grub: bool) -> CmdResult {
    run(
        "pacstrap",
        [
            "-K",
            "/mnt",
            "base",
            "linux",
            "linux-firmware",
            "sudo",
            "efibootmgr",
            if grub { "grub" } else { "" },
            "modular-meta",
        ],
    )
}

fn timezone(tz: &str) -> CmdResult {
    chroot([
        "ln",
        "-sf",
        &format!("/usr/share/zoneinfo/{tz}"),
        "/etc/localtime",
    ])?;
    chroot(["hwclock", "--systohc"])
}

fn locale(locale: &str) -> Result<(), String> {
    write("/mnt/etc/locale.gen", format!("{locale} UTF-8\n")).map_err(|e| e.to_string())?;
    chroot(["locale-gen"])?;
    write("/mnt/etc/locale.conf", format!("LANG={locale}\n")).map_err(|e| e.to_string())
}

fn keymap(keymap: &str) -> Result<(), String> {
    write("/mnt/etc/vconsole.conf", format!("KEYMAP={keymap}\n")).map_err(|e| e.to_string())
}

fn bootloader(kind: &str, disk: &Path, parts: &Partitions) -> CmdResult {
    match kind {
        "grub" => {
            chroot([
                "grub-install",
                "--target=x86_64-efi",
                "--efi-directory=/boot",
                "--bootloader-id=Modular",
            ])?;
            chroot(["grub-mkconfig", "-o", "/boot/grub/grub.cfg"])
        }
        _ => {
            chroot(["bootctl", "install"])?;
            let _ = (disk, parts);
            Ok(String::new())
        }
    }
}

fn enable_services() -> CmdResult {
    chroot(["systemctl", "enable", "NetworkManager"])?;
    chroot(["systemctl", "enable", "sddm"])?;
    Ok(String::new())
}
fn unmount() -> CmdResult {
    run("umount", ["-R", "/mnt"])
}
