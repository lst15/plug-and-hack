// src/pipeline/mod.rs
pub mod collector;
pub mod correlator;
pub mod modeler;
pub mod executor;

pub use collector::Collector;
pub use correlator::Correlator;
pub use modeler::Modeler;
pub use executor::Executor;