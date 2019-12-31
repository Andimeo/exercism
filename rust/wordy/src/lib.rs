extern crate regex;

use regex::{Captures, Regex, RegexSet};

pub struct WordProblem {
    regex_set: RegexSet,
}

impl WordProblem {
    fn new() -> Self {
        WordProblem {
            regex_set: RegexSet::new(&[
                r"What is (-?\d)+\?", 
                r"What is (-?\d+) plus (-?\d+)\?",
                r"What is (-?\d+) minus (-?\d+)\?",
                r"What is (-?\d+) multiplied by (-?\d+)\?",
                r"What is (-?\d+) divided by (-?\d+)\?",
                r"What is (-?\d+) plus (-?\d+) plus (-?\d+)\?",
                r"What is (-?\d+) plus (-?\d+) multiplied by (-?\d+)\?",
                r"What is (-?\d+) raised to the (-?\d+)th power?",
                r"What is (-?\d+) plus (-?\d+) minus (-?\d+)\?",
                r"What is (-?\d+) minus (-?\d+) plus (-?\d+)\?",
                r"What is (-?\d+) minus (-?\d+) minus (-?\d+)\?",
                r"What is (-?\d+) multiplied by (-?\d+) multiplied by (-?\d+)\?",
                r"What is (-?\d+) divided by (-?\d+) divided by (-?\d+)?",
                r"What is (-?\d+) plus (-?\d+) raised to the (-?\d+)(th|nd|st) power\?",
                ]).unwrap(),
        }
    }

    fn func(&self, idx: usize, cap: Captures) -> i32 {
        let p = |s: &str| { s.parse::<i32>().unwrap() };
        match idx {
            0 => p(&cap[1]),
            1 => p(&cap[1]) + p(&cap[2]),
            2 => p(&cap[1]) - p(&cap[2]),
            3 => p(&cap[1]) * p(&cap[2]),
            4 => p(&cap[1]) / p(&cap[2]),
            5 => p(&cap[1]) + p(&cap[2]) + p(&cap[3]),
            6 => (p(&cap[1]) + p(&cap[2])) * p(&cap[3]),
            7 => p(&cap[1]).pow(p(&cap[2]) as u32),
            8 => p(&cap[1]) + p(&cap[2]) - p(&cap[3]),
            9 => p(&cap[1]) - p(&cap[2]) + p(&cap[3]),
            10 => p(&cap[1]) - p(&cap[2]) - p(&cap[3]),
            11 => p(&cap[1]) * p(&cap[2]) * p(&cap[3]),
            12 => p(&cap[1]) / p(&cap[2]) / p(&cap[3]),
            13 => (p(&cap[1]) + p(&cap[2])).pow(p(&cap[3]) as u32),
            _ => panic!()
        }
    }

    fn calc(&self, command: &str) -> Option<i32> {
        let patterns: Vec<_> = self
            .regex_set
            .matches(command)
            .into_iter()
            .map(|idx| (idx, &self.regex_set.patterns()[idx]))
            .collect();
        println!("{:?}", patterns);
        assert!(patterns.len() <= 1);
        if patterns.is_empty() {
            None
        } else {
            let idx = patterns.get(0).unwrap().0;
            let pattern = patterns.get(0).unwrap().1;
            let re = Regex::new(pattern).unwrap();
            re.captures_iter(command).map(|cap| self.func(idx, cap)).next()
        }
    }
}

pub fn answer(command: &str) -> Option<i32> {
    let wp = WordProblem::new();
    wp.calc(command)
}
