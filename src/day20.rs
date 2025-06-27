use std::marker::PhantomData;

pub struct Empty;
pub struct Ready;
pub struct Flying;

// TODO: Define the `status` method for all states
pub trait Status {
    fn status() -> &'static str;
}

macro_rules! sleigh_status {
    ($($state:ident),+) => {
        $(
        impl Status for $state {
            fn status() -> &'static str {
                stringify!($state)
            }
        })*
    };
}

sleigh_status!(Empty, Ready, Flying);

pub struct Sleigh<T: Status> {
    // This is only public for testing purposes
    // In real-world scenarios, this should be private
    pub state: PhantomData<T>,
}

impl<T: Status> Sleigh<T> {
    pub fn status(&self) -> &'static str {
        T::status()
    }
}

impl Sleigh<Empty> {
    pub fn new() -> Self {
        Self { state: PhantomData }
    }

    pub fn load(self) -> Sleigh<Ready> {
        Sleigh { state: PhantomData }
    }
}

impl Sleigh<Ready> {
    pub fn take_off(self) -> Sleigh<Flying> {
        Sleigh { state: PhantomData }
    }

    pub fn unload(self) -> Sleigh<Empty> {
        Sleigh { state: PhantomData }
    }
}

impl Sleigh<Flying> {
    pub fn land(self) -> Sleigh<Ready> {
        Sleigh { state: PhantomData }
    }
}
