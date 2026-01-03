//! Static Configuration

use super::defs::{RestartPolicy, ServiceConfig};

/// Lista de serviços conhecidos pelo Supervisor
/// A ordem aqui define a ordem de tentativa de inicialização
pub const SERVICES: &[ServiceConfig] = &[
    // 1. Input Service (Teclado/Mouse)
    // Essencial iniciar antes da interface gráfica
    ServiceConfig {
        name: "input",
        path: "/system/services/input/input.app",
        args: &[],
        restart_policy: RestartPolicy::Always,
        critical: true,
    },
    // 2. Firefly Compositor (Interface Gráfica)
    ServiceConfig {
        name: "firefly",
        path: "/system/services/firefly/firefly.app",
        args: &[],
        restart_policy: RestartPolicy::Always,
        critical: true,
    },
    // 3. Shell (Desktop Environment)
    ServiceConfig {
        name: "shell",
        path: "/system/services/shell/shell.app",
        args: &[],
        restart_policy: RestartPolicy::Always,
        critical: true,
    },
];
