//! Platform-specific macro sender: paste a message into the focused app,
//! then press Enter.
//!
//! - macOS: put the text on the clipboard via AppleScript's `set the clipboard`,
//!   then Cmd+V via AppleScript. Going through AppleScript for both steps avoids
//!   relying on an external `pbcopy` subprocess, which was silently failing when
//!   the app was launched from `/Applications` instead of `cargo tauri dev`.
//! - Windows: type each char as Unicode via `SendInput`.
//!
//! Going through the clipboard / Unicode paths avoids the IME and modifier
//! quirks that AppleScript `keystroke` runs into.

#[cfg(target_os = "macos")]
pub fn send_macro(text: &str) -> Result<(), String> {
    // Escape the phrase so it survives embedding in an AppleScript string literal.
    // Only backslash and double-quote need escaping inside an AS "…" literal;
    // phrases do not contain newlines.
    let escaped = text.replace('\\', "\\\\").replace('"', "\\\"");

    // One osascript call does all three steps:
    //   1. set the clipboard to the phrase
    //   2. Cmd+V via `key code 9 using {command down}` (raw virtual-key so
    //      the Command modifier is not lost to the char translation path that
    //      `keystroke "v"` takes)
    //   3. Return
    let script = format!(
        r#"set the clipboard to "{escaped}"
tell application "System Events"
  key code 9 using {{command down}}
  delay 0.2
  key code 36
  delay 0.05
end tell"#
    );
    let output = std::process::Command::new("/usr/bin/osascript")
        .arg("-e")
        .arg(&script)
        .output()
        .map_err(|e| format!("osascript spawn failed: {e}"))?;
    if !output.status.success() {
        return Err(format!(
            "osascript failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    Ok(())
}

#[cfg(target_os = "windows")]
use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP,
    KEYEVENTF_UNICODE, VIRTUAL_KEY, VK_MENU, VK_RETURN, VK_TAB,
};

#[cfg(target_os = "windows")]
pub fn send_macro(text: &str) -> Result<(), String> {
    unsafe {
        // Type the message as Unicode (handles CJK / emoji without IME issues)
        for ch in text.chars() {
            send_char(ch);
        }

        // Enter
        send_key_press(VK_RETURN);
    }
    Ok(())
}

#[cfg(target_os = "windows")]
unsafe fn send_key_combo(keys: &[VIRTUAL_KEY]) {
    use windows::Win32::UI::Input::KeyboardAndMouse::*;

    let mut inputs: Vec<INPUT> = Vec::new();

    // Key downs
    for &vk in keys {
        inputs.push(make_key_input(vk, KEYBD_EVENT_FLAGS(0)));
    }
    // Key ups (reverse order)
    for &vk in keys.iter().rev() {
        inputs.push(make_key_input(vk, KEYEVENTF_KEYUP));
    }

    SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
}

#[cfg(target_os = "windows")]
unsafe fn send_key_press(vk: VIRTUAL_KEY) {
    use windows::Win32::UI::Input::KeyboardAndMouse::*;
    let inputs = [
        make_key_input(vk, KEYBD_EVENT_FLAGS(0)),
        make_key_input(vk, KEYEVENTF_KEYUP),
    ];
    SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
}

#[cfg(target_os = "windows")]
unsafe fn send_char(ch: char) {
    use windows::Win32::UI::Input::KeyboardAndMouse::*;
    let inputs = [
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VIRTUAL_KEY(0),
                    wScan: ch as u16,
                    dwFlags: KEYEVENTF_UNICODE,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VIRTUAL_KEY(0),
                    wScan: ch as u16,
                    dwFlags: KEYEVENTF_UNICODE | KEYEVENTF_KEYUP,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
    ];
    SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
}

#[cfg(target_os = "windows")]
fn make_key_input(
    vk: windows::Win32::UI::Input::KeyboardAndMouse::VIRTUAL_KEY,
    flags: windows::Win32::UI::Input::KeyboardAndMouse::KEYBD_EVENT_FLAGS,
) -> windows::Win32::UI::Input::KeyboardAndMouse::INPUT {
    use windows::Win32::UI::Input::KeyboardAndMouse::*;
    INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: vk,
                wScan: 0,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    }
}

#[cfg(target_os = "windows")]
pub fn alt_tab() {
    unsafe {
        send_key_combo(&[
            windows::Win32::UI::Input::KeyboardAndMouse::VK_MENU,
            windows::Win32::UI::Input::KeyboardAndMouse::VK_TAB,
        ]);
    }
}

// Linux: not yet implemented, but stub to compile
#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub fn send_macro(text: &str) -> Result<(), String> {
    eprintln!("[guaiguai-claude] keyboard injection not implemented for this platform. would send: {text}");
    Ok(())
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub fn alt_tab() {}
