pub struct Puncturer {
    pub coefficients: Vec<usize>,
    pub t: usize,
    pub period: usize,
    pub numerator: i32,
    pub denominator: i32,
    pub i: Box<dyn Fn(usize) -> usize>
}

pub enum PredefinedPuncturer {
    Rate2Over3Puncturer,
    Rate1Over3Puncturer,
    Rate292Over432Puncturer,
    Rate148Over432Puncturer,
}

impl Puncturer {
    pub fn build(predefined_puncturer: &PredefinedPuncturer) -> Puncturer {
        match predefined_puncturer {
            PredefinedPuncturer::Rate2Over3Puncturer => Puncturer {
                coefficients: vec![0, 1, 2, 5],
                t: 3,
                period: 8,
                numerator: 2,
                denominator: 3,
                i: Box::new(|j| j),
            },
            PredefinedPuncturer::Rate1Over3Puncturer => Puncturer {
                coefficients: vec![0, 1, 2, 3, 5, 6, 7],
                t: 6,
                period: 8,
                numerator: 1,
                denominator: 3,
                i: Box::new(|j| j),
            },
            PredefinedPuncturer::Rate292Over432Puncturer => Puncturer {
                coefficients: vec![0, 1, 2, 5],
                t: 3,
                period: 8,
                numerator: 292,
                denominator: 432,
                i: Box::new(|j| j + ((j - 1) / 65)),
            },
            PredefinedPuncturer::Rate148Over432Puncturer => Puncturer {
                coefficients: vec![0, 1, 2, 3, 5, 6, 7],
                t: 6,
                period: 8,
                numerator: 148,
                denominator: 432,
                i: Box::new(|j| j + ((j - 1) / 35)),
            },
        }
    }
}
