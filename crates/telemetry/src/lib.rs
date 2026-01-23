#![no_std]

use rtt_target::{rtt_init, UpChannel, ChannelMode};
use core::slice;
use core::mem;

#[repr(C, packed)]
struct CkpDebugPacket {
    header: u16,    // <--- NUEVO: 0x55AA (en Little Endian se ve como AA 55)
    rpm: u16,
    angle: f32,
    sync: u8,
    terminator: u8,
}

pub struct Telemetry {
    channel: UpChannel,
}

impl Telemetry {
    pub fn init() -> Self {
        // CORRECCIÓN FINAL:
        // 1. Solo definimos 'size'. 
        // 2. El 'mode' y 'name' a veces causan conflictos de sintaxis según la versión de la macro.
        //    (Channel 0 por defecto es "Terminal" y está bien así).
        let channels = rtt_init! {
            up: {
                0: {
                    size: 1024
                }
            }
        };

        let mut up_channel = channels.up.0;
        
        // Configuramos el modo NO BLOQUEANTE explícitamente aquí.
        // Esto evita que tu ECU se congele si nadie está leyendo los datos.
        up_channel.set_mode(ChannelMode::NoBlockSkip);

        Telemetry {
            channel: up_channel,
        }
    }

    pub fn send_ckp(&mut self, rpm: u16, angle: f32, is_sync: bool) {
        // En el send_ckp:
        let packet = CkpDebugPacket {
            header: 0x55AA, // Magic number
            rpm,
            angle,
            sync: if is_sync { 1 } else { 0 },
            terminator: 0xFF,
        };

        // SAFETY: Convertimos la struct a bytes crudos para enviar.
        let bytes = unsafe {
            slice::from_raw_parts(
                &packet as *const _ as *const u8,
                mem::size_of::<CkpDebugPacket>(),
            )
        };

        self.channel.write(bytes);
    }
}