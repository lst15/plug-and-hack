// src/pipeline/modeler.rs
use crate::models::target::Target;
use crate::models::attack_surface::AttackSurface;

pub struct Modeler;

impl Modeler {
    pub fn build_attack_surface(targets: Vec<Target>) -> AttackSurface {
        AttackSurface {
            targets,
            timestamp: std::time::SystemTime::now(),
        }
    }
}