pub trait Integer: Clone + Copy {
    const ZERO: Self;

    fn add(self, rhs: Self) -> Self;
    fn sub(self, rhs: Self) -> Self;
    fn mod_floor(self, rhs: Self) -> Self;
    fn greater(self, rhs: Self) -> bool;
}

pub fn align_bytes<I: Integer>(bytes: I, alignment: I) -> I {
    let rem = bytes.mod_floor(alignment);

    if rem.greater(I::ZERO) {
        bytes.add(alignment).sub(rem)
    } else {
        bytes
    }
}

macro_rules! impl_integer {
    ($($ty:ty),*) => {
        $(
            impl Integer for $ty {
                const ZERO: $ty = 0;

                fn add(self, rhs: Self) -> Self {
                    self + rhs
                }

                fn sub(self, rhs: Self) -> Self {
                    self - rhs
                }


                fn mod_floor(self, rhs: Self) -> Self {
                    self % rhs
                }

                fn greater(self, rhs: Self) -> bool {
                    self > rhs
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
