use super::money::Money;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Odds {
    OddsWoLiquidity(f64),
    OddsWLiquidity(f64, Money),
}

impl TryFrom<(&str, &str)> for Odds {
    type Error = ();

    fn try_from((odds, money): (&str, &str)) -> Result<Self, Self::Error> {
        if let (Ok(odds), Ok(money)) = (odds.parse::<f64>(), money.replace('$', "").parse::<f64>())
        {
            Ok(Self::OddsWLiquidity(odds, Money::Aus(money)))
        } else {
            Err(())
        }
    }
}
