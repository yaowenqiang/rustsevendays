extern crate winreg;
use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_READ};

fn main() {
    let hklm = winreg::RegKey::predef(HKEY_LOCAL_MACHINE);
    let subkey = hklm
        .open_subkey_with_flags(
            r#"
        SOFTWARE\Microsoft\Windows NT\currentVersion"#,
            KEY_READ,
        )
        .expect("Failed to open subkey");
    let product_name: String = subkey_get_value("ProductName");
    println!("windows version: {}", product_name);
    println!("Hello, world!");
}
