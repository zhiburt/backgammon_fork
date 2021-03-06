use super::{CurrentRules, Match, Rules, Statistics};

use std::time::{Duration, SystemTimeError};
use uuid::Uuid;

impl Default for Match {
    fn default() -> Self {
        Match {
            id: Uuid::new_v4(),
            points: 3,
            rules: CurrentRules::default(),
            games: Vec::new(),
            statistics: Statistics::default(),
        }
    }
}

/// Implements methods for the Match struct
impl Match {
    /// Start a new match:
    /// ```
    /// use backgammon::Match;
    ///
    /// let m = Match::new();
    ///
    /// # use backgammon::Rules;
    /// # assert!(&m.is_crawford(),true);
    /// ```
    pub fn new() -> Self {
        Match::default()
    }

    /// Define the points required to win the match
    pub fn with_points(mut self, p: u32) -> Self {
        self.points = p;
        self
    }

    /// How many points are required to win the match
    pub fn get_points(&self) -> u32 {
        self.points
    }

    /// Start the match
    /// 1. dices are rolled for the first time
    /// 2. A first game is stored
    /// 3. Whoever may start, may do so
    pub fn start(mut self) -> Self {
        self.statistics = Statistics::default();

        self
    }

    /// Stop the match
    /// 1. Winner is assigned based on current points
    pub fn stop(mut self) -> Self {
        // TODO: the winner needs to be declared, etc.
        self.statistics.stop();
        self
    }

    /// Duration of this match
    pub fn duration(&self) -> Result<Duration, SystemTimeError> {
        self.statistics.duration()
    }
}

impl Rules for Match {
    fn with_beaver(self) -> Self {
        Match {
            rules: self.rules.with_beaver(),
            ..self
        }
    }
    fn is_beaver(&self) -> bool {
        self.rules.is_beaver()
    }

    fn with_raccoon(self) -> Self {
        Match {
            rules: self.rules.with_raccoon(),
            ..self
        }
    }
    fn is_raccoon(&self) -> bool {
        self.rules.is_raccoon()
    }

    fn with_murphy(self, limit: u8) -> Self {
        Match {
            rules: self.rules.with_murphy(limit),
            ..self
        }
    }
    fn is_murphy(&self) -> bool {
        self.rules.is_murphy()
    }

    fn with_jacoby(self) -> Self {
        Match {
            rules: self.rules.with_jacoby(),
            ..self
        }
    }
    fn is_jacoby(&self) -> bool {
        self.rules.is_jacoby()
    }

    fn with_crawford(self) -> Self {
        Match {
            rules: self.rules.with_crawford(),
            ..self
        }
    }
    fn is_crawford(&self) -> bool {
        self.rules.is_crawford()
    }

    fn with_holland(self) -> Self {
        Match {
            rules: self.rules.with_holland(),
            ..self
        }
    }
    fn is_holland(&self) -> bool {
        self.rules.is_holland()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn rule_test_murphy() {
        let m = Match::new().with_murphy(3);

        assert!(&m.is_murphy());
    }

    #[test]
    fn rule_test_raccoon() {
        let m = Match::new().with_raccoon();

        assert!(&m.is_raccoon());
    }

    #[test]
    fn rule_test_jacoby() {
        let m = Match::new().with_jacoby();

        assert!(&m.is_jacoby());
    }

    #[test]
    fn rule_test_crawford() {
        let m = Match::new().with_crawford();

        assert!(&m.is_crawford());
    }

    #[test]
    fn rule_test_holland() {
        let m = Match::new().with_holland();

        assert!(&m.is_holland());
    }

    #[test]
    fn rule_test_beaver() {
        let m = Match::new().with_beaver();

        assert!(&m.is_beaver());
    }

    #[test]
    fn match_test_points() {
        let m = Match::new().with_points(13).with_holland();

        assert!(m.get_points() == 13);
    }

    #[test]
    fn match_test_points1() {
        let m = Match::new().with_holland().with_points(13).with_beaver();

        assert!(m.get_points() == 13);
    }
}
