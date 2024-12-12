use formal_power_series::FormalPowerSeries;
use pentagonal_number_theorem::pentagonal_number_theorem;

pub fn partition<const P: u32>(n: usize) -> FormalPowerSeries<P> {
    let f = FormalPowerSeries(pentagonal_number_theorem(n));
    f.inv(n)
}
