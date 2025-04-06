mod config;
mod pool;

pub use config::*;
pub use pool::*;

use steel::*;

use crate::consts::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum SteelAmmAccount {
    Config = 0,
    Pool = 1,
}
