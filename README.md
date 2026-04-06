# invrev

CLI en Deno para revisar una factura contra uno o varios archivos de sistema.

## Qué hace

- Toma la factura como archivo principal.
- Busca cada clave de la columna `C` de la factura en la columna `A` de los
  archivos del sistema.
- Compara el precio de la columna `E` de factura contra la columna `E` del
  sistema.
- Calcula la variación porcentual con la fórmula:

```ts
(precio_factura / precio_sistema) * 100 - 100;
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
- Si hay varios archivos del sistema, se respetan en el orden recibido.
- Si una clave aparece más de una vez dentro del primer archivo del sistema que
  la contiene, se reporta como coincidencia múltiple.

## Notas

- La CLI lee la primera hoja de cada archivo.
- Las filas sin clave en la columna requerida se ignoran.
- Si una fila tiene clave pero no tiene precio válido en la columna requerida,
  la ejecución falla con un error descriptivo.
