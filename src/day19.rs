// 1. We have 3 states:
// - Empty
// - Ready
// - Flying
pub struct Empty;
pub struct Ready;
pub struct Flying;

// 2. Finish the Seligh struct definition
pub struct Sleigh<T> {
    _whatever: T,
}

// 3. Write the Sleigh Implementations for all states
impl Sleigh<Empty> {
    pub fn new() -> Self {
        Sleigh::<Empty> { _whatever: Empty }
    }

    pub fn load(&self) -> Sleigh<Ready> {
        Sleigh::<Ready> { _whatever: Ready }
    }
}

impl Sleigh<Ready> {
    pub fn take_off(&self) -> Sleigh<Flying> {
        Sleigh::<Flying> { _whatever: Flying }
    }

    pub fn unload(&self) -> Sleigh<Ready> {
        Sleigh::<Ready> { _whatever: Ready }
    }
}

impl Sleigh<Flying> {
    pub fn land(&self) -> Sleigh<Ready> {
        Sleigh::<Ready> { _whatever: Ready }
    }
}
