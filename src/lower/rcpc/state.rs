use std::ops::Index;

pub(crate) struct State(u8);

impl State {

    pub fn new() -> Self {
        Self(0)
    }

    pub fn val(&self) -> u8 {
        self.0
    }

    pub fn shift_in(&mut self, bit: bool) {
        self.0 <<= 1;
        self.0 |= bit as u8;
        self.0 &= 0b1111;
    }

    pub fn set(&mut self, value: u8) {
        self.0 = value;
    }

    pub fn get(&self) -> [bool; 4] {
        [
            self.0 & 0b0001 != 0,
            self.0 & 0b0010 != 0,
            self.0 & 0b0100 != 0,
            self.0 & 0b1000 != 0
        ]
    }

    pub fn reset(&mut self) {
        self.0 = 0;
    }

}

impl Index<usize> for State {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        const TRUE: bool = true;
        const FALSE: bool = false;

        assert!(index < 4, "index out of bounds for 4-bit register");

        if (self.0 >> index) & 1 == 1 {
            &TRUE
        } else {
            &FALSE
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn shifts_correctly() {

        let mut state = State::new();

        // Starts at 0000
        assert_eq!(state.get(), [false, false, false, false]);
        state.shift_in(true);
        assert_eq!(state.get(), [true, false, false, false]);
        assert_eq!(state[0], true);
        state.shift_in(false);
        assert_eq!(state.get(), [false, true, false, false]);
        assert_eq!(state[0], false);

        state.reset();
        assert_eq!(state.get(), [false, false, false, false]);
    }

}