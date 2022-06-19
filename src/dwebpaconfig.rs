pub struct DwebpaConfig {
    pub auto_accept_conversion: bool,
    pub auto_decline_cleanup: bool,
    pub auto_accept_cleanup: bool,
    pub quiet: bool
}

impl DwebpaConfig {
    pub fn new() -> DwebpaConfig {
        DwebpaConfig {
            auto_accept_cleanup: false,
            auto_decline_cleanup: false,
            auto_accept_conversion: false,
            quiet: false
        }
    }
}