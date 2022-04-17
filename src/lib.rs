#![warn(clippy::pedantic)]

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Wins(Vec<Count>);

impl Wins {
    pub fn extend(&mut self, other: &Wins) {
        self.0.extend(other.get())
    }

    pub fn push(&mut self, count: Count) {
        self.0.push(count);
    }

    #[must_use]
    pub fn get(&self) -> &Vec<Count> {
        &self.0
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl From<Vec<Count>> for Wins {
    fn from(counts: Vec<Count>) -> Self {
        Wins(counts)
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests__wins {
    use super::*;

    #[test]
    fn extend() {
        let mut wins = Wins::default();
        let more_wins = Wins::from(vec![Win::FIRST, Win::FIRST, Win::SECOND]);
        let even_more_wins = Wins::from(vec![Win::FIRST, Win::SECOND, Win::SECOND]);

        wins.extend(&more_wins);
        wins.extend(&even_more_wins);

        assert!(!wins.is_empty());
        assert_eq!(more_wins.len() + even_more_wins.len(), wins.len());
    }

    #[test]
    fn push() {
        let mut counter = Wins::default();

        counter.push(Win::FIRST);
        counter.push(Win::SECOND);
        counter.push(Win::FIRST);
        counter.push(Win::SECOND);
        counter.push(Win::FIRST & Win::SECOND);

        assert_eq!(5, counter.get().len())
    }

    #[test]
    fn is_empty() {
        let mut counter = Wins::default();

        counter.push(Win::FIRST);

        assert!(!counter.is_empty());
        assert!(Wins::default().is_empty());
    }

    #[test]
    fn len() {
        let mut counter = Wins::default();

        counter.push(Win::FIRST);
        counter.push(Win::FIRST);
        counter.push(Win::FIRST);
        counter.push(Win::FIRST);

        assert_eq!(4, counter.len());
        assert_eq!(0, Wins::default().len());
    }
}

pub type Count = u8;

pub struct Win;

impl Win {
    pub const FIRST: Count = 0b0000_0001;
    pub const SECOND: Count = 0b0000_0010;
    pub const THIRD: Count = 0b0000_0100;
    pub const FORTH: Count = 0b0000_1000;
    pub const FIFTH: Count = 0b0001_0000;
    pub const SIXTH: Count = 0b0010_0000;
    pub const SEVENTH: Count = 0b0100_0000;
    pub const EIGHT: Count = 0b1000_0000;
}
