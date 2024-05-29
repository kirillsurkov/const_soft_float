use super::SoftF64;

pub(crate) const fn powf(base: SoftF64, exponent: SoftF64) -> SoftF64 {
    SoftF64::exp(exponent.mul(SoftF64::ln(base)))
}

#[cfg(test)]
mod tests {
    use super::SoftF64;

    #[test]
    fn sanity_check() {
        assert_eq!(SoftF64(1.2).powf(SoftF64(3.4)).to_f64(), 1.2f64.powf(3.4));
        assert_eq!(SoftF64(1.2).powf(SoftF64(-3.4)).to_f64(), 1.2f64.powf(-3.4));
        assert_eq!(SoftF64(1.2).powf(SoftF64(0.0)).to_f64(), 1.2f64.powf(0.0));
        assert_eq!(SoftF64(0.0).powf(SoftF64(3.4)).to_f64(), 0.0f64.powf(3.4));
    }
}
