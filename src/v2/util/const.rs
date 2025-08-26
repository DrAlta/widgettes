pub trait Const {
    const ZERO: Self;
    const ONE: Self;
}

impl Const for f32 {
    const ZERO: Self = 0.0;

    const ONE: Self = 1.0;
}
impl Const for f64 {
    const ZERO: Self = 0.0;

    const ONE: Self = 1.0;
}

impl Const for i8 {
    const ZERO: Self = 0;

    const ONE: Self = 1;
}
impl Const for i32 {
    const ZERO: Self = 0;

    const ONE: Self = 1;
}
impl Const for i64 {
    const ZERO: Self = 0;

    const ONE: Self = 1;
}
