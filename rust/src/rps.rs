use std::str::FromStr;
use std::cmp::{
    Ordering,
    PartialOrd,
};
use rand::Rng;
use rand::distributions::{
    Standard,
    Distribution,
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl PartialOrd for RPS {
    fn partial_cmp(&self, rhs: &RPS) -> Option<Ordering> {
        use RPS::*;

        if self == rhs {
            return Some(Ordering::Equal);
        }

        if *self == Rock && *rhs == Paper {
            return Some(Ordering::Less);
        }

        if *self == Rock && *rhs == Scissors {
            return Some(Ordering::Greater);
        }

        if *self == Paper && *rhs == Scissors {
            return Some(Ordering::Less);
        }

        return rhs.partial_cmp(self);
    }
}

impl Distribution<RPS> for Standard {
    fn sample<R: Rng + ?Sized>(&self, r: &mut R) -> RPS {
        match r.gen_range(0..=2) {
            0 => RPS::Rock,
            1 => RPS::Paper,
            2 => RPS::Scissors,
            _ => unreachable!(),
        }
    }
}

impl FromStr for RPS {
    type Err = ();

    fn from_str(s: &str) -> Result<RPS, ()> {
        match s.trim().to_lowercase().as_str() {
            "rock"     => Ok(RPS::Rock),
            "paper"    => Ok(RPS::Paper),
            "scissors" => Ok(RPS::Scissors),
            _          => Err(())
        }
    }
}

