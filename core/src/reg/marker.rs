//! Marker traits for memory-mapped registers.

pub use drone_core::reg::marker::{
  self, CRoReg, CRoRoRegFieldBit, CRoRoRegFieldBits, CRoRwRegFieldBit,
  CRoRwRegFieldBits, CWoReg, CWoWoRegFieldBit, CWoWoRegFieldBits, SRoReg,
  SRoRoRegFieldBit, SRoRoRegFieldBits, SRoRwRegFieldBit, SRoRwRegFieldBits,
  SWoReg, SWoWoRegFieldBit, SWoWoRegFieldBits, URoReg, URoRoRegFieldBit,
  URoRoRegFieldBits, URoRwRegFieldBit, URoRwRegFieldBits, URwReg,
  URwRwRegFieldBit, URwRwRegFieldBits, UWoReg, UWoRwRegFieldBit,
  UWoRwRegFieldBits, UWoWoRegFieldBit, UWoWoRegFieldBits,
};

use reg::prelude::*;

// {{{ SRwReg
/// Synchronized read-write register token.
#[marker]
pub trait SRwReg
where
  Self: marker::SRwReg,
  Self: for<'a> RwRegAtomicRef<'a, Srt>,
{
}

impl<R> SRwReg for R
where
  R: marker::SRwReg,
  R: for<'a> RwRegAtomicRef<'a, Srt>,
{
}

// }}}
// {{{ CRwReg
/// Copyable read-write register token.
#[marker]
pub trait CRwReg
where
  Self: marker::CRwReg,
  Self: for<'a> RwRegAtomicRef<'a, Crt>,
{
}

impl<R> CRwReg for R
where
  R: marker::CRwReg,
  R: for<'a> RwRegAtomicRef<'a, Crt>,
{
}

// }}}
// {{{ URwRegBitBand
/// Unsynchronized bit-band read-write register token.
#[marker]
pub trait URwRegBitBand
where
  Self: URwReg,
  Self: RegBitBand<Urt>,
{
}

impl<R> URwRegBitBand for R
where
  R: URwReg,
  R: RegBitBand<Urt>,
{
}

// }}}
// {{{ URoRegBitBand
/// Unsynchronized bit-band read-only register token.
#[marker]
pub trait URoRegBitBand
where
  Self: URoReg,
  Self: RegBitBand<Urt>,
{
}

impl<R> URoRegBitBand for R
where
  R: URoReg,
  R: RegBitBand<Urt>,
{
}

// }}}
// {{{ UWoRegBitBand
/// Unsynchronized bit-band write-only register token.
#[marker]
pub trait UWoRegBitBand
where
  Self: UWoReg,
  Self: RegBitBand<Urt>,
{
}

impl<R> UWoRegBitBand for R
where
  R: UWoReg,
  R: RegBitBand<Urt>,
{
}

// }}}
// {{{ SRwRegBitBand
/// Synchronized bit-band read-write register token.
#[marker]
pub trait SRwRegBitBand
where
  Self: SRwReg,
  Self: RegBitBand<Srt>,
{
}

impl<R> SRwRegBitBand for R
where
  R: SRwReg,
  R: RegBitBand<Srt>,
{
}

// }}}
// {{{ SRoRegBitBand
/// Synchronized bit-band read-only register token.
#[marker]
pub trait SRoRegBitBand
where
  Self: SRoReg,
  Self: RegBitBand<Srt>,
{
}

impl<R> SRoRegBitBand for R
where
  R: SRoReg,
  R: RegBitBand<Srt>,
{
}

// }}}
// {{{ SWoRegBitBand
/// Synchronized bit-band write-only register token.
#[marker]
pub trait SWoRegBitBand
where
  Self: SWoReg,
  Self: RegBitBand<Srt>,
{
}

impl<R> SWoRegBitBand for R
where
  R: SWoReg,
  R: RegBitBand<Srt>,
{
}

// }}}
// {{{ CRwRegBitBand
/// Copyable bit-band read-write register token.
#[marker]
pub trait CRwRegBitBand
where
  Self: CRwReg,
  Self: RegBitBand<Crt>,
{
}

impl<R> CRwRegBitBand for R
where
  R: CRwReg,
  R: RegBitBand<Crt>,
{
}

// }}}
// {{{ CRoRegBitBand
/// Copyable bit-band read-only register token.
#[marker]
pub trait CRoRegBitBand
where
  Self: CRoReg,
  Self: RegBitBand<Crt>,
{
}

impl<R> CRoRegBitBand for R
where
  R: CRoReg,
  R: RegBitBand<Crt>,
{
}

// }}}
// {{{ CWoRegBitBand
/// Copyable bit-band write-only register token.
#[marker]
pub trait CWoRegBitBand
where
  Self: CWoReg,
  Self: RegBitBand<Crt>,
{
}

impl<R> CWoRegBitBand for R
where
  R: CWoReg,
  R: RegBitBand<Crt>,
{
}

// }}}
// {{{ SRwRwRegFieldBit
/// Synchronized one-bit read-write field of read-write register token.
#[marker]
pub trait SRwRwRegFieldBit
where
  Self: marker::SRwRwRegFieldBit,
  Self: WRwRegFieldBitAtomic<Srt>,
  Self::Reg: SRwReg,
{
}

impl<R> SRwRwRegFieldBit for R
where
  R: marker::SRwRwRegFieldBit,
  R: WRwRegFieldBitAtomic<Srt>,
  R::Reg: SRwReg,
{
}

// }}}
// {{{ SRwRwRegFieldBits
/// Synchronized multi-bit read-write field of read-write register token.
#[marker]
pub trait SRwRwRegFieldBits
where
  Self: marker::SRwRwRegFieldBits,
  Self: WRwRegFieldBitsAtomic<Srt>,
  Self::Reg: SRwReg,
{
}

impl<R> SRwRwRegFieldBits for R
where
  R: marker::SRwRwRegFieldBits,
  R: WRwRegFieldBitsAtomic<Srt>,
  R::Reg: SRwReg,
{
}

// }}}
// {{{ SWoRwRegFieldBit
/// Synchronized one-bit write-only field of read-write register token.
#[marker]
pub trait SWoRwRegFieldBit
where
  Self: marker::SWoRwRegFieldBit,
  Self: WRwRegFieldBitAtomic<Srt>,
  Self::Reg: SRwReg,
{
}

impl<R> SWoRwRegFieldBit for R
where
  R: marker::SWoRwRegFieldBit,
  R: WRwRegFieldBitAtomic<Srt>,
  R::Reg: SRwReg,
{
}

// }}}
// {{{ SWoRwRegFieldBits
/// Synchronized multi-bit write-only field of read-write register token.
#[marker]
pub trait SWoRwRegFieldBits
where
  Self: marker::SWoRwRegFieldBits,
  Self: WRwRegFieldBitsAtomic<Srt>,
  Self::Reg: SRwReg,
{
}

impl<R> SWoRwRegFieldBits for R
where
  R: marker::SWoRwRegFieldBits,
  R: WRwRegFieldBitsAtomic<Srt>,
  R::Reg: SRwReg,
{
}

// }}}
// {{{ CRwRwRegFieldBit
/// Copyable one-bit read-write field of read-write register token.
#[marker]
pub trait CRwRwRegFieldBit
where
  Self: marker::CRwRwRegFieldBit,
  Self: WRwRegFieldBitAtomic<Crt>,
  Self::Reg: CRwReg,
{
}

impl<R> CRwRwRegFieldBit for R
where
  R: marker::CRwRwRegFieldBit,
  R: WRwRegFieldBitAtomic<Crt>,
  R::Reg: CRwReg,
{
}

// }}}
// {{{ CRwRwRegFieldBits
/// Copyable multi-bit read-write field of read-write register token.
#[marker]
pub trait CRwRwRegFieldBits
where
  Self: marker::CRwRwRegFieldBits,
  Self: WRwRegFieldBitsAtomic<Crt>,
  Self::Reg: CRwReg,
{
}

impl<R> CRwRwRegFieldBits for R
where
  R: marker::CRwRwRegFieldBits,
  R: WRwRegFieldBitsAtomic<Crt>,
  R::Reg: CRwReg,
{
}

// }}}
// {{{ CWoRwRegFieldBit
/// Copyable one-bit write-only field of read-write register token.
#[marker]
pub trait CWoRwRegFieldBit
where
  Self: marker::CWoRwRegFieldBit,
  Self: WRwRegFieldBitAtomic<Crt>,
  Self::Reg: CRwReg,
{
}

impl<R> CWoRwRegFieldBit for R
where
  R: marker::CWoRwRegFieldBit,
  R: WRwRegFieldBitAtomic<Crt>,
  R::Reg: CRwReg,
{
}

// }}}
// {{{ CWoRwRegFieldBits
/// Copyable multi-bit write-only field of read-write register token.
#[marker]
pub trait CWoRwRegFieldBits
where
  Self: marker::CWoRwRegFieldBits,
  Self: WRwRegFieldBitsAtomic<Crt>,
  Self::Reg: CRwReg,
{
}

impl<R> CWoRwRegFieldBits for R
where
  R: marker::CWoRwRegFieldBits,
  R: WRwRegFieldBitsAtomic<Crt>,
  R::Reg: CRwReg,
{
}

// }}}
// {{{ URwRwRegFieldBitBand
/// Unsynchronized one-bit read-write field of read-write register token.
#[marker]
pub trait URwRwRegFieldBitBand
where
  Self: URwRwRegFieldBit,
  Self: RRRegFieldBitBand<Urt>,
  Self: WWRegFieldBitBand<Urt>,
  Self::Reg: URwRegBitBand,
{
}

impl<R> URwRwRegFieldBitBand for R
where
  R: URwRwRegFieldBit,
  R: RRRegFieldBitBand<Urt>,
  R: WWRegFieldBitBand<Urt>,
  R::Reg: URwRegBitBand,
{
}

// }}}
// {{{ UWoRwRegFieldBitBand
/// Unsynchronized one-bit write-only field of read-write register token.
#[marker]
pub trait UWoRwRegFieldBitBand
where
  Self: UWoRwRegFieldBit,
  Self: WWRegFieldBitBand<Urt>,
  Self::Reg: URwRegBitBand,
{
}

impl<R> UWoRwRegFieldBitBand for R
where
  R: UWoRwRegFieldBit,
  R: WWRegFieldBitBand<Urt>,
  R::Reg: URwRegBitBand,
{
}

// }}}
// {{{ UWoWoRegFieldBitBand
/// Unsynchronized one-bit write-only field of write-only register token.
#[marker]
pub trait UWoWoRegFieldBitBand
where
  Self: UWoWoRegFieldBit,
  Self: WWRegFieldBitBand<Urt>,
  Self::Reg: UWoRegBitBand,
{
}

impl<R> UWoWoRegFieldBitBand for R
where
  R: UWoWoRegFieldBit,
  R: WWRegFieldBitBand<Urt>,
  R::Reg: UWoRegBitBand,
{
}

// }}}
// {{{ URoRwRegFieldBitBand
/// Unsynchronized one-bit read-only field of read-write register token.
#[marker]
pub trait URoRwRegFieldBitBand
where
  Self: URoRwRegFieldBit,
  Self: RRRegFieldBitBand<Urt>,
  Self::Reg: URwRegBitBand,
{
}

impl<R> URoRwRegFieldBitBand for R
where
  R: URoRwRegFieldBit,
  R: RRRegFieldBitBand<Urt>,
  R::Reg: URwRegBitBand,
{
}

// }}}
// {{{ URoRoRegFieldBitBand
/// Unsynchronized one-bit read-only field of read-only register token.
#[marker]
pub trait URoRoRegFieldBitBand
where
  Self: URoRoRegFieldBit,
  Self: RRRegFieldBitBand<Urt>,
  Self::Reg: URoRegBitBand,
{
}

impl<R> URoRoRegFieldBitBand for R
where
  R: URoRoRegFieldBit,
  R: RRRegFieldBitBand<Urt>,
  R::Reg: URoRegBitBand,
{
}

// }}}
// {{{ SRwRwRegFieldBitBand
/// Synchronized one-bit read-write field of read-write register token.
#[marker]
pub trait SRwRwRegFieldBitBand
where
  Self: SRwRwRegFieldBit,
  Self: RRRegFieldBitBand<Srt>,
  Self: WWRegFieldBitBand<Srt>,
  Self::Reg: SRwRegBitBand,
{
}

impl<R> SRwRwRegFieldBitBand for R
where
  R: SRwRwRegFieldBit,
  R: RRRegFieldBitBand<Srt>,
  R: WWRegFieldBitBand<Srt>,
  R::Reg: SRwRegBitBand,
{
}

// }}}
// {{{ SWoRwRegFieldBitBand
/// Synchronized one-bit write-only field of read-write register token.
#[marker]
pub trait SWoRwRegFieldBitBand
where
  Self: SWoRwRegFieldBit,
  Self: WWRegFieldBitBand<Srt>,
  Self::Reg: SRwRegBitBand,
{
}

impl<R> SWoRwRegFieldBitBand for R
where
  R: SWoRwRegFieldBit,
  R: WWRegFieldBitBand<Srt>,
  R::Reg: SRwRegBitBand,
{
}

// }}}
// {{{ SWoWoRegFieldBitBand
/// Synchronized one-bit write-only field of write-only register token.
#[marker]
pub trait SWoWoRegFieldBitBand
where
  Self: SWoWoRegFieldBit,
  Self: WWRegFieldBitBand<Srt>,
  Self::Reg: SWoRegBitBand,
{
}

impl<R> SWoWoRegFieldBitBand for R
where
  R: SWoWoRegFieldBit,
  R: WWRegFieldBitBand<Srt>,
  R::Reg: SWoRegBitBand,
{
}

// }}}
// {{{ SRoRwRegFieldBitBand
/// Synchronized one-bit read-only field of read-write register token.
#[marker]
pub trait SRoRwRegFieldBitBand
where
  Self: SRoRwRegFieldBit,
  Self: RRRegFieldBitBand<Srt>,
  Self::Reg: SRwRegBitBand,
{
}

impl<R> SRoRwRegFieldBitBand for R
where
  R: SRoRwRegFieldBit,
  R: RRRegFieldBitBand<Srt>,
  R::Reg: SRwRegBitBand,
{
}

// }}}
// {{{ SRoRoRegFieldBitBand
/// Synchronized one-bit read-only field of read-only register token.
#[marker]
pub trait SRoRoRegFieldBitBand
where
  Self: SRoRoRegFieldBit,
  Self: RRRegFieldBitBand<Srt>,
  Self::Reg: SRoRegBitBand,
{
}

impl<R> SRoRoRegFieldBitBand for R
where
  R: SRoRoRegFieldBit,
  R: RRRegFieldBitBand<Srt>,
  R::Reg: SRoRegBitBand,
{
}

// }}}
// {{{ CRwRwRegFieldBitBand
/// Copyable one-bit read-write field of read-write register token.
#[marker]
pub trait CRwRwRegFieldBitBand
where
  Self: CRwRwRegFieldBit,
  Self: RRRegFieldBitBand<Crt>,
  Self: WWRegFieldBitBand<Crt>,
  Self::Reg: CRwRegBitBand,
{
}

impl<R> CRwRwRegFieldBitBand for R
where
  R: CRwRwRegFieldBit,
  R: RRRegFieldBitBand<Crt>,
  R: WWRegFieldBitBand<Crt>,
  R::Reg: CRwRegBitBand,
{
}

// }}}
// {{{ CWoRwRegFieldBitBand
/// Copyable one-bit write-only field of read-write register token.
#[marker]
pub trait CWoRwRegFieldBitBand
where
  Self: CWoRwRegFieldBit,
  Self: WWRegFieldBitBand<Crt>,
  Self::Reg: CRwRegBitBand,
{
}

impl<R> CWoRwRegFieldBitBand for R
where
  R: CWoRwRegFieldBit,
  R: WWRegFieldBitBand<Crt>,
  R::Reg: CRwRegBitBand,
{
}

// }}}
// {{{ CWoWoRegFieldBitBand
/// Copyable one-bit write-only field of write-only register token.
#[marker]
pub trait CWoWoRegFieldBitBand
where
  Self: CWoWoRegFieldBit,
  Self: WWRegFieldBitBand<Crt>,
  Self::Reg: CWoRegBitBand,
{
}

impl<R> CWoWoRegFieldBitBand for R
where
  R: CWoWoRegFieldBit,
  R: WWRegFieldBitBand<Crt>,
  R::Reg: CWoRegBitBand,
{
}

// }}}
// {{{ CRoRwRegFieldBitBand
/// Copyable one-bit read-only field of read-write register token.
#[marker]
pub trait CRoRwRegFieldBitBand
where
  Self: CRoRwRegFieldBit,
  Self: RRRegFieldBitBand<Crt>,
  Self::Reg: CRwRegBitBand,
{
}

impl<R> CRoRwRegFieldBitBand for R
where
  R: CRoRwRegFieldBit,
  R: RRRegFieldBitBand<Crt>,
  R::Reg: CRwRegBitBand,
{
}

// }}}
// {{{ CRoRoRegFieldBitBand
/// Copyable one-bit read-only field of read-only register token.
#[marker]
pub trait CRoRoRegFieldBitBand
where
  Self: CRoRoRegFieldBit,
  Self: RRRegFieldBitBand<Crt>,
  Self::Reg: CRoRegBitBand,
{
}

impl<R> CRoRoRegFieldBitBand for R
where
  R: CRoRoRegFieldBit,
  R: RRRegFieldBitBand<Crt>,
  R::Reg: CRoRegBitBand,
{
}

// }}}
// vim: set fdm=marker fmr={{{,}}} :