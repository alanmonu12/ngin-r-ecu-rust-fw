use engine_core::decoder::{TriggerDecoder, DecoderEvent};
use engine_core::decoders::MissingToothDecoder;

#[test]
fn test_60_minus_2_sync() {
    // 1. Configurar: Rueda 60-2
    let mut decoder = MissingToothDecoder::new(60, 2);

    // 2. Simulación
    // Imaginemos que el motor gira constante a una velocidad donde 
    // cada diente tarda 1000 us (1ms).
    // El hueco debería tardar 3000 us (3 dientes faltantes de tiempo).
    
    let tooth_time = 1000;
    let mut current_time = 0;

    // A) Mandamos 5 dientes normales (Aún no hay sync)
    for _ in 0..5 {
        current_time += tooth_time;
        let evt = decoder.on_edge(current_time);
        assert_eq!(evt, DecoderEvent::ToothProcessed);
        assert_eq!(decoder.is_synced(), false, "No debería tener sync todavía");
    }

    // B) Mandamos el HUECO (Gap)
    // El tiempo salta 3 veces lo normal (simulando los 2 dientes que faltan + el actual)
    current_time += tooth_time * 3; 
    let evt = decoder.on_edge(current_time);

    // C) Validación
    assert_eq!(evt, DecoderEvent::SyncGained, "Debería haber detectado el hueco");
    assert_eq!(decoder.is_synced(), true);
    assert_eq!(decoder.get_angle(), 0.0, "Después del hueco, el ángulo debe ser 0 (o cerca)");

    // D) Mandamos otro diente normal
    current_time += tooth_time;
    let evt = decoder.on_edge(current_time);
    assert_eq!(decoder.get_angle(), 6.0, "Diente 1 debe ser 6 grados (360/60)");
}

#[test]
fn test_timer_overflow() {
    let mut decoder = MissingToothDecoder::new(60, 2);
    
    // Situación: Estamos al borde del final de u32
    // u32::MAX es 4,294,967,295
    let mut time = u32::MAX; 
    let step = 1000; // 1ms por diente

    assert_eq!(time, u32::MAX);
    // Mandamos un diente antes del desbordamiento
    let mut event = decoder.on_edge(time); 

    assert_eq!(event, DecoderEvent::ToothProcessed);
    
    // Mandamos el siguiente diente (Esto causará el wrap around a números pequeños)
    // time pasará de 4,294,967,xxx a aprox 500
    time = time.wrapping_add(step); 
    assert_eq!(time, 999);
    
    // Si tu código usa wrapping_sub, esto no debería paniquear ni dar un delta gigante
    event = decoder.on_edge(time);

    // El decoder debería aceptarlo como un diente normal (ToothProcessed)
    // Si fallara la matemática, detectaría un "Gap" falso o crashearía.
    assert_eq!(event, DecoderEvent::ToothProcessed);
}