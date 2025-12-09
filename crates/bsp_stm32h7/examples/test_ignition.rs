#![no_main]
#![no_std]

use panic_halt as _;
use cortex_m_rt::entry;
use bsp_stm32h7::Board;
use bsp_stm32h7::hal::prelude::*;

use bsp_stm32h7::ecu_traits::engine_io::{Injector, IgnitionCoil};

// I: Tipo generico que implementa el Trait Injector
// C: Tipo generico que impleneta el trait coil
fn probar_cilindro<I, C>(
    inyector: &mut I,
    coil: &mut C,
    delay: &mut bsp_stm32h7::hal::delay::Delay
)
where 
    I: Injector,
    C: IgnitionCoil,
{
    // 1. Se inicia el tiempo de carga de la bobina
    let _ = coil.start_dwell();

    // Se simula la aperutra del inyector, tenemos combustible
    let _ = inyector.open();

    // Este seria el teimpo total de carga del inyector
    // Este tiempo es manejadfo por RTIC y calculado en tiempo de ejecucion
    delay.delay_ms(5u32);

    // Se apga la bobina para generar chispa (fuego en el hoyo)
    let _ = coil.coil_fire();

    // Delay para el cierre del inyector, no sabemos si es necesario en la aplicacion
    delay.delay_ms(2u32);
    let _ = inyector.close();
}


#[entry]
fn main() -> ! {

    let mut board = Board::init();

    loop{
        // Se simula en encendido de las bobinas en secuancia con delay

        probar_cilindro(&mut board.inyector_1, &mut board.coil_1,&mut board.delay);
        board.delay.delay_ms(100u32);

        probar_cilindro(&mut board.inyector_3, &mut board.coil_3,&mut board.delay);
        board.delay.delay_ms(100u32);

        probar_cilindro(&mut board.inyector_4, &mut board.coil_4,&mut board.delay);
        board.delay.delay_ms(100u32);

        probar_cilindro(&mut board.inyector_2, &mut board.coil_2,&mut board.delay);
        board.delay.delay_ms(100u32);

        // Ultimo delay para esperar el siguiente ciclo
        //board.delay.delay_ms(1000u32);
    }
}
