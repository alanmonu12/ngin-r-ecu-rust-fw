#![no_std]

//use stm32h7xx_hal::pac::syscfg;
// Re-exportamos el HAL para que los ejemplos lo puedan usar
pub use stm32h7xx_hal as hal;
pub use ecu_traits;

pub mod injector;
pub mod ignition;
pub mod pinout; // <--- Nuevo módulo
pub mod sensors;

use hal::prelude::*;
use hal::gpio::GpioExt;
//use injector::Stm32h7Injector;
use pinout::{map_hardware, RawPorts};
//use embedded_hal::digital::OutputPin;
use hal::delay::Delay; // Importar Delay
//use hal::gpio::ExtiPin;

use hal::adc::{Adc, Enabled, Resolution};
use hal::rcc::rec::AdcClkSel;

use cortex_m::Peripherals as CorePeripherals;
use hal::pac::Peripherals as DevicePeripherals;
//use hal::traits::Adc as AdcTrait;

// --- LA ESTRUCTURA DEL HARDWARE ---
// Esta struct representa tu ECU física completa.
pub struct Board {
    // Usamos los tipos abstractos definidos en pinout
    pub inyector_1: pinout::Inj1Driver,
    pub inyector_2: pinout::Inj2Driver,
    pub inyector_3: pinout::Inj3Driver,
    pub inyector_4: pinout::Inj4Driver,

    pub coil_1: pinout::Ign1Driver,
    pub coil_2: pinout::Ign2Driver,
    pub coil_3: pinout::Ign3Driver,
    pub coil_4: pinout::Ign4Driver,

    pub ckp: pinout::CkpDriver, // <--- Público para el firmware
    pub cmp: pinout::CmpDriver, // <--- Público para el firmware

    pub delay: Delay, // <--- La board incluye su propio reloj de espera

    pub tps: pinout::TpsPin,
    pub map: pinout::MapPin,
    pub iat: pinout::IatPin,
    pub cts: pinout::CtsPin,

    // EL COMPONENTE CLAVE: El ADC (Convertidor)
    // Usaremos ADC1 para todos estos sensores
    pub adc1: Adc<hal::device::ADC1, Enabled>,
}

impl Board {
    /// Función que toma los periféricos "crudos" del micro y devuelve la Board configurada
    pub fn init(dp: DevicePeripherals, cp: CorePeripherals) -> Self {
        // 1. Tomar periféricos crudos
        //let dp = hal::pac::Peripherals::take().unwrap();
        //let cp = cortex_m::Peripherals::take().unwrap(); // Necesario para el SYST (Delay)

        // 2. Configurar Relojes (RCC) - Centralizado aquí
        let mut syscfg = dp.SYSCFG;
        let mut exti = dp.EXTI;
        let pwr = dp.PWR.constrain();
        let pwrcfg = pwr.vos0(&syscfg).freeze();
        let rcc = dp.RCC.constrain();

        let rcc = rcc.use_hse(25.MHz());
        let mut ccdr = rcc
            .use_hse(25.MHz()) // Cristal externo de 25MHz
            .sys_ck(400.MHz()) // Petición de 480MHz
            .freeze(pwrcfg, &syscfg);

        ccdr.peripheral.kernel_adc_clk_mux(AdcClkSel::Per);
        //let mut syscfg = dp.SYSCFG;
        //let mut exti = dp.EXTI;

        // 3. Dividir los GPIOs (Split)
        let gpioe = dp.GPIOE.split(ccdr.peripheral.GPIOE);
        let gpioa = dp.GPIOA.split(ccdr.peripheral.GPIOA);
        let gpioc = dp.GPIOC.split(ccdr.peripheral.GPIOC);

        // 4. Empaquetamos los recursos crudos
        let raw_ports = RawPorts {
            gpioa,
            gpioc,
            gpioe,
        };

        // 5. LLAMAMOS AL MAPEO (Aquí ocurre la abstracción)
        let mut hardware = map_hardware(raw_ports);

        // Configuración adicional de interrupción para CKP
        // Queremos que interrumpa en el flanco de SUBIDA (Rising Edge)
        // Esto accede al registro EXTI hardware real.
        // --- CONFIGURACIÓN DE INTERRUPCIONES ---
        hardware.ckp.enable_interrupt(&mut syscfg, &mut exti);
        //hardware.ckp.trigger_on_edge(&mut exti, Edge::Rising);
        //hardware.ckp.enable_interrupt(&mut exti);

        let mut sys_delay = Delay::new(cp.SYST, ccdr.clocks);

        let mut adc1 = Adc::adc1(
            dp.ADC1, 
            4.MHz(),
            &mut sys_delay, 
            ccdr.peripheral.ADC12, 
            &ccdr.clocks
        ).enable();

        adc1.set_resolution(Resolution::SixteenBit);

        // 6. Retornar la estructura empaquetada
        Board {
            inyector_1: hardware.inj1,
            inyector_2: hardware.inj2,
            inyector_3: hardware.inj3,
            inyector_4: hardware.inj4,

            coil_1:     hardware.ing1,
            coil_2:     hardware.ing2,
            coil_3:     hardware.ing3,
            coil_4:     hardware.ing4,

            ckp: hardware.ckp,
            cmp: hardware.cmp,
            
            delay: sys_delay, // <--- Lo guardamos

            // Sensores y ADC
            tps: hardware.tps,
            map: hardware.map,
            iat: hardware.iat,
            cts: hardware.cts,
            adc1: adc1,
        }
    }
}