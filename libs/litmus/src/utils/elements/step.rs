use std::borrow::Cow;

pub struct Step<'d, F> {
    pub label: StepLabel,
    pub description: Cow<'d, str>,
    pub body: F,
}

pub enum StepLabel {
    Given,
    When,
    Then,
    And,
    But,
}
