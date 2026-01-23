#![no_main]
#![no_std]

use panic_halt as _; // En caso de pánico, detenerse
use rtic::app;
use rtic_monotonics::systick::prelude::*;
// use rtt_target::{rtt_init_print, rprintln};

// Importamos nuestras librerías
use bsp_stm32h7::Board;
use engine_core::decoder::{TriggerDecoder, DecoderEvent};
use engine_core::decoders::MissingToothDecoder;

use telemetry::Telemetry;

extern crate cortex_m;

systick_monotonic!(Mono, 1000);

#[app(device = bsp_stm32h7::hal::pac, peripherals = true, dispatchers = [SPI1])]
mod app {
    //use core::task;

    use ecu_traits::engine_io::RotationSensor;

    use super::*;
        
    //Recursos compartidos entre las tareas
    #[shared]
    struct Shared {
        telemetry: Telemetry,
        decoder: MissingToothDecoder,
        rpm_para_mostrar: u16,
    }

    #[local]
    struct  Local {
        board: Board,
    }

    #[init]
    fn init(mut cx: init::Context) -> (Shared, Local) {
        // rtt_init_print!();
        // rprintln!("--- ECU START: CKP INTEGRATION TEST ---");

        let telemetry = Telemetry::init();

        // Activar caches para rendimiento real
        cx.core.SCB.enable_icache();
        // Activar D-Cache con cuidado (recuerda gestionar coherencia si usas DMA)
        cx.core.SCB.enable_dcache(&mut cx.core.CPUID);
        // 2. CONFIGURACIÓN DEL DWT (Contador de Ciclos)
        // Necesitamos acceso mutable a los periféricos del Core (Cortex-M)
        
        // A) Habilitar el Trace (Requisito para que el DWT corra)
        cx.core.DCB.enable_trace();
        
        // B) Resetear el contador a 0 (Buena práctica)
        cx.core.DWT.set_cycle_count(0);
        
        // C) Encender el contador
        cx.core.DWT.enable_cycle_counter();

        // Se inicializa el hardware
        let board = Board::init(cx.device, cx.core);

        // Inicializamos el Monotonic (SysTick) para los delays asíncronos.
        // Usamos steal() porque Board::init consumió cx.core.
        let syst = unsafe { cortex_m::peripheral::Peripherals::steal().SYST };
        Mono::start(syst, 400_000_000);

        let decoder = MissingToothDecoder::new(60, 2);

        // rprintln!("Hardware Initialized. Waiting for signal...");

        // Lanzamos la tarea de monitoreo asíncrona
        monitor_rpm::spawn().ok();

        (
            Shared {
                telemetry,
                decoder,
                rpm_para_mostrar: 0,
            },
            Local{
                board,
            },
        )

    }

    // Tarea de IDLE
    #[idle(shared = [decoder, rpm_para_mostrar])]
    fn idle(_: idle::Context) -> ! {
        loop {
            cortex_m::asm::wfi();
        }
    }

    /// Tarea asíncrona que monitorea el RPM y estado de sincronización del motor.
    #[task(priority = 1, shared = [decoder, rpm_para_mostrar, telemetry])]
    async fn monitor_rpm(mut cx: monitor_rpm::Context) {
        
        loop {

            let (rpm, status, angle, timestamp_us) = cx.shared.decoder.lock(|dec| {
                // Obtenemos el tiempo aquí mismo para que sea coherente
                let cycle = cortex_m::peripheral::DWT::cycle_count();
                let now_us = cycle / 400;
                
                (dec.get_instant_rpm(), dec.is_synced(), dec.get_angle(), now_us)
            });
            
            cx.shared.telemetry.lock(|telem| {
                telem.send_ckp(rpm, angle, status);
            });

            cx.shared.decoder.lock(|dec| {
                if dec.check_stall(timestamp_us) { // Funciona para TODOS
                    cx.shared.rpm_para_mostrar.lock(|r| *r = 0);
                }
            });

            Mono::delay(100.millis()).await;
        }
    }

    // Tarea que hace la lectura del sensor
    #[task(binds = EXTI4, priority = 2, local = [board], shared = [decoder, rpm_para_mostrar])]
    fn on_ckp_event (mut cx: on_ckp_event::Context) {
        //rprintln!("CKP Event Interrupt Triggered");
        // Se limpia la interrupcion para evitar ciclado
        let _ = cx.local.board.ckp.acknowledge_interrupt();

        // Obtenemos el tiempo actuial en el que ocurre el flanco de subida
        // Usamos el DWT (Cycle Counter) del Cortex-M para precisión de nanosegundos.
        // DWT::get_cycle_count() devuelve ciclos de reloj.
        // A 480MHz, dividimos entre 480 para tener microsegundos.
        let cycle = cortex_m::peripheral::DWT::cycle_count();
        let timestamp_us = cycle/400;

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