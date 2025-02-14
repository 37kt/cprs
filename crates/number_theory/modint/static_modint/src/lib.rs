use ntt_precalc::NTTPrecalc;

mod mod_arithmetic;
mod ntt_precalc;
pub mod ops;

pub struct StaticModInt<const MOD: u32>(u32);

impl<const MOD: u32> StaticModInt<MOD> {
    pub const NTT_PRECALC: NTTPrecalc = NTTPrecalc::new(MOD);
}

#[test]
fn test() {
    let a = StaticModInt::<1223>::NTT_PRECALC;
    eprintln!("{}", a.primitive_root);
}
