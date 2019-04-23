use std::fmt;

pub struct Ascii {
    min: f64,
    max: f64,
    data: Vec<f64>,
    step: usize,
}

impl Ascii {
    pub fn new(min: f64, max: f64, data: Vec<f64>, step: usize) -> Self {
        Self {
            min,
            max,
            data,
            step,
        }
    }
}

impl fmt::Display for Ascii {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ticks = vec!['▁', '▂', '▃', '▄', '▅', '▆', '▇'];
        let ticks_len = (ticks.len() - 1) as f64;
        let sparkline: String = self
            .data
            .iter()
            .step_by(self.step)
            .map(|x| {
                let mut i = ticks_len * (x - self.min);
                if i > 0. {
                    i = (i / self.max).round();
                }
                ticks[i as usize]
            })
            .collect();
        write!(f, "{}", sparkline)
    }
}

#[test]
fn test_display() {
    let s = Ascii {
        min: 0.,
        max: 1.,
        data: vec![0.],
        step: 2,
    };
    assert_eq!(format!("{}", s), "▁");
    let s = Ascii {
        min: 0.,
        max: 1.,
        data: vec![0., 10.],
        step: 2,
    };
    assert_eq!(format!("{}", s), "▁");

    let s = Ascii {
        min: 0.,
        max: 1.,
        data: vec![0., 0., 1.],
        step: 2,
    };
    assert_eq!(format!("{}", s), "▁▇");
}
