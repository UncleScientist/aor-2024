use std::{error::Error, fmt};

// For a better understanding of the problem, have a look at the end of the file and see the `main`
// function to see how the structs are being used.

pub struct Kid {
    pub name: String,
    pub gifted: bool,
}

pub struct Reindeer {
    pub name: String,
    pub gifted: bool,
}

pub struct Elf {
    pub name: String,
    pub gifted: bool,
}

pub trait Giftable {
    // 1. Define the trait definition
    fn receive_gift(&mut self);
}

// 2. Implement `Giftable` for `Kid`, `Reindeer`, and `Elf`
impl Giftable for Kid {
    fn receive_gift(&mut self) {
        self.gifted = true;
    }
}
impl Giftable for Reindeer {
    fn receive_gift(&mut self) {
        self.gifted = true;
    }
}
impl Giftable for Elf {
    fn receive_gift(&mut self) {
        self.gifted = true;
    }
}

pub trait Gift {
    fn wrap(&mut self);
    // 3. Define a function named `is_wrapped` that returns a boolean
    fn is_wrapped(&self) -> bool;
}

// 4. Update the `Gift` trait implementation for `KidsGift`, `ElvesGift`, and `ReindeerGift` to
//    include the `is_wrapped` function

macro_rules! gift_type {
    ($($gift:ident),+) => {
        $(
            impl Gift for $gift {
                fn wrap(&mut self) { self.is_wrapped = true; }
                fn is_wrapped(&self) -> bool { self.is_wrapped }
            }
        )*};
}

gift_type!(ElvesGift, ReindeerGift, KidsGift);

pub struct Santa;

impl Santa {
    pub fn give_gift<R: Giftable, G: Gift>(
        &self,
        recipient: &mut R,
        gift: &G,
    ) -> Result<(), Box<dyn Error>> {
        // 5. Update the function signature to accept any type of recipient and gift
        if !gift.is_wrapped() {
            Err("unwrapped".into())
        } else {
            recipient.receive_gift();
            Ok(())
        }
    }
}

pub struct KidsGift {
    pub name: String,
    pub is_wrapped: bool,
}

impl fmt::Display for KidsGift {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub struct ElvesGift {
    pub name: String,
    pub is_wrapped: bool,
}

impl fmt::Display for ElvesGift {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub struct ReindeerGift {
    pub name: String,
    pub is_wrapped: bool,
}

impl fmt::Display for ReindeerGift {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub fn prepare_gift<T: Gift + fmt::Display>(gift: &mut T) {
    println!("Preparing gift for {}", &gift);
    gift.wrap();
    println!("Gift wrapped for {}", &gift);
}

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

    let mut alice = Kid {
        name: "Alice".to_string(),
        gifted: false,
    };

    let mut prancer = Reindeer {
        name: "Prancer".to_string(),
        gifted: false,
    };

    let mut bernard = Elf {
        name: "Buddy".to_string(),
        gifted: false,
    };

    let santa = Santa;

    prepare_gift(&mut kids_gift);
    prepare_gift(&mut elves_gift);
    prepare_gift(&mut reindeer_gift);

    if santa.give_gift(&mut alice, &kids_gift).is_ok() {
        println!("{} received {}", alice.name, kids_gift);
        assert!(alice.gifted);
    } else {
        panic!("{} should have received {}", alice.name, kids_gift);
    }

    if santa.give_gift(&mut prancer, &reindeer_gift).is_ok() {
        println!("{} received {}", prancer.name, reindeer_gift);
        assert!(prancer.gifted);
    } else {
        panic!("{} should have received {}", prancer.name, reindeer_gift);
    }

    if santa.give_gift(&mut bernard, &elves_gift).is_ok() {
        println!("{} received {}", bernard.name, elves_gift);
        assert!(bernard.gifted);
    } else {
        panic!("{} should have received {}", bernard.name, elves_gift);
    }
}
