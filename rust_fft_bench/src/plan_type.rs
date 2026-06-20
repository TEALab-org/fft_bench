use clap::ValueEnum;
use fftw::types::Flag;
use serde::{Deserialize, Serialize};

/// FFTW3 Provides several strategies for plan creation,
/// we expose three of them.
#[derive(Copy, Clone, Debug, ValueEnum, Default, Deserialize, Serialize)]
pub enum PlanType {
    /// Create optimziated plan
    Measure,

    /// Create optimized plan with more exhaustive search than Measaure
    Patient,

    /// Create an un-optimal plan quickly
    #[default]
    Estimate,

    /// Create plan only based on loaded wisdom
    WisdomOnly,
}

impl PlanType {
    pub fn to_fftw3_flag(&self) -> Flag {
        match self {
            PlanType::Measure => Flag::MEASURE,
            PlanType::Patient => Flag::PATIENT,
            PlanType::Estimate => Flag::ESTIMATE,
            PlanType::WisdomOnly => Flag::WISDOWMONLY,
        }
    }
}
