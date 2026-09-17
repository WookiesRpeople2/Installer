use std::sync::LazyLock;

pub const PROMPT_CARD_FOOTER: &str = "Enter confirm · Esc quit";
pub const SCROLL_LIST_CARD_FOOTER: &str = "↑↓ scroll · Enter select";
pub const GAUGE_CARD_FOOTER: &str = "Please wait…";
pub const PASSWORD_MASK: bool = true;

pub const BMGR_C_NAME: &str = "Boot Manager";
pub const BMGR_F_H: u16 = 10;
pub const BMGR_F_C: usize = 1;
pub const BMGR_T_INTRO: &str = "Select a boot magaer";
pub const BMGR_T_FOOTER: &str = "This will be used to boot the system";

pub const CONF_C_NAME: &str = "Confirm";
pub const CONF_F_H: u16 = 12;
pub const CONF_F_C: usize = 1;
pub const CONF_T_INTRO: &str = "Review your choices";
pub const CONF_T_USER: &str = "User:";
pub const CONF_T_HOST: &str = "Host:";
pub const CONF_T_BOOT: &str = "Boot:";
pub const CONF_T_DISK: &str = "Disk:";
pub const CONF_T_SWAP: &str = "Swap:";
pub const CONF_T_TZ: &str = "Timezone:";
pub const CONF_T_LOCALE: &str = "Locale:";
pub const CONF_T_KEY: &str = "Keymap:";
pub const CONF_T_FOOTER: &str = "Enter to install · Shift-Tab back";

pub const DISK_C_NAME: &str = "Disk";
pub const DISK_F_H: u16 = 12;
pub const DISK_F_C: usize = 1;
pub const DISK_T_INTRO: &str = "Select install target";
pub const DISK_T_FOOTER: &str = "WARNING: All data on this disk will be destroyed";

pub const DONE_C_NAME: &str = "Done";
pub const DONE_F_H: u16 = 4;
pub const DONE_F_C: usize = 0;
pub const DONE_T_INTRO: &str = "Install complete";
pub const DONE_T_FOOTER: &str = "Press Enter to reboot";

pub const HNAME_C_NAME: &str = "Hostname";
pub const HNAME_F_H: u16 = 4;
pub const HNAME_F_C: usize = 1;
pub const HNAME_T_INTRO: &str = "Name this computer";
pub const HNAME_T_FOOTER: &str = "Letters, numbers, hyphens only";

pub const INST_C_NAME: &str = "Installing";
pub const INST_F_H: u16 = 3;
pub const INST_F_C: usize = 1;
pub const INST_T_INTRO: &str = "Installing Modular Linux";

pub const KEY_C_NAME: &str = "Keyboard";
pub const KEY_F_H: u16 = 10;
pub const KEY_F_C: usize = 1;
pub const KEY_T_INTRO: &str = "Select keyboard layout";
pub const KEY_T_FOOTER: &str = "Written to /etc/vconsole.conf";

pub const LOCAL_C_NAME: &str = "Locale";
pub const LOCAL_F_H: u16 = 12;
pub const LOCAL_F_C: usize = 1;
pub const LOCAL_T_INTRO: &str = "Select system language / locale";
pub const LOCAL_T_FOOTER: &str = "Used for dates, numbers, messages";

pub const PASS_C_NAME: &str = "Password";
pub const PASS_F_H: u16 = 4;
pub const PASS_F_C: usize = 2;
pub const PASS_T_INTRO: &str = "Set the user and root password";
pub const PASS_T_FOOTER: &str = "Hidden while typig · Enter next";

pub const SWAP_C_NAME: &str = "Swap Partition";
pub const SWAP_F_H: u16 = 12;
pub const SWAP_F_C: usize = 1;
pub const SWAP_T_INTRO: &str = "Select install target";
pub const SWAP_T_FOOTER: &str = "Select None if you do not want to use the swap partition";
pub const SWAP_ON_INSTALL: &str = "Create on main disk";
pub const SWAP_NONE: &str = "None";

pub const TZ_C_NAME: &str = "Timezone";
pub const TZ_F_H: u16 = 10;
pub const TZ_F_C: usize = 1;
pub const TZ_T_INTRO: &str = "Select your timezone";
pub const TZ_T_FOOTER: &str = "PgUp/PgDn jump faster";

pub const USER_C_NAME: &str = "USERNAME";
pub const USER_F_H: u16 = 4;
pub const USER_F_C: usize = 1;
pub const USER_T_INTRO: &str = "Please enter your username";
pub const USER_T_FOOTER: &str = "The usernam can have any alphanumeric and digtial charaters in it";

pub const WEL_C_NAME: &str = "Welcome";
pub const WEL_C_M: u16 = 10;
pub const WEL_C_L: u16 = 1;
pub const WEL_T_FOOTER: &str =
    "Use Esc in order to quit\n Use Shift-Tab to go back one\nMade by WookiesRpeople2";
pub const WEL_LOGO_PATH: &str = include_str!("../assets/logo.txt");

pub const WIFI_C_NAME: &str = "Wi-Fi";
pub const WIFI_F_H: u16 = 12;
pub const WIFI_F_C: usize = 1;
pub const WIFI_SKIP: &str = "Skip (Ethernet)";
pub const WIFI_T_INTRO_NET: &str = "Select a Wi-Fi network";
pub const WIFI_T_FOOTER_NET: &str = "Skip if you are on Ethernet";
pub const WIFI_T_INTRO_PASS: &str = "Enter password for";
pub const WIFI_T_FOOTER_PASS: &str = "Enter connect · Shift-Tab back to list";

pub const SMGR_P_U_KEY: &str = "Username";
pub const SMGR_P_U_VALUE: &str = "type your name…";
pub const SMGR_P_U_MAX: usize = 64;
pub const SMGR_P_H_KEY: &str = "Hostname";
pub const SMGR_P_H_VALUE: &str = "modular";
pub const SMGR_P_H_MAX: usize = 64;
pub const SMGR_P_UP_KEY: &str = "User Password";
pub const SMGR_P_UP_VALUE: &str = "Set the user password";
pub const SMGR_P_UP_MAX: usize = 64;
pub const SMGR_P_RP_KEY: &str = "Root Password";
pub const SMGR_P_RP_VALUE: &str = "Set the root password";
pub const SMGR_P_RP_MAX: usize = 64;
pub const SMGR_P_WP_KEY: &str = "Wi-Fi Password";
pub const SMGR_P_WP_VALUE: &str = "network password…";
pub const SMGR_P_WP_MAX: usize = 128;

pub const SMGR_S_TZ_KEY: &str = "Timezone";
pub const SMGR_S_LOC_KEY: &str = "Locale";
pub const SMGR_S_KM_KEY: &str = "Keyboard";
pub const SMGR_S_DISK_KEY: &str = "Disks";
pub const SMGR_S_SWAP_KEY: &str = "Swaps";
pub const SMGR_S_BOOT_KEY: &str = "Boot Managers";
pub const SMGR_S_WIFI_KEY: &str = "WI-FI";
pub static SMGR_S_TZ_OPTS: LazyLock<Vec<String>> = LazyLock::new(|| {
    vec![
        "Europe/Paris".into(),
        "Europe/London".into(),
        "Europe/Berlin".into(),
        "America/New_York".into(),
        "America/Los_Angeles".into(),
        "Asia/Tokyo".into(),
        "UTC".into(),
    ]
});
pub static SMGR_S_LOC_OPTS: LazyLock<Vec<String>> = LazyLock::new(|| {
    vec![
        "en_US.UTF-8".into(),
        "en_GB.UTF-8".into(),
        "fr_FR.UTF-8".into(),
        "de_DE.UTF-8".into(),
        "es_ES.UTF-8".into(),
        "it_IT.UTF-8".into(),
        "pt_PT.UTF-8".into(),
        "nl_NL.UTF-8".into(),
        "pl_PL.UTF-8".into(),
        "sv_SE.UTF-8".into(),
        "ja_JP.UTF-8".into(),
        "zh_CN.UTF-8".into(),
    ]
});
pub static SMGR_S_KM_OPTS: LazyLock<Vec<String>> = LazyLock::new(|| {
    vec![
        "us".into(),
        "uk".into(),
        "fr".into(),
        "de".into(),
        "es".into(),
        "it".into(),
        "pt".into(),
        "nl".into(),
        "pl".into(),
        "sv".into(),
        "dvorak".into(),
        "colemak".into(),
    ]
});
pub static SMGR_S_BOOT_OPTS: LazyLock<Vec<String>> =
    LazyLock::new(|| vec!["efi".into(), "grub".into()]);

pub const INSTALL_RATIO: f64 = 0.0;
pub const INSTALL_LABEL: &str = "Starting…";
