//! Minimal og generisk skisse for en spørringsmotor.

pub trait QueryEngine<Q, R> {
    fn execute(&self, query: &Q) -> R;
}
