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
    let mut evt = DecoderEvent::None;

    // mandamos un diente que se ignora para detectar el inicio de arranque
    current_time += tooth_time;
    evt = decoder.on_edge(current_time);
    assert_eq!(evt, DecoderEvent::None);
    
    // A) Mandamos 5 dientes normales (Aún no hay sync)
    for _ in 0..5 {
        current_time += tooth_time;
        evt = decoder.on_edge(current_time);
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
    
    // Arrancamos muy cerca del final del reloj (u32::MAX)
    let mut time = u32::MAX - 2500; 
    let step = 1000; 

    // 1. PRIMER PULSO (Ignorado por lógica de arranque)
    // Sirve para setear last_timestamp = u32::MAX - 2500
    decoder.on_edge(time); 
    
    // 2. SEGUNDO PULSO (Establece el primer Delta válido)
    // time avanza a u32::MAX - 1500
    // Delta calculado = 1000. last_delta = 1000.
    time = time.wrapping_add(step);
    let event_init = decoder.on_edge(time);
    assert_eq!(event_init, DecoderEvent::ToothProcessed);

    // 3. TERCER PULSO (EL TEST REAL DE OVERFLOW)
    // time avanza y DA LA VUELTA (Wrap around)
    // time pasa de ser gigante a ser pequeño (aprox 500)
    time = time.wrapping_add(step); 
    
    // Aquí es donde wrapping_sub salva el día.
    // Si usaras resta normal: 500 - 4,294,965,795 = CRASH (Panic)
    // Con wrapping_sub: Resultado = 1000.
    let event_overflow = decoder.on_edge(time);

    // Si todo salió bien, el decoder vio un diente de 1000us normal
    assert_eq!(event_overflow, DecoderEvent::ToothProcessed);
}

#[test]
fn test_wheel_36_minus_1_config() {
    // 1. Configuramos una rueda distinta: 36 dientes, falta 1
    // En esta rueda, el hueco dura 2 tiempos (el espacio del faltante + el actual)
    let mut decoder = MissingToothDecoder::new(36, 1);
    
    // Simulamos 1000 RPM (aprox 1666 us por diente)
    let tooth_time = 1666; 
    let mut current_time = 10_000_000; // Empezamos en un tiempo arbitrario alto

    // A) Primer pulso (Reset/Start)
    decoder.on_edge(current_time);

    // B) Unos cuantos dientes normales para estabilizar
    for _ in 0..10 {
        current_time += tooth_time;
        decoder.on_edge(current_time);
        assert!(!decoder.is_synced());
    }

    // C) EL HUECO (GAP) de una 36-1
    // La duración es x2 (1 faltante + 1 actual)
    current_time += tooth_time * 2; 
    
    // El decoder compara: NuevoDelta (3332) > AnteriorDelta (1666) * 1.5
    // 3332 > 2499 -> TRUE
    let evt = decoder.on_edge(current_time);

    assert_eq!(evt, DecoderEvent::SyncGained, "Debe detectar hueco en 36-1");
    assert_eq!(decoder.get_angle(), 0.0);
}

#[test]
fn test_hard_acceleration_through_gap() {
    // Rueda 60-2
    let mut decoder = MissingToothDecoder::new(60, 2);
    
    // Arrancamos lento (2000 us por diente)
    let mut tooth_duration = 2000; 
    let mut current_time = 1000;

    // Primer pulso
    decoder.on_edge(current_time);

    // Bucle de aceleración: Cada diente es 10us más rápido que el anterior
    // Simulamos llegar hasta justo antes del hueco
    for _ in 0..55 {
        current_time += tooth_duration;
        decoder.on_edge(current_time);
        
        // ACELERACIÓN: El siguiente diente será más rápido
        tooth_duration -= 20; 
    }

    // Guardamos la duración del "último diente normal" para referencia del test
    let last_normal_duration = tooth_duration; // Digamos que bajó a ~900us

    // AHORA VIENE EL HUECO
    // El motor sigue acelerando DURANTE el hueco.
    // En teoría 60-2 es x3. 
    // Pero como aceleramos, el tiempo real será un poco menos de 3 veces el ANTERIOR.
    // Vamos a ser agresivos: Digamos que el hueco dura 2.8 veces el anterior debido a la aceleración
    let gap_duration = (last_normal_duration as f32 * 2.8) as u32;
    
    current_time += gap_duration;
    let evt = decoder.on_edge(current_time);

    // Verificación:
    // Tu lógica es: Delta (2.8x) > Last (1.0x) * 1.5
    // 2.8 > 1.5 -> TRUE. ¡Debe pasar!
    assert_eq!(evt, DecoderEvent::SyncGained, "Debe sincronizar aun acelerando");
}

#[test]
fn test_rpm_calculation_accuracy() {
    let mut decoder = MissingToothDecoder::new(60, 2);
    
    // Configuración: 1000us por diente en rueda 60-2 equivale EXACTAMENTE a 1000 RPM.
    let tooth_time = 1000;
    let mut current_time = 10_000; // Empezamos lejos del 0

    // 1. Primer pulso (Inicialización)
    decoder.on_edge(current_time);

    // 2. Segundo pulso (Establece el primer delta)
    current_time += tooth_time;
    decoder.on_edge(current_time);

    // 3. Verificamos la RPM Instantánea
    // Debería ser 1000 exactos (o 999/1001 por redondeo entero)
    let rpm = decoder.get_instant_rpm();
    assert_eq!(rpm, 1000, "1000us/diente en 60t debe ser 1000 RPM");
}

#[test]
fn test_rpm_filtering_response() {
    let mut decoder = MissingToothDecoder::new(60, 2);
    
    // Paso 1: Estabilizar a 1000 RPM (1000us por diente)
    let steady_time = 1000;
    let mut current_time = 1_000_000;

    // Mandamos 50 dientes estables para que el filtro "se llene" y converja a 1000
    // (Como el filtro es exponencial, tarda unos cuantos ciclos en llegar al valor objetivo)
    for _ in 0..1000 {
        current_time += steady_time;
        decoder.on_edge(current_time);
    }
    
    // Verificamos que ya estamos estables
    let rpm = decoder.get_rpm();
    // verificamos con un 10% de tolerancia para el filtro
    assert!(rpm >= 990 && rpm <= 1010, "El filtro se quedó lejos: {}", rpm);

    // Paso 2: SIMULAR UN PICO (Ruido o Explosión de cilindro)
    // El siguiente diente llega en la mitad del tiempo (500us -> 2000 RPM instantáneo)
    current_time += 500; 
    decoder.on_edge(current_time);

    // Verificaciones:
    
    // A) La instantánea DEBE detectar el cambio brusco inmediatamente
    let instant = decoder.get_instant_rpm();
    assert_eq!(instant, 2000, "Instantánea debe reaccionar inmediatamente al pico");

    // B) La filtrada DEBE amortiguar el golpe
    let filtered = decoder.get_rpm();
    
    // Cálculo manual con Alpha = 85 (el que pusimos en el código):
    // Nuevo = (Old_1000 * 0.85) + (New_2000 * 0.15)
    // Nuevo = 850 + 300 = 1150 RPM
    // Comprobamos que esté en ese rango (amortiguado)
    assert!(filtered < 1500, "El filtro no está suavizando lo suficiente! Valor: {}", filtered);
    assert!(filtered > 1000, "El filtro no está subiendo nada! Valor: {}", filtered);
    
    // Opcional: Verificar valor exacto si no cambiamos el alpha
    // assert_eq!(filtered, 1150); 
}