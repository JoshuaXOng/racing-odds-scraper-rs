use super::money::Money;

pub enum Odds {
  OddsWoLiquidity(f64),
  OddsWLiquidity(f64, Money),
}
