use std::borrow::Cow;

pub struct Step<'s, Body> {
    pub label: StepLabel,
    pub description: Cow<'s, str>,
    pub body: Body,
}

impl<'s, Body> std::fmt::Display for Step<'s, Body> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{} {}", self.label, self.description)
    }
}

#[derive(strum::Display)]
pub enum StepLabel {
    Given,
    When,
    Then,
    And,
    But,
}
