use std::sync::atomic::AtomicBool;

#[derive(Default)]
pub struct Config {
    pub allow_signups: AtomicBool,
}
