// Let's abstract an alloy

use rand::distributions::Weighted;

#[derive(Debug)]
pub struct Alloy {
    items: Vec<Weighted<String>>,
}

impl Alloy {
    pub fn new(kinds: Vec<&str>, ratios: Vec<u32>) -> Self {
        let items: Vec<_> = kinds
            .into_iter()
            .zip(ratios.into_iter())
            .map(|(item, weight)| Weighted {
                weight,
                item: item.to_owned(),
            })
            .collect();
        Self { items }
    }

    pub fn choices(self) -> Vec<Weighted<String>> {
        self.items
    }
}
