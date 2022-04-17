#![warn(clippy::pedantic)]

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Wins(Vec<Count>);

impl Wins {
    pub fn add_win(&mut self, count: Count) {
        self.0.push(count);
    }

    pub fn add_win_first(&mut self) {
        self.0.push(Win::FIRST);
    }

    pub fn add_win_second(&mut self) {
        self.0.push(Win::SECOND);
    }

    pub fn add_win_third(&mut self) {
        self.0.push(Win::THIRD);
    }

    pub fn extend(&mut self, other: &Wins) {
        self.0.extend(other.get());
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

    #[must_use]
    pub fn wins_for(&self, result: Count) -> (usize, usize) {
        let wins: Vec<Count> = self
            .0
            .clone()
            .into_iter()
            .filter(|r| r.win_for(result))
            .collect();
        (wins.len(), wins.into_iter().filter(Result::is_tie).count())
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
    fn get() {
        let v = vec![Win::FIRST, Win::FIRST, Win::SECOND, Win::FIRST];

        let wins = Wins::from(v.clone());

        assert_eq!(&v, wins.get())
    }

    #[test]
    fn add_win() {
        let mut counter = Wins::default();

        counter.add_win_first();
        counter.add_win_second();
        counter.add_win_first();
        counter.add_win_third();
        counter.add_win(Win::FIRST | Win::SECOND);
        counter.add_win(Win::FIFTH);

        assert_eq!(6, counter.len())
    }

    #[test]
    fn is_empty() {
        let mut counter = Wins::default();

        counter.add_win(Win::FIRST);

        assert!(!counter.is_empty());
        assert!(Wins::default().is_empty());
    }

    #[test]
    fn len() {
        let mut counter = Wins::default();

        counter.add_win(Win::FIRST);
        counter.add_win(Win::FIRST);
        counter.add_win(Win::FIRST);
        counter.add_win(Win::FIRST);

        assert_eq!(4, counter.len());
        assert_eq!(0, Wins::default().len());
    }

    #[test]
    fn wins_for() {
        let mut counter = Wins::default();

        counter.add_win_first();
        counter.add_win(Win::FIRST | Win::SECOND);
        counter.add_win_third();
        counter.add_win_third();
        counter.add_win_third();
        counter.add_win(Win::FORTH);

        assert_eq!((2, 1), counter.wins_for(Win::FIRST));
        assert_eq!((1, 1), counter.wins_for(Win::SECOND));
        assert_eq!((3, 0), counter.wins_for(Win::THIRD));
        assert_eq!((1, 0), counter.wins_for(Win::FORTH));
    }
}

pub type Count = u8;

pub trait Result {
    #[must_use]
    fn is_tie(&self) -> bool;

    #[must_use]
    fn win_for(&self, count: Count) -> bool;
}

impl Result for Count {
    fn is_tie(&self) -> bool {
        self.count_ones() > 1
    }

    fn win_for(&self, count: Count) -> bool {
        self & count == count
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests__result {
    use super::*;

    #[test]
    fn is_tie() {
        let r = Win::FIRST | Win::SECOND;

        assert_eq!(2, r.count_ones());
        assert!(r.is_tie());
    }

    #[test]
    fn win_for() {
        let tie = Win::FIRST | Win::THIRD;

        assert!(Win::FIRST.win_for(Win::FIRST));
        assert!(tie.win_for(Win::FIRST));
        assert!(tie.win_for(Win::THIRD));
        assert!(!tie.win_for(Win::SECOND));
        assert!(!Win::FIRST.win_for(Win::SECOND));
        assert!(!Win::FIRST.win_for(Win::THIRD));
    }
}

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
