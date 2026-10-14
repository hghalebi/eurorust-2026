use std::{fmt, num::NonZeroU32};

#[derive(Debug, thiserror::Error)]
#[error("attempt budget must be positive")]
pub struct InvalidBudget;
pub struct BudgetLimit(NonZeroU32);
impl TryFrom<u32> for BudgetLimit {
    type Error = InvalidBudget;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        NonZeroU32::new(value).map(Self).ok_or(InvalidBudget)
    }
}
pub struct AttemptsUsed(u32);
impl fmt::Display for AttemptsUsed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
pub struct AttemptOrdinal(NonZeroU32);
impl fmt::Display for AttemptOrdinal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// One local attempt token is issued by a budget and consumed by a work call.
/// ```
/// use agent_contract_lab::retry::*;
/// let mut budget=AttemptBudget::new(BudgetLimit::try_from(1).unwrap());
/// let token=budget.issue().unwrap();
/// let mut work=SimulatedWork::new(RemainingFailures::from(0));
/// assert!(work.perform(token).is_ok());
/// assert!(budget.issue().is_none());
/// ```
/// ```compile_fail
/// use agent_contract_lab::retry::*;
/// let mut budget=AttemptBudget::new(BudgetLimit::try_from(1).unwrap());
/// let token=budget.issue().unwrap();
/// let mut work=SimulatedWork::new(RemainingFailures::from(0));
/// let first=work.perform(token);
/// let duplicate=work.perform(token);
/// ```
pub struct AttemptToken {
    ordinal: AttemptOrdinal,
}
pub struct AttemptBudget {
    limit: BudgetLimit,
    used: AttemptsUsed,
}
impl AttemptBudget {
    pub fn new(limit: BudgetLimit) -> Self {
        Self {
            limit,
            used: AttemptsUsed(0),
        }
    }
    pub fn issue(&mut self) -> Option<AttemptToken> {
        if self.used.0 >= self.limit.0.get() {
            return None;
        }
        self.used.0 += 1;
        Some(AttemptToken {
            ordinal: AttemptOrdinal(NonZeroU32::new(self.used.0).expect("issued count positive")),
        })
    }
}
pub struct WorkReceipt {
    ordinal: AttemptOrdinal,
}
impl fmt::Display for WorkReceipt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "completed on attempt {}", self.ordinal)
    }
}
#[derive(Debug, thiserror::Error)]
#[error("simulated transient failure before an external effect")]
pub struct WorkFailure;

/// Implementations perform one modeled attempt. This is not authentication or
/// durable deduplication; behavior beyond the signature remains a runtime contract.
pub trait Work {
    fn perform(&mut self, token: AttemptToken) -> Result<WorkReceipt, WorkFailure>;
}
pub struct RemainingFailures(u32);
impl From<u32> for RemainingFailures {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
pub struct SimulatedWork {
    remaining: RemainingFailures,
}
impl SimulatedWork {
    pub fn new(remaining: RemainingFailures) -> Self {
        Self { remaining }
    }
}
impl Work for SimulatedWork {
    fn perform(&mut self, token: AttemptToken) -> Result<WorkReceipt, WorkFailure> {
        if self.remaining.0 > 0 {
            self.remaining.0 -= 1;
            return Err(WorkFailure);
        }
        Ok(WorkReceipt {
            ordinal: token.ordinal,
        })
    }
}
pub enum Outcome {
    Completed(WorkReceipt),
    Exhausted(AttemptsUsed),
}
pub fn run<W: Work>(work: &mut W, mut budget: AttemptBudget) -> Outcome {
    while let Some(token) = budget.issue() {
        if let Ok(receipt) = work.perform(token) {
            return Outcome::Completed(receipt);
        }
    }
    Outcome::Exhausted(budget.used)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn success_after_failures_and_exhaustion_are_observable() {
        let mut work = SimulatedWork::new(RemainingFailures::from(2));
        match run(
            &mut work,
            AttemptBudget::new(BudgetLimit::try_from(3).unwrap()),
        ) {
            Outcome::Completed(receipt) => {
                assert_eq!(receipt.to_string(), "completed on attempt 3")
            }
            _ => panic!("expected recovery"),
        }
        let mut work = SimulatedWork::new(RemainingFailures::from(2));
        match run(
            &mut work,
            AttemptBudget::new(BudgetLimit::try_from(2).unwrap()),
        ) {
            Outcome::Exhausted(used) => assert_eq!(used.to_string(), "2"),
            _ => panic!("expected exhaustion"),
        }
        assert!(BudgetLimit::try_from(0).is_err());
    }
}
