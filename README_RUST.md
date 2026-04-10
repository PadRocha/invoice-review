# invrev en Rust

Migración idiomática del CLI `invoice-review` desde Deno + TypeScript hacia
Rust, preservando el comportamiento observable del proyecto original.

## Qué preserva

- Factura: clave en columna `C`, precio en columna `E`
- Sistema: clave en columna `A`, precio en columna `E`
- Múltiples archivos de sistema respetando el orden recibido
- Coincidencia múltiple cuando la primera fuente que contiene la clave la repite
- Descuentos porcentuales sucesivos sin redondeo intermedio
- Sensitivity aplicada sobre el resultado porcentual final
- Salida en texto y JSON
- Errores y mensajes principales en español

## Arquitectura

```text
cli -> commands -> application -> domain -> infrastructure
```

```text
src/
  main.rs
  lib.rs
  cli/
  commands/
  application/
  domain/
    comparison/
    spreadsheet/
  infrastructure/
    spreadsheet/
    filesystem/
    report/
  shared/
```

## Uso

```bash
cargo run -- --invoice ./47088.xls --system ./EAG.xlsx
cargo run -- -i ./47088.xls -s ./EAG.xlsx --discount 19 --discount 12
cargo run -- -i ./47088.xls -s ./EAG.xlsx --sensitivity -1
cargo run -- -i ./47088.xls -s ./EAG.xlsx -o ./reporte.txt --json ./reporte.json
```

## Pruebas

```bash
cargo test
```

## Mapeo arquitectónico

- `main.ts` -> `src/main.rs` + `src/lib.rs`
- `cli/` -> `src/cli/`
- `controllers/` -> `src/commands/` + `src/application/`
- `core/comparison/` -> `src/domain/comparison/`
- `core/spreadsheet/` -> `src/domain/spreadsheet/` +
  `src/infrastructure/spreadsheet/`
- `services/filesystem/` -> `src/infrastructure/filesystem/`
- `report.service.ts` -> `src/infrastructure/report/text_report.rs`
- `interfaces/` y `models/` -> `struct` y `enum` en `domain/comparison/types.rs`
- `utils/` -> `src/shared/utils/`
- `errors/` -> `src/shared/errors.rs`

## Ajustes de diseño

- No se usó `clap` porque la CLI original tiene precedencias y mensajes muy
  específicos. Un parser manual conserva mejor la compatibilidad observable.
- La lectura de Excel se movió a infraestructura usando `calamine`.
- El dominio quedó puro y testeable sin depender de IO.
- Se mantuvo la lógica real del código fuente original, incluso donde difiere de
  algunas notas del README de Deno.
