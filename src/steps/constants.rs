pub const PROMPT_CARD_FOOTER: &str = "Enter confirm · Esc quit";
pub const SCROLL_LIST_CARD_FOOTER: &str = "↑↓ scroll · Enter select";
pub const GAUGE_CARD_FOOTER: &str = "Please wait…";

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
