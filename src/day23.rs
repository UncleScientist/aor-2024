use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct SantaList {
    records: HashMap<String, bool>,
}

impl SantaList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add<S: AsRef<str>>(&mut self, child: S, behavior: bool) {
        self.records.insert(child.as_ref().into(), behavior);
    }

    pub fn remove<S: AsRef<str>>(&mut self, child: S) {
        self.records.remove(child.as_ref());
    }

    pub fn get<S: AsRef<str>>(&self, child: S) -> Option<bool> {
        self.records.get(child.as_ref()).copied()
    }

    pub fn count(&self) -> (usize, usize) {
        let total = self.records.len();
        // let nice_count = self.records.iter().filter(|(_, b)| **b).count();
        let nice_count = self
            .records
            .iter()
            .fold(0usize, |sum, next| sum + *next.1 as usize);
        (nice_count, total - nice_count)

        /*
        let (nice, naughty): (Vec<_>, Vec<_>) =
            self.records.iter().partition(|(_, behavior)| **behavior);
        (nice.len(), naughty.len())
        */
    }

    pub fn list_by_behavior(&self, behavior: bool) -> Vec<String> {
        self.records
            .iter()
            .filter(|rec| *rec.1 == behavior)
            .map(|rec| rec.0.clone())
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut santa_list = SantaList::new();

        santa_list.add("Alice", true);
        santa_list.add("Bob", false);
        santa_list.add("Charlie", true);

        if let Some(behavior) = santa_list.get("Alice") {
            println!(
                "Alice is on the {} list.",
                if behavior { "Nice" } else { "Naughty" }
            );
        }

        let (nice, naughty) = santa_list.count();
        println!("Santa's list contains {nice} nice and {naughty} naughty children.",);

        let nice_list = santa_list.list_by_behavior(true);
        println!("Nice children: {nice_list:?}");

        let naughty_list = santa_list.list_by_behavior(false);
        println!("Naughty children: {naughty_list:?}");

        santa_list.remove("Bob");
        let (nice, naughty) = santa_list.count();
        println!(
            "After removing Bob, Santa's list contains {nice} nice and {naughty} naughty children.",
        );
        // assert!(false);
    }
}
