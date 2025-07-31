struct Puncturer {
    coefficients: Vec<i32>,
    mother_rate: i32,
    t: i32,
    period: i32,
    numerator: i32,
    denominator: i32,
    i: Box<dyn Fn(i32) -> i32>
}

enum PredefinedPuncturer {
    Rate2Over3Puncturer,
    Rate1Over3Puncturer,
    Rate292Over432Puncturer,
    Rate148Over432Puncturer,
}

impl Puncturer {
    fn build(predefined_puncturer: PredefinedPuncturer) -> Puncturer {
        match predefined_puncturer {
            PredefinedPuncturer::Rate2Over3Puncturer => Puncturer {
                coefficients: vec![0, 1, 2, 5],
                mother_rate: 4,
                t: 3,
                period: 8,
                numerator: 2,
                denominator: 3,
                i: Box::new(|j| j),
            },
            PredefinedPuncturer::Rate1Over3Puncturer => Puncturer {
                coefficients: vec![0, 1, 2, 3, 5, 6, 7],
                mother_rate: 4,
                t: 3,
                period: 8,
                numerator: 1,
                denominator: 3,
                i: Box::new(|j| j),
            },
            PredefinedPuncturer::Rate292Over432Puncturer => Puncturer {
                coefficients: vec![0, 1, 2, 5],
                mother_rate: 4,
                t: 3,
                period: 8,
                numerator: 292,
                denominator: 432,
                i: Box::new(|j| j + ((j - 1) / 65)),
            },
            PredefinedPuncturer::Rate148Over432Puncturer => Puncturer {
                coefficients: vec![0, 1, 2, 3, 5, 6, 7],
                mother_rate: 4,
                t: 6,
                period: 8,
                numerator: 148,
                denominator: 432,
                i: Box::new(|j| j + ((j - 1) / 35)),
            },
        }
    }
}
