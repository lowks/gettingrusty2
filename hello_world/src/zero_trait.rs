trait Zero {
    const ZERO: Self;
    fn am_i_zero(&self) -> bool;
}

impl Zero for i32 {
    const ZERO: Self = 0;
    fn am_i_zero(&self) -> bool {
        *self == Self::ZERO
    }
}

impl Zero for &str {
    const ZERO: Self = "zero";
    fn am_i_zero(&self) -> bool {
        *self == Self::ZERO
    }
}

// #[cfg(test)]

// assert_eq!(i32::ZERO, 0);
// assert_eq!(&str::ZERO, "zero");