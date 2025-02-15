//! `SpiDevice` implementations.

use core::fmt::Debug;
use embedded_hal::spi::{Error, ErrorKind};

mod exclusive;
pub use exclusive::*;
mod refcell;
pub use refcell::*;
#[cfg(feature = "std")]
mod mutex;
#[cfg(feature = "std")]
pub use mutex::*;
mod critical_section;
pub use self::critical_section::*;

#[cfg(feature = "defmt-03")]
use crate::defmt;

/// Error type for [`ExclusiveDevice`] operations.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum DeviceError<BUS, CS> {
    /// An inner SPI bus operation failed.
    Spi(BUS),
    /// Asserting or deasserting CS failed.
    Cs(CS),
}

impl<BUS, CS> Error for DeviceError<BUS, CS>
where
    BUS: Error + Debug,
    CS: Debug,
{
    #[inline]
    fn kind(&self) -> ErrorKind {
        match self {
            Self::Spi(e) => e.kind(),
            Self::Cs(_) => ErrorKind::ChipSelectFault,
        }
    }
}

/// Dummy `DelayUs` implementation that panics on use.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub struct NoDelay;

#[cold]
fn no_delay_panic() {
    panic!("You've tried to execute a SPI transaction containing a `Operation::Delay` in a `SpiDevice` created with `new_no_delay()`. Create it with `new()` instead, passing a `DelayUs` implementation.");
}

impl embedded_hal::delay::DelayUs for NoDelay {
    #[inline]
    fn delay_us(&mut self, _us: u32) {
        no_delay_panic();
    }
}

#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
impl embedded_hal_async::delay::DelayUs for NoDelay {
    #[inline]
    async fn delay_us(&mut self, _us: u32) {
        no_delay_panic();
    }

    #[inline]
    async fn delay_ms(&mut self, _ms: u32) {
        no_delay_panic();
    }
}
