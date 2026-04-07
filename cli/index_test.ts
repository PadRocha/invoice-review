import {
  assert,
  assertEquals,
  assertStringIncludes,
  assertThrows,
} from "@std/assert";
import { CliError } from "@errors";
import {
  buildHelpMessage,
  buildVersionMessage,
  parseCliArgs,
  validateCliOptions,
} from "@cli";

Deno.test("parseCliArgs acepta invocación directa con flags", () => {
  const options = parseCliArgs([
    "-i",
    "./47088.xls",
    "-s",
    "./EAG.xlsx",
    "--system=./ACC.xlsx,./BSC.xlsx",
    "-d",
    "19",
    "--discount=12.5",
    "--sensitivity",
    "-1",
    "-o",
    "./reporte.txt",
    "--json",
    "./reporte.json",
  ]);

  assertEquals(options, {
    invoice_path: "./47088.xls",
    system_paths: ["./EAG.xlsx", "./ACC.xlsx", "./BSC.xlsx"],
    discounts: [19, 12.5],
    sensitivity: -1,
    output_path: "./reporte.txt",
    json_path: "./reporte.json",
  });
});

Deno.test("validateCliOptions exige la factura", () => {
  assertThrows(
    () => validateCliOptions({ system_paths: ["./EAG.xlsx"], discounts: [] }),
    CliError,
    "Debes indicar la factura con `--invoice <ruta>` o `-i <ruta>`.",
  );
});

Deno.test("parseCliArgs acepta --version", () => {
  const options = parseCliArgs(["--version"]);

  assertEquals(options, {
    command: "version",
  });
});

Deno.test("parseCliArgs acepta -v", () => {
  const options = parseCliArgs(["-v"]);

  assertEquals(options, {
    command: "version",
  });
});

Deno.test("parseCliArgs acepta sensitivity en sintaxis compacta", () => {
  const options = parseCliArgs([
    "--invoice",
    "./47088.xls",
    "--system",
    "./EAG.xlsx",
    "--discount",
    "5.5",
    "--sensitivity=-0.5",
  ]);

  assertEquals(options, {
    invoice_path: "./47088.xls",
    system_paths: ["./EAG.xlsx"],
    discounts: [5.5],
    sensitivity: -0.5,
  });
});

Deno.test("parseCliArgs acepta descuentos repetibles y conserva el orden", () => {
  const options = parseCliArgs([
    "--invoice",
    "./47088.xls",
    "--system",
    "./EAG.xlsx",
    "-d",
    "19",
    "-d",
    "12",
    "--discount",
    "5.5",
  ]);

  assertEquals(options, {
    invoice_path: "./47088.xls",
    system_paths: ["./EAG.xlsx"],
    discounts: [19, 12, 5.5],
  });
});

Deno.test("parseCliArgs rechaza descuentos no numéricos", () => {
  assertThrows(
    () =>
      parseCliArgs([
        "--invoice",
        "./47088.xls",
        "--system",
        "./EAG.xlsx",
        "--discount",
        "abc",
      ]),
    CliError,
    "`--discount` requiere un porcentaje numérico mayor a 0 y menor a 100, por ejemplo `--discount 19` o `--discount 5.5`.",
  );
});

Deno.test("parseCliArgs rechaza descuentos fuera de rango razonable", () => {
  assertThrows(
    () =>
      parseCliArgs([
        "--invoice",
        "./47088.xls",
        "--system",
        "./EAG.xlsx",
        "--discount",
        "0",
      ]),
    CliError,
    "`--discount` requiere un porcentaje numérico mayor a 0 y menor a 100, por ejemplo `--discount 19` o `--discount 5.5`.",
  );
});

Deno.test("parseCliArgs rechaza sensitivity no negativa", () => {
  assertThrows(
    () =>
      parseCliArgs([
        "--invoice",
        "./47088.xls",
        "--system",
        "./EAG.xlsx",
        "--sensitivity",
        "0",
      ]),
    CliError,
    "`--sensitivity` requiere un valor numérico negativo, por ejemplo `--sensitivity -1`.",
  );
});

Deno.test("parseCliArgs rechaza `review` como pseudo-subcomando", () => {
  assertThrows(
    () =>
      parseCliArgs([
        "review",
        "-i",
        "./47088.xls",
        "-s",
        "./EAG.xlsx",
      ]),
    CliError,
    "Argumento no reconocido: review. Esta CLI no usa subcomandos. Usa directamente -i y -s.",
  );
});

Deno.test("parseCliArgs rechaza argumentos posicionales sueltos", () => {
  assertThrows(
    () => parseCliArgs(["foo"]),
    CliError,
    "Argumento no reconocido: foo. Esta CLI no usa subcomandos. Usa directamente -i y -s.",
  );
});

Deno.test("buildVersionMessage muestra nombre y versión", () => {
  assertEquals(buildVersionMessage(), "invrev 0.2.0");
});

Deno.test("buildHelpMessage documenta la invocación directa", () => {
  const help_message = buildHelpMessage();

  assertStringIncludes(
    help_message,
    "invrev -i ./47088.xls -s ./EAG.xlsx -o ./reporte.txt --json ./reporte.json",
  );
  assertStringIncludes(
    help_message,
    "--sensitivity <n>",
  );
  assertStringIncludes(help_message, "--discount <n>");
  assertStringIncludes(help_message, buildVersionMessage());
  assertStringIncludes(help_message, "-v, --version");
  assertStringIncludes(help_message, "precio_usado / precio_sistema");
  assert(!help_message.includes("main.ts review"));
  assert(!help_message.includes("Compatibilidad"));
  assert(!help_message.includes("deno task review"));
});
