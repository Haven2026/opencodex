//! Resolve the packaged `ocx` command without exposing platform packaging details to callers.
//!
//! Tauri's Linux AppImage pipeline sends every `externalBin` through linuxdeploy's ELF dependency
//! pass. The bundled CLI is a self-contained Bun executable: rewriting it with the external-bin
//! `RUNPATH=$ORIGIN/../lib` makes that executable crash before `main`, so it must be carried as an
//! ordinary executable resource on Linux. macOS and Windows keep Tauri's native sidecar path and
//! naming rules.

use tauri::AppHandle;
#[cfg(target_os = "linux")]
use tauri::Manager;
use tauri_plugin_shell::{process::Command, ShellExt};

#[cfg(target_os = "linux")]
fn linux_resource(resource_dir: &std::path::Path) -> std::path::PathBuf {
    resource_dir.join("sidecar").join("ocx")
}

/// Build a command for the trusted CLI embedded in this application package.
pub fn command(app: &AppHandle) -> Result<Command, String> {
    #[cfg(target_os = "linux")]
    {
        let resource_dir = app
            .path()
            .resource_dir()
            .map_err(|error| error.to_string())?;
        let executable = linux_resource(&resource_dir);
        if !executable.is_file() {
            return Err(format!(
                "the bundled CLI resource is missing ({})",
                executable.display()
            ));
        }
        Ok(app.shell().command(executable))
    }

    #[cfg(not(target_os = "linux"))]
    {
        app.shell()
            .sidecar("ocx")
            .map_err(|error| error.to_string())
    }
}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::linux_resource;
    use std::path::Path;

    #[test]
    fn the_linux_cli_lives_under_the_packaged_resource_root() {
        assert_eq!(
            linux_resource(Path::new("/opt/OpenCodex/resources")),
            Path::new("/opt/OpenCodex/resources/sidecar/ocx")
        );
    }
}
