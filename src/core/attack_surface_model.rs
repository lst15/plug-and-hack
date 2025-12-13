// src/core/attack_surface_model.rs

use crate::models::target::Target;
use std::time::SystemTime;

/// Representa uma superfície de ataque modelada a partir de dados OSINT.
pub trait AttackSurfaceModel {
    /// Retorna a lista de alvos identificados
    fn targets(&self) -> &[Target];

    /// Retorna o timestamp da coleta/modelagem
    fn timestamp(&self) -> SystemTime;

    /// Filtra alvos por porta
    fn filter_by_port(&self, port: u16) -> Vec<&Target> {
        self.targets().iter().filter(|t| t.port == port).collect()
    }

    /// Filtra alvos com vulnerabilidades conhecidas
    fn vulnerable_targets(&self) -> Vec<&Target> {
        self.targets().iter().filter(|t| !t.vulns.is_empty()).collect()
    }

    /// Verifica se a superfície está vazia
    fn is_empty(&self) -> bool {
        self.targets().is_empty()
    }
}

/// Trait para construir uma superfície de ataque a partir de alvos brutos.
pub trait AttackSurfaceBuilder {
    type Input;
    fn build_from(&self, input: Self::Input) -> Box<dyn AttackSurfaceModel>;
}