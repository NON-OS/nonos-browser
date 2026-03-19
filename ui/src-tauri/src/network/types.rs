pub const NYM_VERSION: &str = "nym-binaries-v2026.5-raclette";

pub struct PlatformBinary {
    pub filename: &'static str,
    pub sha256: &'static str,
}

pub const LINUX_X86_64: PlatformBinary = PlatformBinary {
    filename: "nym-socks5-client",
    sha256: "e52966dd7bea4a9d18f07de38bd2344955425bf44b4a89b952ea660a826f7983",
};

pub fn get_platform_binary() -> Result<PlatformBinary, String> {
    match (std::env::consts::OS, std::env::consts::ARCH) {
        ("linux", "x86_64") => Ok(LINUX_X86_64),
        (os, arch) => Err(format!(
            "No prebuilt Nym binary for {}-{}. Install nym-socks5-client manually. \
             Build from source: cargo install nym-socks5-client --git https://github.com/nymtech/nym",
            os, arch
        )),
    }
}

pub fn get_binary_name() -> &'static str {
    if std::env::consts::OS == "windows" {
        "nym-socks5-client.exe"
    } else {
        "nym-socks5-client"
    }
}
