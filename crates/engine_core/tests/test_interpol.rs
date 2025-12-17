use engine_core::tables::Table3D;

#[test]
fn test_exact_interpolation() {
    // Una tabla simple de 2x2
    // Eje X (RPM): 1000, 2000
    // Eje Y (MAP): 50, 100
    // Datos:
    //      1000rpm  2000rpm
    // 50kPa:  10       20
    // 100kPa: 30       40
    let x_axis = [1000.0, 2000.0];
    let y_axis = [50.0, 100.0];
    let data = [
        [10.0, 20.0],
        [30.0, 40.0],
    ];

    let table = Table3D::new(x_axis, y_axis, data);

    // Caso 1: Punto exacto (1000 RPM, 50 MAP) -> Debería ser 10
    assert_eq!(table.interpolate(1000.0, 50.0), 10.0);

    // Caso 2: Punto exacto (2000 RPM, 100 MAP) -> Debería ser 40
    assert_eq!(table.interpolate(2000.0, 100.0), 40.0);
}

#[test]
fn test_interpolacion_mitad() {
    let x_axis = [1000.0, 2000.0];
    let y_axis = [50.0, 100.0];
    let data = [
        [10.0, 20.0], 
        [30.0, 40.0], 
    ];
    let tabla = Table3D::new(x_axis, y_axis, data);

    // Caso: Justo en el centro (1500 RPM, 75 MAP)
    // En X=1500 (mitad), fila 50kpa es 15.
    // En X=1500 (mitad), fila 100kpa es 35.
    // Entre 15 y 35 a la mitad (75kpa) es 25.
    let val = tabla.interpolate(1500.0, 75.0);
    assert!((val - 25.0).abs() < 0.001); // Comparación float segura
}

#[test]
fn test_clamping_fuera_de_rango() {
    let x_axis = [1000.0, 2000.0];
    let y_axis = [50.0, 100.0];
    let data = [
        [10.0, 20.0], 
        [30.0, 40.0], 
    ];
    let tabla = Table3D::new(x_axis, y_axis, data);

    // Caso: 5000 RPM (Fuera de rango por arriba) -> Debería usar columna 2000
    // MAP 50 -> Valor 20
    assert_eq!(tabla.interpolate(5000.0, 50.0), 20.0);
}
