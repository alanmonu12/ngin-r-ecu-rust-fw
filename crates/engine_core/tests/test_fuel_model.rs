use engine_core::fuel_model::SpeedDensity;

#[test]
fn test_calculo_inyeccion_estandar() {
    // Arrange: Motor 2000cc, 4 cil, inyectores 300cc/min
    let motor = SpeedDensity::new(2000.0, 4, 300.0);

    // Act: Calcular para 100kPa, 20°C, 100% VE
    let map = 100.0;
    let iat = 20.0;
    let ve = 100.0;
    let target_afr = 14.7;

    // 1. Masa de Aire
    let air_mass = motor.calculate_air_mass(map, iat, ve);
    
    // Verificamos que esté cerca de 0.59g (tolerancia pequeña por float)
    // 0.594 g aprox
    assert!(air_mass > 0.59 && air_mass < 0.60, "Masa de aire incorrecta: {}", air_mass);

    // 2. Pulse Width
    let pw_us = motor.calculate_pulse_width_us(air_mass, target_afr);

    // Debería ser aprox 10,900 us (10.9ms)
    // Damos un margen de +/- 200us por redondeos de constantes
    assert!(pw_us > 10_700 && pw_us < 11_100, "PW incorrecto: {} us", pw_us);
}

#[test]
fn test_ve_effect() {
    let motor = SpeedDensity::new(2000.0, 4, 300.0);
    
    // Si bajamos la VE al 50%, el combustible debería ser exactamente la mitad
    let air_100 = motor.calculate_air_mass(100.0, 20.0, 100.0);
    let air_50 = motor.calculate_air_mass(100.0, 20.0, 50.0);

    assert!((air_50 - (air_100 / 2.0)).abs() < 0.001);
}