use formal_power_series::FormalPowerSeries;
use pentagonal_number_theorem::pentagonal_number_theorem;

/// 分割数 p(0), p(1), ... p(n-1) を列挙する  
/// 分割数 p(n) は、 n を順番の違いを除いて自然数の和として表す方法の数
pub fn partition<const P: u32>(n: usize) -> FormalPowerSeries<P> {
    let f = FormalPowerSeries(pentagonal_number_theorem(n));
    f.inv(n)
}
