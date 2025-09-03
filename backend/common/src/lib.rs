pub mod models;
pub mod appstate;
pub mod middlewares;
pub mod crypto;
mod config;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CONFIG : config::Config = config::Config::init();
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
