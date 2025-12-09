
# Ngin-R: ECU Aftermarket Mexicana (STM32H7 + Rust)

![Language](https://img.shields.io/badge/Language-Rust-orange.svg)
![Architecture](https://img.shields.io/badge/Arch-ARM%20Cortex--M7-blue)
![Framework](https://img.shields.io/badge/Framework-RTIC%202.0-green)
![Status](https://img.shields.io/badge/Status-Prototyping-yellow)

**Ngin-R** es un proyecto de ingeniería automotriz open-source enfocado en desarrollar una Unidad de Control de Motor (ECU) de alto rendimiento, segura y determinista.

El proyecto está diseñado separando estrictamente la lógica de negocio (física del motor) de la implementación de hardware, permitiendo simulación en PC y validación modular.

---

## 🏗 Arquitectura del Sistema

El proyecto utiliza un **Rust Workspace** para garantizar la modularidad y testabilidad.

```text
ngin-r-rust-rtic/
├── Cargo.toml              # Configuración del Workspace
├── firmware/               # APLICACIÓN FINAL (RTIC)
│   ├── src/main.rs         # Orquestador de tareas y tiempos
│   └── memory.x            # Mapa de memoria (Flash/DTCM/RAM)
│
├── crates/                 # LIBRERÍAS (Componentes)
│   ├── ecu_traits/         # CONTRATOS: Interfaces abstractas (Agnóstico)
│   ├── engine_core/        # CEREBRO: Física, Tablas VE, Lógica (Pure Rust, Testable)
│   └── bsp/                # MÚSCULO: Drivers de Hardware (STM32H7, HAL)
│       ├── src/            # Implementación de Traits
│       └── examples/       # Pruebas de integración Hardware (HIL)
│
└── tests/                  # Pruebas de Sistema (Simulación completa)
```

## ⚡ Hardware Soportado

Microcontrolador: STM32H750VBT6 (ARM Cortex-M7 @ 480MHz, FPU Doble Precisión).

Memoria:

Flash: 128KB (Código).

DTCM RAM: 128KB (Stack y variables críticas de tiempo real).

AXI SRAM: 512KB (Buffers y datos generales).

### Pinout Actual (Dev Board)

TBD

## 🚀 Guía de Inicio Rápido

Requisitos Previos

    Rust Toolchain (nightly o stable).

    Target ARM: rustup target add thumbv7em-none-eabihf

    Herramientas Embedded: cargo install cargo-embed flip-link probe-rs-tools

    Drivers ST-Link instalados.

1. Clonar y Preparar

``` bash
git clone [https://github.com/tu-usuario/ngin-r-rust-rtic.git](https://github.com/tu-usuario/ngin-r-rust-rtic.git)
cd ngin-r-rust-rtic
```

2. Validar Hardware (BSP Tests)

Para probar que el hardware (ej. Inyectores) funciona sin correr toda la lógica de la ECU, ejecutamos los ejemplos aislados del BSP. Nota: Esto compila y flashea un binario pequeño específico para pruebas.

``` bash
# Prueba de activación de inyector (manual loop)
cargo run -p bsp --example test_injector
```

3. Ejecutar la ECU (Firmware)

Para correr el sistema operativo completo (RTIC) con el orquestador de tareas.

``` bash
# Compilar, flashear y abrir consola de logs RTT
cd firmware
cargo embed --release
```

## 🧪 Estrategia de Testing

TBD

## 🛠 Estado del Proyecto

[x] Toolchain: Configuración de Workspace, compilación cruzada y mapas de memoria.

[x] RTIC: Integración básica y "Hello World" (Blinky).

[x] BSP Driver: Abstracción de Inyectores (Modelo Fire and Forget).

[ ] Engine Core: Implementación de tablas VE y cálculo de PW.

[ ] Decoder: Lectura de rueda fónica (60-2 / 36-1).

[ ] Communication: Protocolo de calibración (Serial/CAN).

## 📄 Licencia

Este proyecto está licenciado bajo MIT / Apache-2.0.