mod fraction;
pub use crate::fraction::Frac;

#[cfg(test)]
mod tests;

#[cfg(feature= "bonus_tests")]
#[cfg(test)]
mod tests_bonus;
