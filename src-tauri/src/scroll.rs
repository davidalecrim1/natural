use std::process::Command;

fn parse_scroll_output(s: &str) -> bool {
    s.trim() == "1"
}

/// Returns true if natural scrolling is currently enabled.
pub fn is_natural_scrolling() -> bool {
    let output = Command::new("defaults")
        .args(["read", "-g", "com.apple.swipescrolldirection"])
        .output();

    match output {
        Ok(out) => parse_scroll_output(&String::from_utf8_lossy(&out.stdout)),
        Err(_) => true,
    }
}

/// Toggle natural scrolling and return the new state.
pub fn toggle() -> bool {
    let new_value = !is_natural_scrolling();
    set(new_value);
    new_value
}

fn set(natural: bool) {
    let bool_str = if natural { "true" } else { "false" };

    let _ = Command::new("defaults")
        .args([
            "write",
            "-g",
            "com.apple.swipescrolldirection",
            "-bool",
            bool_str,
        ])
        .output();

    apply_immediately(natural);
}

/// Calls setSwipeScrollDirection from PreferencePanesSupport.framework
/// for instant effect without logout. Falls back to DistributedNotification.
fn apply_immediately(natural: bool) {
    let applied = unsafe { try_private_framework(natural) };

    if !applied {
        post_notification_fallback();
    }
}

unsafe fn try_private_framework(natural: bool) -> bool {
    let lib = unsafe {
        libloading::Library::new(
            "/System/Library/PrivateFrameworks/PreferencePanesSupport.framework/PreferencePanesSupport",
        )
    };

    let Ok(lib) = lib else {
        return false;
    };

    let func: Result<libloading::Symbol<unsafe extern "C" fn(bool)>, _> =
        unsafe { lib.get(b"setSwipeScrollDirection") };

    if let Ok(set_direction) = func {
        unsafe { set_direction(natural) };
        true
    } else {
        false
    }
}

fn post_notification_fallback() {
    let script = r#"
        import Foundation
        DistributedNotificationCenter.default().postNotificationName(
            NSNotification.Name("SwipeScrollDirectionDidChangeNotification"),
            object: nil, userInfo: nil, deliverImmediately: true
        )
    "#;
    let _ = Command::new("swift").args(["-e", script]).output();
}

#[cfg(test)]
mod tests {
    use super::parse_scroll_output;

    #[test]
    fn natural_on() {
        assert!(parse_scroll_output("1"));
    }

    #[test]
    fn natural_off() {
        assert!(!parse_scroll_output("0"));
    }

    #[test]
    fn empty_string() {
        assert!(!parse_scroll_output(""));
    }

    #[test]
    fn trims_whitespace() {
        assert!(parse_scroll_output("1\n"));
    }
}
