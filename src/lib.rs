#![warn(clippy::pedantic)]

pub type WinCount = u8;

struct Flag;

impl Flag {
    const FIRST: u8 = 0b0000_0001;
    const SECOND: u8 = 0b0000_0010;
    const THIRD: u8 = 0b0000_0100;
    const FORTH: u8 = 0b0000_1000;
    const FIFTH: u8 = 0b0001_0000;
    const SIXTH: u8 = 0b0010_0000;
    const SEVENTH: u8 = 0b0100_0000;
    const EIGHT: u8 = 0b1000_0000;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
