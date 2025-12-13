// src/pipeline/executor.rs
use crate::core::exploit_engine::ExploitEngine;
use crate::engines::custom::React2ShellEngine;
use crate::models::attack_surface::AttackSurface;
use crate::models::target::Target;
use crate::utils::error::Result;

pub struct Executor;

impl Executor {

    // src/pipeline/executor.rs
    // src/pipeline/executor.rs
    pub async fn run_react2shell(engine: &React2ShellEngine, surface: &AttackSurface) -> Result<()> {
        let targets: Vec<&Target> = surface
            .targets // ← campo, não método!
            .iter()
            .filter(|t| t.vulns.contains(&"CVE-2025-55182".to_string()))
            .collect();

        // exploit_many espera &[Target], não &[&Target]
        let owned_targets: Vec<Target> = targets.into_iter().cloned().collect();
        engine.exploit_many(&owned_targets).await
    }
    pub async fn run<E>(&self, engine: &E, surface: &AttackSurface) -> Result<()>
    where
        E: ExploitEngine,
    {
        for target in &surface.targets {
            if !target.vulns.is_empty() {
                // Aqui você pode filtrar por engine suportada, CVE, etc.
                engine.exploit(target).await?;
            }
        }
        Ok(())
    }
}