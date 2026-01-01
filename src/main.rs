//! # RedstoneOS Supervisor (PID 1)
//!
//! O processo raiz do sistema. Responsável por iniciar e manter os serviços essenciais.

#![no_std]
#![no_main]

extern crate alloc;

// Módulo de serviços
mod service;

use core::panic::PanicInfo;
use redpowder::println;
use service::manager::ServiceManager;

/// Global allocator usando syscalls do kernel (heap)
#[global_allocator]
static ALLOCATOR: redpowder::mem::heap::SyscallAllocator = redpowder::mem::heap::SyscallAllocator;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("");
    println!("==================================================");
    println!("   RedstoneOS Supervisor (PID 1) v0.2.0");
    println!("==================================================");
    println!("[Supervisor] Inicializando...");

    let mut manager = ServiceManager::new();

    // Iniciar serviços configurados
    manager.start_all();

    println!("[Supervisor] Sistema inicializado. Entrando em loop de supervisão.");

    // Entrar no loop eterno de supervisão
    manager.supervision_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("");
    println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
    println!("             SUPERVISOR PANIC (PID 1)             ");
    println!("!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
    println!("Erro: {}", info);
    println!("O sistema está instável e parou.");

    loop {
        // Halt loops
    }
}
