pub trait Integer: Clone + Copy {
    fn add(self, rhs: Self) -> Self;
    fn sub(self, rhs: Self) -> Self;
    fn div(self, rhs: Self) -> Self;
    fn mod_floor(self, rhs: Self) -> Self;
    fn mul(self, rhs: Self) -> Self;
    fn eq(self, rhs: Self) -> bool;
    fn greater(self, rhs: Self) -> bool;
    fn zero() -> Self;
    fn one() -> Self;
}

pub fn align_bytes<I: Integer>(bytes: I, alignment: I) -> I {
    let rem = bytes.mod_floor(alignment);

    if rem.greater(I::zero()) {
        bytes.add(alignment).sub(rem)
    } else {
        bytes
    }
}

macro_rules! impl_integer {
    ($($ty:ty),*) => {
        $(
            impl Integer for $ty {
                fn add(self, rhs: Self) -> Self {
                    self + rhs
                }

                fn sub(self, rhs: Self) -> Self {
                    self - rhs
                }

                fn div(self, rhs: Self) -> Self {
                    self / rhs
                }

                fn mod_floor(self, rhs: Self) -> Self {
                    self % rhs
                }

                fn mul(self, rhs: Self) -> Self {
                    self * rhs
                }

                fn eq(self, rhs: Self) -> bool {
                    self == rhs
                }

                fn greater(self, rhs: Self) -> bool {
                    self > rhs
                }

                fn zero() -> Self {
                    0
                }

                fn one() -> Self {
                    1
                }
            }
        )*
    };
}

impl_integer!(usize, i32);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_align_bytes() {
        assert_eq!(align_bytes(8, 16), 16_usize);
        assert_eq!(align_bytes(8, 16), 16_i32);
    }
}
