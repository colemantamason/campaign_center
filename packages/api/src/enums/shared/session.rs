use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

// used by both redis cache TTL and postgres session expiry (7 days in seconds)
pub const DEFAULT_SESSION_EXPIRY_SECONDS: i64 = 604800;

// platform from which the session was created
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum Platform {
    Web,
    Mobile,
}

impl Platform {
    pub fn as_str(&self) -> &'static str {
        match self {
            Platform::Web => "web",
            Platform::Mobile => "mobile",
        }
    }

    pub fn from_str(string: &str) -> Option<Self> {
        match string {
            "web" => Some(Platform::Web),
            "mobile" => Some(Platform::Mobile),
            _ => None,
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Platform::Web => "Web",
            Platform::Mobile => "Mobile",
        }
    }
}

impl Display for Platform {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        write!(formatter, "{}", self.as_str())
    }
}

// device information parsed from user agent string
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeviceInfo {
    pub os: Option<String>,
    pub os_version: Option<String>,
    pub browser: Option<String>,
    pub device_type: Option<String>,
}

impl DeviceInfo {
    // parse device info from user agent string
    pub fn from_user_agent(user_agent: &str) -> Self {
        let user_agent = user_agent.to_lowercase();

        // detect OS
        let (os, os_version) = if user_agent.contains("iphone")
            || user_agent.contains("ipad")
            || user_agent.contains("ios")
        {
            let version = Self::extract_ios_version(&user_agent);
            (Some("iOS".to_string()), version)
        } else if user_agent.contains("android") {
            let version = Self::extract_android_version(&user_agent);
            (Some("Android".to_string()), version)
        } else if user_agent.contains("windows") {
            (
                Some("Windows".to_string()),
                Self::extract_windows_version(&user_agent),
            )
        } else if user_agent.contains("mac os")
            || user_agent.contains("macos")
            || user_agent.contains("macintosh")
        {
            (
                Some("macOS".to_string()),
                Self::extract_macos_version(&user_agent),
            )
        } else if user_agent.contains("linux") {
            (Some("Linux".to_string()), None)
        } else {
            (None, None)
        };

        // detect device type
        let device_type = if user_agent.contains("iphone") {
            Some("iPhone".to_string())
        } else if user_agent.contains("ipad") {
            Some("iPad".to_string())
        } else if user_agent.contains("pixel") {
            Some("Pixel".to_string())
        } else if user_agent.contains("samsung") || user_agent.contains("sm-") {
            Some("Samsung Galaxy".to_string())
        } else {
            None
        };

        // detect browser (for web sessions)
        let browser = if user_agent.contains("edg/") {
            Some("Edge".to_string())
        } else if user_agent.contains("chrome") && !user_agent.contains("chromium") {
            Some("Chrome".to_string())
        } else if user_agent.contains("safari") && !user_agent.contains("chrome") {
            Some("Safari".to_string())
        } else if user_agent.contains("firefox") {
            Some("Firefox".to_string())
        } else {
            None
        };

        DeviceInfo {
            os,
            os_version,
            browser,
            device_type,
        }
    }

    // returns a human-readable description like "iPhone (iOS 17.2)" or "Chrome on macOS"
    pub fn display_string(&self, platform: Platform) -> String {
        match platform {
            Platform::Mobile => {
                let device = self.device_type.as_deref().unwrap_or("Mobile Device");
                match (&self.os, &self.os_version) {
                    (Some(os), Some(version)) => format!("{} ({} {})", device, os, version),
                    (Some(os), None) => format!("{} ({})", device, os),
                    _ => device.to_string(),
                }
            }
            Platform::Web => {
                let browser = self.browser.as_deref().unwrap_or("Browser");
                match &self.os {
                    Some(os) => format!("{} on {}", browser, os),
                    None => browser.to_string(),
                }
            }
        }
    }

    fn extract_ios_version(user_agent: &str) -> Option<String> {
        // pattern: "OS 17_2" or "iOS 17.2"
        let patterns = ["os ", "ios "];
        for pattern in patterns {
            if let Some(start) = user_agent.to_lowercase().find(pattern) {
                let version_start = start + pattern.len();
                let version: String = user_agent[version_start..]
                    .chars()
                    .take_while(|character| {
                        character.is_ascii_digit() || *character == '_' || *character == '.'
                    })
                    .collect();
                if !version.is_empty() {
                    return Some(version.replace('_', "."));
                }
            }
        }
        None
    }

    fn extract_android_version(user_agent: &str) -> Option<String> {
        // pattern: "Android 14" or "Android 13.0"
        if let Some(start) = user_agent.to_lowercase().find("android ") {
            let version_start = start + 8;
            let version: String = user_agent[version_start..]
                .chars()
                .take_while(|character| character.is_ascii_digit() || *character == '.')
                .collect();
            if !version.is_empty() {
                return Some(version);
            }
        }
        None
    }

    fn extract_windows_version(user_agent: &str) -> Option<String> {
        // pattern: "Windows NT 10.0"
        if let Some(start) = user_agent.to_lowercase().find("windows nt ") {
            let version_start = start + 11;
            let version: String = user_agent[version_start..]
                .chars()
                .take_while(|character| character.is_ascii_digit() || *character == '.')
                .collect();
            if !version.is_empty() {
                // map NT versions to marketing names
                return Some(match version.as_str() {
                    "10.0" => "10/11".to_string(),
                    "6.3" => "8.1".to_string(),
                    "6.2" => "8".to_string(),
                    "6.1" => "7".to_string(),
                    _ => version,
                });
            }
        }
        None
    }

    fn extract_macos_version(user_agent: &str) -> Option<String> {
        // pattern: "Mac OS X 10_15_7" or "macOS 14.2"
        let patterns = ["mac os x ", "macos "];
        for pattern in patterns {
            if let Some(start) = user_agent.to_lowercase().find(pattern) {
                let version_start = start + pattern.len();
                let version: String = user_agent[version_start..]
                    .chars()
                    .take_while(|character| {
                        character.is_ascii_digit() || *character == '_' || *character == '.'
                    })
                    .collect();
                if !version.is_empty() {
                    return Some(version.replace('_', "."));
                }
            }
        }
        None
    }
}
