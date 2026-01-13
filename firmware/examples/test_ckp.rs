#![no_main]
#![no_std]

use panic_halt as _; // En caso de pánico, detenerse
use rtic::app;
use rtic_monotonics::systick::prelude::*;
use rtt_target::{rtt_init_print, rprintln};

// Importamos nuestras librerías
use bsp_stm32h7::Board;
use engine_core::decoder::{TriggerDecoder, DecoderEvent};
use engine_core::decoders::MissingToothDecoder;

extern crate cortex_m;

systick_monotonic!(Mono, 1000);

#[app(device = bsp_stm32h7::hal::pac, peripherals = true)]
mod app {
    use super::*;
        
    //Recursos compartidos entre las tareas
    #[shared]
    struct Shared {
        decoder: MissingToothDecoder,
        rpm_para_mostrar: u16,
    }

    #[local]
    struct  Local {
        board: Board,
    }

    #[init]
    fn init(mut cx: init::Context) -> (Shared, Local) {
        rtt_init_print!();
        rprintln!("--- ECU START: CKP INTEGRATION TEST ---");

        // 2. CONFIGURACIÓN DEL DWT (Contador de Ciclos)
        // Necesitamos acceso mutable a los periféricos del Core (Cortex-M)
        
        // A) Habilitar el Trace (Requisito para que el DWT corra)
        cx.core.DCB.enable_trace();
        
        // B) Resetear el contador a 0 (Buena práctica)
        cx.core.DWT.set_cycle_count(0);
        
        // C) Encender el contador
        cx.core.DWT.enable_cycle_counter();

        // Se inicializa el hardware
        let mut board = Board::init();

        let decoder = MissingToothDecoder::new(60, 2);

        rprintln!("Hardware Initialized. Waiting for signal...");

        (
            Shared {
                decoder,
                rpm_para_mostrar: 0,
            },
            Local{
                board,
            },
        )

    }

    // Tarea de IDLE
    #[idle(shared = [rpm_para_mostrar])]
    fn idle(mut cx: idle::Context) -> ! {
        loop {
            
            let rpm = cx.shared.rpm_para_mostrar.lock(|rpm| *rpm);

            if rpm > 0 {
                rprintln!("Motor Girando: {} RPM", rpm);
            }

            Mono::delay(1000.millis());
        }
    }

    // Tarea que hace la lectura del sensor
    #[task(binds = EXTI4, local = [board], shared = [decoder, rpm_para_mostrar])]
    fn on_ckp_event (mut cx: on_ckp_event::Context) {
        // Se limpia la interrupcion para evitar ciclado
        cx.local.board.ckp.disable_interrupt(EXTI4);

        // Obtenemos el tiempo actuial en el que ocurre el flanco de subida
        // Usamos el DWT (Cycle Counter) del Cortex-M para precisión de nanosegundos.
        // DWT::get_cycle_count() devuelve ciclos de reloj.
        // A 480MHz, dividimos entre 480 para tener microsegundos.
        let cycle = cortex_m::peripheral::DWT::cycle_count();
        let timestamp_us = cycle/480;

        cx.shared.decoder.lock(|decoder| {
            let event = decoder.on_edge(timestamp_us);
            
            match event {
                DecoderEvent::SyncGained => {

                },
                DecoderEvent::SyncLost => {

                },
                _ => {}

            }

            // Guardamos el valor leido en lo recursos compartidos
            // Solo actualizamos el shared resource a veces para ahorrar tiempo,
            // o siempre si es rápido. El lock es barato en RTIC.
            let current_rpm = decoder.get_rpm();
            cx.shared.rpm_para_mostrar.lock(|r| *r = current_rpm);
        });
    }


}