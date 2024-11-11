use super::actions::CornerAction;
use std::process::Command;

#[cfg(target_os = "macos")]
pub struct MacCornerAction;

#[cfg(target_os = "macos")]
impl CornerAction for MacCornerAction {
    fn go_left(&self) {
        // AppleScript command to switch to the left workspace with Control + Left Arrow
        let script = r#"
        tell application "System Events"
            key code 123 using {control down}
        end tell
    "#;

        // Execute the AppleScript command
        let output = Command::new("osascript")
            .arg("-e")
            .arg(script)
            .output()
            .expect("Failed to execute AppleScript");

        if output.status.success() {
        } else {
            eprintln!(
                "Error switching workspace: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }

    fn go_right(&self) {
        // AppleScript command to switch to the left workspace with Control + Left Arrow
        let script = r#"
        tell application "System Events"
            key code 124 using {control down}
        end tell
    "#;

        // Execute the AppleScript command
        let output = Command::new("osascript")
            .arg("-e")
            .arg(script)
            .output()
            .expect("Failed to execute AppleScript");

        if output.status.success() {
        } else {
            eprintln!(
                "Error switching workspace: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }

    fn open_window_tray(&self) {
        // AppleScript command to switch to the left workspace with Control + Left Arrow
        let script = r#"
        tell application "System Events"
            key code 126 using {control down}
        end tell
    "#;

        // Execute the AppleScript command
        let output = Command::new("osascript")
            .arg("-e")
            .arg(script)
            .output()
            .expect("Failed to execute AppleScript");

        if output.status.success() {
        } else {
            eprintln!(
                "Error switching workspace: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }
}
