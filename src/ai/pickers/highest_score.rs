use big_brain::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct HighestScore;

impl HighestScore {
    pub fn new() -> Self {
        Self
    }
}

impl Picker for HighestScore {
    fn pick<'a>(
        &self,
        choices: &'a [big_brain::choices::Choice],
        scores: &bevy::prelude::Query<&Score>,
    ) -> Option<&'a big_brain::choices::Choice> {
        choices.iter().max_by(|a, b| {
            let sa = a.calculate(scores);
            let sb = b.calculate(scores);
            sa.partial_cmp(&sb).unwrap()
        })
    }
}
