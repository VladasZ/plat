pub struct Platform;

impl Platform {
    pub const MAC: bool = cfg!(target_os = "macos");
    pub const LINUX: bool = cfg!(target_os = "linux");
    pub const WINDOWS: bool = cfg!(target_os = "windows");
    pub const IOS: bool = cfg!(target_os = "ios");
    pub const ANDROID: bool = cfg!(target_os = "android");
    pub const DESKTOP: bool = Self::MAC || Self::LINUX || Self::WINDOWS;
    pub const MOBILE: bool = Self::IOS || Self::ANDROID;
    pub const WASM: bool = cfg!(target_arch = "wasm32");
}

impl Platform {
    pub fn dump() {
        dbg!(Self::MAC);
        dbg!(Self::LINUX);
        dbg!(Self::WINDOWS);
        dbg!(Self::IOS);
        dbg!(Self::ANDROID);
        dbg!(Self::DESKTOP);
        dbg!(Self::MOBILE);
        dbg!(Self::WASM);
    }
}

pub fn platforms() {
    cfg_aliases::cfg_aliases! {
        wasm:     {     target_arch = "wasm32"  },
        not_wasm: { not(target_arch = "wasm32") },

        android:     {     target_os = "android"  },
        not_android: { not(target_os = "android") },

        ios:     {     target_os = "ios"  },
        not_ios: { not(target_os = "ios") },

        macos:   { target_os = "macos" },
        linux:   { target_os = "linux" },
        windows: { target_os = "windows" },

        desktop: { any(target_os =   "macos", target_os = "linux", target_os = "windows") },
        mobile:  { any(target_os = "android", target_os =   "ios"                       ) },
    }
}

#[cfg(test)]
mod test {
    use crate::Platform;

    #[test]
    fn test() {
        Platform::dump();
    }
}
