use super::RPS;

pub enum EndState {
    Win,
    Loss,
    Tie,
}

impl EndState {
    pub fn from(a: RPS, b: RPS) -> Self {
        if a == b {
            return Self::Tie;
        }

        if a > b {
            return Self::Win;
        }

        if a < b {
            return Self::Loss;
        }

        unreachable!();
    }
}

