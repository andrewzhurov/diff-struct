use super::*;
use fraction::Fraction;
use fraction::Zero;

impl Diff for Fraction {
    type Repr = Fraction;

    fn diff(&self, other: &Self) -> Self::Repr {
        other - self
    }

    fn apply(&mut self, diff: &Self::Repr) {
        *self += diff
    }

    fn identity() -> Self {
        Fraction::zero()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fraction_test() {
        let mut fr1 = Fraction::new(1u64, 2u64);
        let fr2 = Fraction::new(1u64, 3u64);
        let diff = fr1.diff(&fr2);
        fr1.apply(&diff);
        assert_eq!(fr1, fr2);
    }
}
