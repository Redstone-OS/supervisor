# Supervisor (PID 1)

O **Supervisor** é o coração do user-space do RedstoneOS. Ele atua como o **PID 1**, sendo o primeiro processo iniciado pelo kernel e responsável por orquestrar todo o sistema.

## 🚀 Funcionalidades

Diferente de um `init` simples que apenas roda um script, o Supervisor do RedstoneOS é um gerenciador de serviços ativo:

- **Gerenciamento de Ciclo de Vida**: Inicia, para e monitora serviços.
- **Auto-Healing (Recuperação Automática)**: Se um serviço crítico (como o Compositor ou Shell) falhar, o Supervisor o detecta e reinicia automaticamente.
- **Gestão de Dependências (Estática)**: Garante uma ordem de inicialização correta (ex: Input -> Compositor -> Shell).
- **Zumbi Reaping**: Adota processos órfãos e limpa seus estados para evitar vazamento de recursos no kernel.

## 📂 Estrutura do Projeto

O código foi organizado para ser modular e extensível:

```text
services/supervisor/
├── src/
│   ├── main.rs            # Ponto de entrada (PID 1)
│   └── service/           # Módulo de Serviços
│       ├── log.rs         # Definições (Service, ServiceStatus)
│       ├── config.rs      # Lista ESTÁTICA de serviços (Hardcoded)
│       └── manager.rs     # Lógica Core (Spawn, Monitor, Restart)
└── Cargo.toml
```

## ⚙️ Configuração de Serviços

Atualmente, a lista de serviços é definida estaticamente em `src/service/config.rs`. Isso garante segurança e rapidez no boot.

Exemplo de configuração:

```rust
ServiceConfig {
    name: "firefly",
    path: "/system/services/firefly",
    args: &[],
    restart_policy: RestartPolicy::Always, // Se morrer, nasce de novo
    critical: true,
}
```

### Serviços Padrão
1.  **Input Service**: Drivers de Mouse/Teclado (User-space).
2.  **Firefly Compositor**: Gerenciador de Janelas e Gráficos.
3.  **Shell**: Ambiente de Trabalho (Barra de tarefas, Wallpaper).
4.  **Terminal**: Emulador de terminal.

## 🛠️ Como Compilar

Este serviço é compilado como parte do build do RedstoneOS. Para testar individualmente:

```bash
cargo build --release --target x86_64-unknown-none
```

O binário resultante deve ser colocado em `/init` ou `/system/supervisor` na imagem de disco/initrd.
