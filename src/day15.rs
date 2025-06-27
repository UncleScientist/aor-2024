use std::fmt;

// 1. Add the `is_wrapped` field to the gift structs
pub struct KidsGift {
    pub name: String,
    pub is_wrapped: bool,
}

pub struct ElvesGift {
    pub name: String,
    pub is_wrapped: bool,
}

pub struct ReindeerGift {
    pub name: String,
    pub is_wrapped: bool,
}

pub trait Gift: fmt::Display {
    fn wrap(&mut self);
}

macro_rules! wrap_gift {
    ($($gift:ident),+) => {
        $(
        impl Gift for $gift {
            fn wrap(&mut self) {
                self.is_wrapped = true;
            }
        }
        impl fmt::Display for $gift {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.name)
            }
        }
    )*};
}

// 3. Update the function signature
pub fn prepare_gift<T: Gift>(gift: &mut T) {
    println!("Preparing gift for {}", &gift);
    gift.wrap();
    println!("Gift wrapped for {}", &gift);
}

// 4. Implement the Gift trait to the gift structs
wrap_gift!(KidsGift, ElvesGift, ReindeerGift);

pub fn main() {
    let mut kids_gift = KidsGift {
        name: "toy car".to_string(),
        is_wrapped: false,
    };
    let mut elves_gift = ElvesGift {
        name: "vertical monitor".to_string(),
        is_wrapped: false,
    };
    let mut reindeer_gift = ReindeerGift {
        name: "carrot".to_string(),
        is_wrapped: false,
    };

    prepare_gift(&mut kids_gift);
    prepare_gift(&mut elves_gift);
    prepare_gift(&mut reindeer_gift);
}
