//! Service Manager Core Logic

use super::config::SERVICES;
use super::defs::{RestartPolicy, ServiceState, ServiceStatus};
use alloc::vec::Vec;
use redpowder::process::spawn;
use redpowder::{println, time};

pub struct ServiceManager {
    services: Vec<ServiceState>,
}

impl ServiceManager {
    pub fn new() -> Self {
        let mut services = Vec::new();
        // Carregar configurações estáticas
        for cfg in SERVICES {
            services.push(ServiceState::new(*cfg));
        }
        Self { services }
    }

    /// Inicia todos os serviços marcados para start
    pub fn start_all(&mut self) {
        println!("[Supervisor] Iniciando serviços...");

        // Como o borrow checker do Rust não gosta de iterar e modificar,
        // usamos indices.
        for i in 0..self.services.len() {
            self.start_service(i);

            // Delay removido pois estava causando travamento se o scheduler não voltasse
            // rápido o suficiente. Melhor iniciar tudo e deixar eles se entenderem via IPC.
            // time::sleep(100).ok();
        }
    }

    fn start_service(&mut self, index: usize) {
        let svc = &mut self.services[index];

        println!(
            "[Supervisor] Iniciando '{}' ({})",
            svc.config.name, svc.config.path
        );

        match spawn(svc.config.path, svc.config.args) {
            Ok(pid) => {
                println!(
                    "[Supervisor] '{}' iniciado com PID {}",
                    svc.config.name, pid
                );
                svc.pid = Some(pid);
                svc.status = ServiceStatus::Running;
            }
            Err(e) => {
                println!(
                    "[Supervisor] FALHA ao iniciar '{}': {:?}",
                    svc.config.name, e
                );
                svc.status = ServiceStatus::Failed;
                svc.pid = None;
            }
        }
    }

    /// Loop principal de supervisão
    /// Bloqueia usando wait() mas com timeout, para periodicamente checar outras coisas
    pub fn supervision_loop(&mut self) -> ! {
        loop {
            // 1. Verificar processos filhos mortos (Reaping)
            // Timeout de 1000ms significa que acordamos a cada segundo mesmo se nada acontecer
            // 1. Sleep loop
            // TODO: Implementar sys_wait no kernel para usar wait(0, timeout)
            // Por enquanto usamos sleep para não gastar 100% CPU
            time::sleep(1000).ok();

            // Verificar status
            self.check_services_vitality();

            // 2. Tentar reiniciar serviços falhos
            self.restart_failed_services();
        }
    }

    fn check_services_vitality(&mut self) {
        for i in 0..self.services.len() {
            let pid = self.services[i].pid;
            if let Some(p) = pid {
                // Tenta esperar especificamente por este PID com timeout 0 (poll)
                match redpowder::process::wait(p, 0) {
                    Ok(exit_code) => {
                        self.handle_service_exit(i, exit_code);
                    }
                    Err(redpowder::syscall::SysError::NotFound) => {
                        // Ainda rodando, tudo ok
                    }
                    Err(e) => {
                        println!("[Supervisor] Erro ao monitorar PID {}: {:?}", p, e);
                        // Se der erro crítico, assumimos que morreu para tentar recuperar
                        self.handle_service_exit(i, -1);
                    }
                }
            }
        }
    }

    fn handle_service_exit(&mut self, index: usize, code: i32) {
        let svc = &mut self.services[index];
        println!(
            "[Supervisor] Serviço '{}' (PID {:?}) morreu com código {}",
            svc.config.name, svc.pid, code
        );

        svc.pid = None;
        svc.status = ServiceStatus::Stopped;

        match svc.config.restart_policy {
            RestartPolicy::Always => {
                println!(
                    "[Supervisor] Agendando reinício de '{}'...",
                    svc.config.name
                );
                svc.status = ServiceStatus::Restarting;
            }
            RestartPolicy::OnFailure if code != 0 => {
                println!(
                    "[Supervisor] Falha detectada. Agendando reinício de '{}'...",
                    svc.config.name
                );
                svc.status = ServiceStatus::Restarting;
            }
            _ => {
                println!(
                    "[Supervisor] Serviço '{}' não será reiniciado (Política {:?})",
                    svc.config.name, svc.config.restart_policy
                );
            }
        }
    }

    fn restart_failed_services(&mut self) {
        for i in 0..self.services.len() {
            if self.services[i].status == ServiceStatus::Restarting {
                self.services[i].restart_count += 1;
                println!(
                    "[Supervisor] Reiniciando serviço '{}' (Tentativa {})",
                    self.services[i].config.name, self.services[i].restart_count
                );
                self.start_service(i);
            }
        }
    }
}
