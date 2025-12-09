#![no_main]
#![no_std]

use panic_halt as _;
use cortex_m_rt::entry;

// Importamos HAL y Prelude
use bsp_stm32h7::hal::prelude::*;
use bsp_stm32h7::hal::pac;
use bsp_stm32h7::hal::delay::Delay; // <--- El delay vive AQUÍ, en el test, no en el driver

// Importamos nuestro driver
use bsp_stm32h7::injector::Stm32h7Injector;
use bsp_stm32h7::ecu_traits::engine_io::Injector; // Trait

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    // 1. Configuración de Relojes (Igual que siempre)
    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.freeze();
    let rcc = dp.RCC.constrain();
    let ccdr = rcc.sys_ck(100.MHz()).freeze(pwrcfg, &dp.SYSCFG);

    // 2. Configurar Pin
    let gpioe = dp.GPIOE.split(ccdr.peripheral.GPIOE);
    let inject_pin = gpioe.pe3.into_push_pull_output();
    let inject_pin_2 = gpioe.pe2.into_push_pull_output();

    // 3. Inicializar el Driver (Fíjate que ya no le pasamos el delay)
    let mut inyector_1 = Stm32h7Injector::new(inject_pin);
    let mut inyector_2 = Stm32h7Injector::new(inject_pin_2);

    // 4. Inicializar el Reloj Maestro (Simulando el Scheduler de RTIC)
    let mut timer_simulador = Delay::new(cp.SYST, ccdr.clocks);

    // 5. Bucle de Inyección
    loop {
        // --- INICIO DE EVENTO (Simulado) ---
        
        // A) RTIC dice: "¡Abre ahora!"
        let _ = inyector_1.open();

        // B) RTIC espera el tiempo de inyección (ej. 4ms)
        // El driver ya retornó, el CPU está "esperando" aquí en el test
        timer_simulador.delay_ms(4u32);

        // C) RTIC dice: "¡Tiempo cumplido, cierra!"
        let _ = inyector_1.close();

        // --- FIN DE EVENTO ---

        // --- INICIO DE EVENTO (Simulado) ---
        
        // A) RTIC dice: "¡Abre ahora!"
        let _ = inyector_2.open();

        // B) RTIC espera el tiempo de inyección (ej. 4ms)
        // El driver ya retornó, el CPU está "esperando" aquí en el test
        timer_simulador.delay_ms(10u32);

        // C) RTIC dice: "¡Tiempo cumplido, cierra!"
        let _ = inyector_2.close();

        // --- FIN DE EVENTO ---

        // Esperamos un poco antes del siguiente ciclo
        timer_simulador.delay_ms(100u32);
    }
}