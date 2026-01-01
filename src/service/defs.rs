#![allow(dead_code)]
//! Definitions for Supervisor

/// Status do serviço
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceStatus {
    Starting,
    Running,
    Stopped,
    Failed,
    Restarting,
}

/// Política de reinício
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RestartPolicy {
    Always,
    OnFailure,
    Never,
}

/// Configuração estática de um serviço
#[derive(Debug, Clone, Copy)]
pub struct ServiceConfig {
    pub name: &'static str,
    pub path: &'static str,
    pub args: &'static [&'static str],
    pub restart_policy: RestartPolicy,
    pub critical: bool, // Se true, o supervisor pode entrar em pânico ou tentar recuperação agressiva
}

/// Estado dinâmico de um serviço em execução
pub struct ServiceState {
    pub config: ServiceConfig,
    pub pid: Option<usize>,
    pub status: ServiceStatus,
    pub restart_count: u32,
    pub last_restart: u64, // Timestamp (ticks ou ms)
}

impl ServiceState {
    pub fn new(config: ServiceConfig) -> Self {
        Self {
            config,
            pid: None,
            status: ServiceStatus::Stopped,
            restart_count: 0,
            last_restart: 0,
        }
    }
}
