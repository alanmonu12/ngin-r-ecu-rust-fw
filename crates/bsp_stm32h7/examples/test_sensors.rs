#![no_main]
#![no_std]

use panic_halt as _;
use cortex_m_rt::entry;
use bsp_stm32h7::Board;
use bsp_stm32h7::ecu_traits::engine_io::RotationSensor;
use bsp_stm32h7::hal::prelude::*;
use rtt_target::{rtt_init_print, rprintln};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("--- INICIO DE TEST DE SENAL CKP ---");
    rprintln!("Conecta tu generador de señales a PA4 (CKP)");

    let mut board = Board::init();
    
    // Variables para el algoritmo de conteo
    let mut pulsos = 0;
    let mut estado_anterior = false;
    
    // Para medir tiempo (usamos el delay de la board como referencia burda)
    // En una prueba real usariamos un Timer, pero para validar señal esto basta.
    let ventanas_de_muestreo = 1000; // 1000 iteraciones de 1ms = 1 segundo aprox

    loop {
        pulsos = 0;
        
        // Ventana de tiempo de 1 segundo (aprox)
        for _ in 0..ventanas_de_muestreo {
            // Muestreamos rápido dentro del milisegundo
            // Hacemos 100 lecturas rápidas para no perder pulsos
            for _ in 0..100 {
                // 1. Lectura del Pin (Polling)
                let estado_actual = board.ckp.get_state().unwrap_or(false);

                // 2. Detección de Flanco de Subida (Rising Edge)
                // Si antes estaba bajo (false) y ahora es alto (true) -> ¡Pulso!
                if estado_actual && !estado_anterior {
                    pulsos += 1;
                }
                
                estado_anterior = estado_actual;
                
                // Pequeño delay para estabilizar (muy corto)
                //board.delay.delay_us(100u32);
                //cortex_m::asm::delay(150); 
            }
            
            // Avanzamos el reloj de "tiempo real"
            board.delay.delay_ms(1u32);
        }

        // --- CÁLCULOS ---
        // Frecuencia = Pulsos por segundo
        let hertz = pulsos;
        
        // RPM (Asumiendo rueda 60-2 o similar, digamos 60 dientes por vuelta)
        // RPM = (Frecuencia * 60 segundos) / Dientes
        let dientes_rueda = 60; 
        let rpm = (hertz * 60) / dientes_rueda;

        rprintln!("Frecuencia: {} Hz | RPM Estimadas (60t): {}", hertz, rpm);
    }
}