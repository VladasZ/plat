pub struct Platform;

impl Platform {
    pub const MAC: bool = cfg!(target_os = "macos");
    pub const WIN: bool = cfg!(target_os = "windows");
    pub const IOS: bool = cfg!(target_os = "ios");
    pub const ANDROID: bool = cfg!(target_os = "android");
    pub const MOBILE: bool = Self::IOS || Self::ANDROID;
    pub const DESKTOP: bool = !Self::MOBILE;
    pub const WASM: bool = cfg!(target_arch = "wasm32");
}

impl Platform {
    pub fn dump() {
        dbg!(Self::MAC);
        dbg!(Self::WIN);
        dbg!(Self::IOS);
        dbg!(Self::ANDROID);
        dbg!(Self::MOBILE);
        dbg!(Self::DESKTOP);
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

        mobile:  { any(    target_os = "android", target_os = "ios")  },
        desktop: { not(any(target_os = "android", target_os = "ios")) },
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
