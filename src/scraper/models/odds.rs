use super::money::Money;

pub enum Odds {
  OddsWoLiquidity,
  OddsWLiquidity,
}

pub struct OddsWoLiquidity(f64);

pub struct OddsWLiquidity(OddsWoLiquidity, Money);
