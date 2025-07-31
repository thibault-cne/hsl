pub trait Integer: Clone + Copy {
    fn add(self, rhs: Self) -> Self;
    fn div(self, rhs: Self) -> Self;
    fn mod_floor(self, rhs: Self) -> Self;
    fn mul(self, rhs: Self) -> Self;
    fn eq(self, rhs: Self) -> bool;
    fn zero() -> Self;
    fn one() -> Self;
}

pub fn smallest_multiple_greater_than<I: Integer>(multiple: I, threshold: I) -> I {
    if threshold.mod_floor(multiple).eq(I::zero()) {
        threshold
    } else {
        (threshold.div(multiple).add(I::one())).mul(multiple)
    }
}

macro_rules! impl_integer {
    ($($ty:ty),*) => {
        $(
            impl Integer for $ty {
                fn add(self, rhs: Self) -> Self {
                    self + rhs
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
    fn test_smallest_multiple_greater_than() {
        assert_eq!(smallest_multiple_greater_than(16, 8), 16_usize);
        assert_eq!(smallest_multiple_greater_than(16, 8), 16_i32);
    }
}
