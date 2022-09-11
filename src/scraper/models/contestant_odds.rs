use super::odds::Odds;

pub struct ContestantOdds {
  contestant: String,
  backing_odds: Odds,
  laying_odds: Odds,
}
