use num_complex::{Complex, Complex32};

const NUMBER_OF_SYMBOLS: usize = 4;

const NUMBER_OF_PHASES: usize = 8;

/// The change in phase (in multiples of π/4) for each bit pair (00, 01, 10, 11).
const SYMBOL_PHASE_DIFFS: [i32; NUMBER_OF_SYMBOLS] = [
    1,  // 00
    3,  // 01
    -1, // 10
    -3  // 11
];

/// Mapping of the possible absolute phases (multiples of π/4) to their I/Q values
const ABSOLUTE_PHASES: [Complex<f32>; NUMBER_OF_PHASES] = [
    Complex::new(1.0, 0.0),
    Complex::new(0.707, 0.707),
    Complex::new(0.0, 1.0),
    Complex::new(-0.707, 0.707),
    Complex::new(-1.0, 0.0),
    Complex::new(-0.707, -0.707),
    Complex::new(0.0, -1.0),
    Complex::new(0.707, -0.707),
];

struct Modulator {
    /// The current phase of the modulator, in multiples of π/4
    phase: u32
}

impl Modulator {

    pub fn new() -> Self {
        Self {
            phase: 0
        }
    }

    /// Modulate a bit-pair (value 0 to 3) into a complex I/Q value
    pub fn next(&mut self, symbol: u32) -> Complex32 {

        if symbol > 3 {
            panic!("Invalid symbol value: {}. Must be between 0 and 3.", symbol);
        }

        // Compute the new phase by adding the symbol's phase change to the current phase
        // Note that this may wrap in either direction
        let new_phase = self.phase as i32 + SYMBOL_PHASE_DIFFS[symbol as usize];

        // Update the modulator's phase, accounting for wrapping around the number of phases
        self.phase = ((new_phase % NUMBER_OF_PHASES as i32) + NUMBER_OF_PHASES as i32) as u32 % NUMBER_OF_PHASES as u32;

        // Return the corresponding I/Q value for the new phase
        ABSOLUTE_PHASES[self.phase as usize]
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modulates_symbols_correctly() {

        let mut modulator = Modulator::new();
        let mut result = modulator.next(0);
        assert_eq!(modulator.phase, 1);
        assert_eq!(result, Complex::new(0.707, 0.707));
        
        result = modulator.next(0);
        assert_eq!(modulator.phase, 2);
        assert_eq!(result, Complex::new(0.0, 1.0));
        
        result = modulator.next(1);
        assert_eq!(modulator.phase, 5);
        assert_eq!(result, Complex::new(-0.707, -0.707));
    }
}