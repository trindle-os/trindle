//! Trindle's keyring: the Secret Service apps keep passwords in, opened by
//! the login password.

pub mod gnome;
pub mod migrate;
pub mod pam;
pub mod prompt;
pub mod renew;
pub mod service;
pub mod store;

pub fn main(args: Vec<String>) -> i32 {
    match args.get(1).map(String::as_str) {
        Some("--pam-auth") => pam::helper(),
        Some("--pam-move") => migrate::at_login(),
        _ => service::run(),
    }
}
