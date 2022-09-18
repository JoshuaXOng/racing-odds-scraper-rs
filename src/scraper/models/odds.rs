use super::money::Money;

#[derive(Debug)]
pub enum Odds {
  OddsWoLiquidity(f64),
  OddsWLiquidity(f64, Money),
}
