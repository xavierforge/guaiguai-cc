/// Send Ctrl+C (interrupt), type a message, then press Enter.
/// This is the equivalent of badclaude's sendMacro().

#[cfg(target_os = "macos")]
pub fn send_macro(text: &str) -> Result<(), String> {
    let escaped = text.replace('\\', "\\\\").replace('"', "\\\"");
    let script = format!(
        r#"tell application "System Events"
  key code 8 using {{control down}}
  delay 0.5
  keystroke "{escaped}"
  key code 36
end tell"#
    );

    std::process::Command::new("osascript")
        .arg("-e")
        .arg(&script)
        .output()
        .map_err(|e| format!("osascript failed: {e}"))?;
    Ok(())
}

#[cfg(target_os = "windows")]
pub fn send_macro(text: &str) -> Result<(), String> {
    use windows::Win32::UI::Input::KeyboardAndMouse::*;

    unsafe {
        // Ctrl down
        let inputs = [make_key_input(VK_CONTROL, KEYBD_EVENT_FLAGS(0))];
        SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
        
        // C down/up
        send_key_press(VK_C);
        
        // Ctrl up
        let inputs = [make_key_input(VK_CONTROL, KEYEVENTF_KEYUP)];
        SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);

        std::thread::sleep(std::time::Duration::from_millis(100));

        // Type the message
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
        inputs.push(make_key_input(vk, 0));
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
    eprintln!("[baiclaude] keyboard injection not implemented for this platform. would send: {text}");
    Ok(())
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub fn alt_tab() {}
