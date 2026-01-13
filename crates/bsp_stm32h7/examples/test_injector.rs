#![no_main]
#![no_std]

use panic_halt as _;
use cortex_m_rt::entry;
use bsp_stm32h7::Board; // Solo importamos la Board
use bsp_stm32h7::ecu_traits::engine_io::Injector;
use bsp_stm32h7::hal::prelude::*;


#[entry]
fn main() -> ! {
    // 1. Inicialización de Hardware (Una sola línea)
    // Esto prueba implícitamente que el mapeo de pines y relojes es correcto.
    let mut board = Board::init();

    // 2. Bucle de Prueba
    loop {
        // --- CILINDRO 1 ---
        let _ = board.inyector_1.close();
        
        // Usamos el delay que vive dentro de la board
        board.delay.delay_ms(100u32); 
        
        let _ = board.inyector_1.open();

        // --- CILINDRO 2 ---
        let _ = board.inyector_2.open();
        
        //board.delay.delay_ms(500u32);
        
        let _ = board.inyector_2.close();

        // Ciclo muerto
        board.delay.delay_ms(100u32);
    }
}