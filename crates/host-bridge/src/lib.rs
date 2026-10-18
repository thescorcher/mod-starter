//! DLL the mod loader injects into the host game (me3, ASI loader, SKSE-style plugin, ...).
//!
//! Current stage: proves the DLL loads and that guest-core runs inside the game process. It writes a
//! log and does not read or write game memory yet.

use std::ffi::c_void;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use guest_core::{HostInput, JumpParams, Movement};

const DLL_PROCESS_ATTACH: u32 = 1;

/// Log goes to `local/logs/` in this project, never into the game folder.
/// Override with the MOD_LOG environment variable.
fn log_path() -> PathBuf {
    std::env::var_os("MOD_LOG").map(PathBuf::from).unwrap_or_else(|| {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../local/logs/host-bridge.log")
    })
}

fn log(line: &str) {
    let path = log_path();
    if let Some(dir) = path.parent() {
        let _ = fs::create_dir_all(dir);
    }
    if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(path) {
        let secs = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
        let _ = writeln!(f, "[{secs}] {line}");
        let _ = f.flush();
    }
}

fn run() {
    log(concat!("host-bridge ", env!("CARGO_PKG_VERSION"), " loaded"));

    // Self-check: run one simulated jump through guest-core and log the apex.
    let result = std::panic::catch_unwind(|| {
        let mut m = Movement::new(JumpParams::default());
        m.step(HostInput { jump_pressed: true, grounded: true });
        let mut height = 0.0f32;
        let mut apex = 0.0f32;
        for _ in 0..120 {
            let out = m.step(HostInput { jump_pressed: false, grounded: false });
            height += out.vertical_velocity * guest_core::TICK_DT;
            apex = apex.max(height);
        }
        apex
    });
    match result {
        Ok(apex) => log(&format!("guest-core self-check ok, simulated jump apex {apex:.2} m")),
        Err(_) => log("guest-core self-check panicked"),
    }
}

/// Windows calls this when the DLL loads. Do almost nothing here (the loader lock is held);
/// start a thread and return.
#[no_mangle]
pub extern "system" fn DllMain(_module: *mut c_void, reason: u32, _reserved: *mut c_void) -> i32 {
    if reason == DLL_PROCESS_ATTACH {
        std::thread::spawn(run);
    }
    1
}
