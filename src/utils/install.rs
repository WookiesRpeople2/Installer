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
    swap: Option<PathBuf>,
}

pub fn install(state: &State, mut on_progress: impl FnMut(f64, &str)) -> CmdResult {
    let disk = state.disk.as_path();
    let boot_manager = state.boot_managers.selected();

    on_progress(0.05, "Partitioning");
    let parts = partition(disk, state)?;

    on_progress(0.15, "Formatting");
    format_partitions(&parts)?;

    on_progress(0.25, "Mounting");
    mount_boot(&parts)?;
    let swap_other = state
        .swap_create_other
        .then(|| partition_swap_disk(&state.swap_disk, state.swap_size_gb))
        .transpose()?;
    let swap_dev: Option<&Path> = if state.swap_on_install {
        parts.swap.as_deref()
    } else {
        swap_other
            .as_deref()
            .or_else(|| (!state.swap.as_os_str().is_empty()).then(|| state.swap.as_path()))
    };
    if let Some(swap) = swap_dev {
        setup_swap(swap)?;
    }

    on_progress(0.40, "Installing packages");
    pacstrap(boot_manager == "grub".into())?;
    gen_fstab()?;

    on_progress(0.70, "Configuring system");
    hostname(state.hostname.value())?;
    timezone(state.timezone.selected().unwrap_or("UTC"))?;
    locale(state.locale.selected().unwrap_or("en_US.UTF-8"))?;
    keymap(state.keymap.selected().unwrap_or("us"))?;
    user(state.username.value(), state.user_password.value())?;
    root(state.root_password.value())?;
    sudoers()?;

    on_progress(0.90, "Bootloader");
    bootloader(state.boot_managers.selected().unwrap_or("efi"), &parts)?;
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
    let _ = chroot(&["useradd", "-m", "-G", "wheel", "-s", "/bin/bash", user])?;
    chroot_with_stdin(&["chpasswd"], &format!("{}:{}\n", user, user_password))
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
    run("udevadm", ["settle"])?;
    Ok(String::new())
}

fn partition(disk: &Path, state: &State) -> Result<Partitions, String> {
    let disk_str = disk.to_string_lossy();
    let d = disk_str.as_ref();
    let with_swap = state.swap_on_install;

    let _ = run("swapoff", ["-a"]);

    run("wipefs", ["-a", d])?;
    run("sgdisk", ["--zap-all", d])?;
    run("partprobe", [d])?;
    run("udevadm", ["settle"])?;
    run("sgdisk", ["-n1:0:+512M", "-t1:EF00", d])?;

    let root_num: u32 = if with_swap {
        let swap_size = format!("+{}G", state.swap_size_gb);
        let n = format!("-n2:0:{swap_size}");
        run("sgdisk", [n.as_str(), "-t2:8200", d])?;
        run("sgdisk", ["-n3:0:0", "-t3:8300", d])?;
        3
    } else {
        run("sgdisk", ["-n2:0:0", "-t2:8300", d])?;
        2
    };

    run("partprobe", [d])?;
    run("udevadm", ["settle"])?;

    let parts = Partitions {
        efi: part_name(disk, 1),
        swap: with_swap.then(|| part_name(disk, 2)),
        root: part_name(disk, root_num),
    };

    for p in std::iter::once(&parts.efi)
        .chain(parts.swap.iter())
        .chain(std::iter::once(&parts.root))
    {
        wait_for_part(p)?;
        run("wipefs", ["-a", p.to_str().unwrap()])?;
    }

    Ok(parts)
}

fn partition_swap_disk(disk: &Path, size_gb: u64) -> Result<PathBuf, String> {
    let disk_str = disk.to_string_lossy();
    let d = disk_str.as_ref();

    let _ = run("swapoff", ["-a"]);
    run("wipefs", ["-a", d])?;
    run("sgdisk", ["--zap-all", d])?;
    run("partprobe", [d])?;
    run("udevadm", ["settle"])?;

    let size = format!("+{}G", size_gb);
    let n = format!("-n1:0:{size}");
    run("sgdisk", [n.as_str(), "-t1:8200", d])?;

    run("partprobe", [d])?;
    run("udevadm", ["settle"])?;

    let part = part_name(disk, 1);
    wait_for_part(&part)?;
    run("wipefs", ["-a", part.to_str().unwrap()])?;
    Ok(part)
}

fn mount_boot(parts: &Partitions) -> CmdResult {
    run(
        "mount",
        ["-t", "ext4", parts.root.to_str().unwrap(), "/mnt"],
    )?;
    create_dir_all("/mnt/boot").map_err(|e| e.to_string())?;
    run(
        "mount",
        ["-t", "vfat", parts.efi.to_str().unwrap(), "/mnt/boot"],
    )?;
    Ok(String::new())
}

fn setup_swap(swap: &Path) -> CmdResult {
    run("mkswap", [swap.to_str().unwrap()])?;
    run("swapon", [swap.to_str().unwrap()])?;
    Ok(String::new())
}

fn pacstrap(grub: bool) -> CmdResult {
    let mut args = vec![
        "-K",
        "/mnt",
        "base",
        "linux",
        "linux-firmware",
        "sudo",
        "efibootmgr",
        "modular-meta",
    ];
    if grub {
        args.push("grub");
    }
    run("pacstrap", args)
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

fn bootloader(kind: &str, parts: &Partitions) -> CmdResult {
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
            let partuuid = run(
                "blkid",
                [
                    "-s",
                    "PARTUUID",
                    "-o",
                    "value",
                    parts.root.to_str().unwrap(),
                ],
            )?;
            write(
                "/mnt/boot/loader/loader.conf",
                "default modular.conf\ntimeout 3\nconsole-mode max\neditor no\n",
            )
            .map_err(|e| e.to_string())?;
            create_dir_all("/mnt/boot/loader/entries").map_err(|e| e.to_string())?;
            write(
                "/mnt/boot/loader/entries/modular.conf",
                format!(
                    "title   Modular Linux\n\
                     linux   /vmlinuz-linux\n\
                     initrd  /initramfs-linux.img\n\
                     options root=PARTUUID={partuuid} rw\n"
                ),
            )
            .map_err(|e| e.to_string())?;
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
    let _ = run("swapoff", ["-a"]);
    run("umount", ["-R", "/mnt"])
}

fn wait_for_part(part: &Path) -> CmdResult {
    run("udevadm", ["wait", "--timeout=30", part.to_str().unwrap()])
}
