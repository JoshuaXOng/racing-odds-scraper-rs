pub use super::odds::Odds;

#[derive(Debug)]
pub struct ContestantOdds {
  pub contestant: String,
  pub backing_odds: Vec<Odds>,
  pub laying_odds: Vec<Odds>,
}
