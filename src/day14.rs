use std::fmt::Display;

pub struct KidsGift {
    pub name: String,
}

pub struct ElvesGift {
    pub name: String,
}

pub struct ReindeerGift {
    pub name: String,
}

macro_rules! impl_display {
    ($($gift:ident),+) => {
        $(impl Display for $gift {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.name)
            }
    })*};
}

impl_display!(KidsGift, ElvesGift, ReindeerGift);

pub fn display_gift<D: Display>(gift: D) {
    println!("{gift}");
}

pub fn main() {
    let kids_gift = KidsGift {
        name: "toy car".to_string(),
    };
    let elves_gift = ElvesGift {
        name: "vertical monitor".to_string(),
    };
    let reindeer_gift = ReindeerGift {
        name: "carrot".to_string(),
    };

    display_gift(&kids_gift);
    display_gift(&elves_gift);
    display_gift(&reindeer_gift);
}
