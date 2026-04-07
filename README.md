# invrev

CLI en Deno para revisar una factura contra uno o varios archivos de sistema.

## Qué hace

- Toma la factura como archivo principal.
- Busca cada clave de la columna `C` de la factura en la columna `A` de los
  archivos del sistema.
- Compara el precio de la columna `E` de factura contra la columna `E` del
  sistema.
- Si se indican descuentos con `-d` o `--discount`, aplica descuentos
  porcentuales sucesivos al precio de factura antes de compararlo.
- Calcula la variación porcentual con la fórmula:

```ts
(precio_usado / precio_sistema) * 100 - 100;
```

- Reporta:
  - precios incorrectos
  - claves no encontradas
  - coincidencias múltiples
  - resumen final

## Uso

```bash
invrev --invoice ./47088.xls --system ./EAG.xlsx
```

Invocación directa del binario:

```bash
invrev -i ./47088.xls -s ./EAG.xlsx
```

Invocación directa del script en desarrollo:

```bash
deno run --allow-read --allow-write main.ts -i ./47088.xls -s ./EAG.xlsx
```

Con varios archivos del sistema:

```bash
invrev --invoice ./47088.xls --system ./EAG.xlsx --system ./ACC.xlsx
```

Aplicando un descuento porcentual:

```bash
invrev --invoice ./47088.xls --system ./EAG.xlsx --discount 19
```

Aplicando descuentos sucesivos:

```bash
invrev --invoice ./47088.xls --system ./EAG.xlsx --discount 19 --discount 12
```

Ocultando diferencias pequeñas con `--sensitivity`:

```bash
invrev --invoice ./47088.xls --system ./EAG.xlsx --sensitivity -1
```

Combinando descuentos con `--sensitivity`:

```bash
invrev --invoice ./47088.xls --system ./EAG.xlsx \
  --discount 19 \
  --discount 12 \
  --sensitivity -1
```

Exportando reporte en texto y JSON:

```bash
invrev --invoice ./47088.xls --system ./EAG.xlsx \
  --out ./reporte.txt \
  --json ./reporte.json
```

## Reglas fijas

- Factura:
  - clave en columna `C`
  - precio en columna `E`
- Sistema:
  - clave en columna `A`
  - precio en columna `E`
- Si no se pasan descuentos, la comparación usa el precio original de factura.
- Si se pasan descuentos, la comparación usa el precio ajustado final.
- Los descuentos porcentuales se aplican en secuencia sobre el resultado del
  anterior.
- Si hay varios archivos del sistema, se respetan en el orden recibido.
- Si una clave aparece más de una vez dentro del primer archivo del sistema que
  la contiene, se reporta como coincidencia múltiple.

## Descuentos

- Usa `-d <n>` o `--discount <n>`.
- La opción puede repetirse.
- Acepta valores numéricos como `19`, `12` o `5.5`.
- Cada descuento debe ser mayor a `0` y menor a `100`.
- No se suman porcentajes; cada descuento se aplica sobre el resultado del
  anterior.
- No se redondea entre descuentos; el redondeo se deja para la salida del
  reporte.

Ejemplo:

```ts
530 * (1 - 19 / 100) * (1 - 12 / 100) === 377.784;
```

## Notas

- La CLI lee la primera hoja de cada archivo.
- Las filas sin clave en la columna requerida se ignoran.
- Si una fila tiene clave pero no tiene precio válido en la columna requerida,
  la ejecución falla con un error descriptivo.
- `--sensitivity <n>` oculta diferencias en el rango `(n, 0]` cuando `n` es
  negativo. Ejemplo: `--sensitivity -1`.
- Cuando hay descuentos, `--sensitivity` se aplica sobre la variación calculada
  con el precio ajustado.
