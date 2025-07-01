use std::marker::PhantomData;

// 1. We have 3 states:
// - Empty
// - Ready
// - Flying
pub struct Empty;
pub struct Ready;
pub struct Flying;

// 2. Finish the Seligh struct definition
pub struct Sleigh<T> {
    pub state: PhantomData<T>,
}

impl Default for Sleigh<Empty> {
    fn default() -> Self {
        Self { state: PhantomData }
    }
}

// 3. Write the Sleigh Implementations for all states
impl Sleigh<Empty> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load(&self) -> Sleigh<Ready> {
        Sleigh::<Ready> { state: PhantomData }
    }
}

impl Sleigh<Ready> {
    pub fn take_off(&self) -> Sleigh<Flying> {
        Sleigh::<Flying> { state: PhantomData }
    }

    pub fn unload(&self) -> Sleigh<Ready> {
        Sleigh::<Ready> { state: PhantomData }
    }
}

impl Sleigh<Flying> {
    pub fn land(&self) -> Sleigh<Ready> {
        Sleigh::<Ready> { state: PhantomData }
    }
}
