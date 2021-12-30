use ming_xu::possible_paths;
use std::ops::Add;

#[derive(Clone, Hash, Eq, PartialEq)]
struct Situation {
    pos: Vec<i8>,
    sco: Vec<i8>,
    trn: usize
}

#[derive(Clone, Default)]
struct Score (u64, u64);

impl Add for Score {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Score(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl Score {
    fn one_at_first_index(b: bool) -> Self {
        if b {
            Self(1, 0)
        } else {
            Self(0, 1)
        }
    }

    fn max(&self) -> u64 {
        self.0.max(self.1)
    }
}

pub fn run() -> u64 {
    possible_paths::get(
        Situation {
            pos: vec![ 6, 10 ],
            sco: vec![ 0,  0 ],
            trn: 0
        }, &| s: &Situation | (s.sco[1 - s.trn] >= 21).then_some(Score::one_at_first_index(1 != s.trn)),
        &| s: &Situation | {
            let mut successors = Vec::new();
            for a in 1..=3 {
                for b in 1..=3 {
                    for c in 1..=3 {
                        let mut new_situation = s.clone();
                        new_situation.pos[new_situation.trn] = (new_situation.pos[new_situation.trn] - 1 + a + b + c) % 10 + 1;
                        new_situation.sco[new_situation.trn] += new_situation.pos[new_situation.trn];
                        new_situation.trn = 1 - new_situation.trn;
                        successors.push(new_situation);
                    }
                }
            }
            successors
        }
    ).max()
}