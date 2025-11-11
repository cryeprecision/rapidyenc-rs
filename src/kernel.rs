use crate::bindings;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Kernel {
    Generic,
    Sse2,
    Ssse3,
    Avx,
    Avx2,
    Vbmi2,
    Neon,
    Rvv,
    Pclmul,
    Vpclmul,
    ArmCrc,
    ArmPmull,
    Zbc,
}

impl TryFrom<i32> for Kernel {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        u32::try_from(value).map_err(|_| ())?.try_into()
    }
}

impl TryFrom<u32> for Kernel {
    type Error = ();

    fn try_from(kernel: u32) -> Result<Kernel, Self::Error> {
        match kernel {
            bindings::RYKERN_GENERIC => Ok(Kernel::Generic),
            bindings::RYKERN_SSE2 => Ok(Kernel::Sse2),
            bindings::RYKERN_SSSE3 => Ok(Kernel::Ssse3),
            bindings::RYKERN_AVX => Ok(Kernel::Avx),
            bindings::RYKERN_AVX2 => Ok(Kernel::Avx2),
            bindings::RYKERN_VBMI2 => Ok(Kernel::Vbmi2),
            bindings::RYKERN_NEON => Ok(Kernel::Neon),
            bindings::RYKERN_RVV => Ok(Kernel::Rvv),
            bindings::RYKERN_PCLMUL => Ok(Kernel::Pclmul),
            bindings::RYKERN_VPCLMUL => Ok(Kernel::Vpclmul),
            bindings::RYKERN_ARMCRC => Ok(Kernel::ArmCrc),
            bindings::RYKERN_ARMPMULL => Ok(Kernel::ArmPmull),
            bindings::RYKERN_ZBC => Ok(Kernel::Zbc),
            _ => Err(()),
        }
    }
}
