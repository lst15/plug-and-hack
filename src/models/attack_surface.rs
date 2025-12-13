// src/models/attack_surface.rs
use crate::models::target::Target;
use std::time::SystemTime;

#[derive(Debug)]
pub struct AttackSurface {
    pub targets: Vec<Target>,
    pub timestamp: SystemTime,
}