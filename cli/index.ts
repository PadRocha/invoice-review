import { CliError } from "@errors";
import type {
  CliOptions,
  ReviewCliOptions,
  ValidReviewCliOptions,
} from "./types.ts";

export type {
  CliOptions,
  HelpCliOptions,
  ReviewCliOptions,
  ValidReviewCliOptions,
} from "./types.ts";

export function parseCliArgs(args: string[]): CliOptions {
  if (args.length === 0 || args.includes("--help") || args.includes("-h")) {
    return {
      command: "help",
    };
  }

  const options: ReviewCliOptions = {
    system_paths: [],
  };

  for (let index = 0; index < args.length; index += 1) {
    const current_arg = args[index];

    switch (current_arg) {
      case "--invoice":
      case "-i": {
        options.invoice_path = args[index + 1];
        index += 1;
        break;
      }
      case "--system":
      case "-s": {
        const next_arg = args[index + 1];
        if (!next_arg || next_arg.startsWith("-")) {
          throw new CliError("`--system` requiere al menos una ruta.");
        }

        options.system_paths.push(next_arg);
        index += 1;
        break;
      }
      case "--out":
      case "-o": {
        options.output_path = args[index + 1];
        index += 1;
        break;
      }
      case "--json": {
        options.json_path = args[index + 1];
        index += 1;
        break;
      }
      case "--sensitivity": {
        options.sensitivity = parseSensitivityValue(args[index + 1]);
        index += 1;
        break;
      }
      default: {
        if (current_arg.startsWith("--system=")) {
          const value = current_arg.slice("--system=".length);
          if (value) {
            options.system_paths.push(...splitSystemPaths(value));
            break;
          }
        }

        if (current_arg.startsWith("-s=")) {
          const value = current_arg.slice(3);
          if (value) {
            options.system_paths.push(...splitSystemPaths(value));
            break;
          }
        }

        if (current_arg.startsWith("--sensitivity=")) {
          const value = current_arg.slice("--sensitivity=".length);
          options.sensitivity = parseSensitivityValue(value);
          break;
        }

        if (current_arg.startsWith("-")) {
          throw new CliError(`Argumento no reconocido: ${current_arg}`);
        }

        throw new CliError(
          `Argumento no reconocido: ${current_arg}. Esta CLI no usa subcomandos. Usa directamente -i y -s.`,
        );
      }
    }
  }

  return options;
}

export function validateCliOptions(
  options: ReviewCliOptions,
): asserts options is ValidReviewCliOptions {
  if (!options.invoice_path) {
    throw new CliError(
      "Debes indicar la factura con `--invoice <ruta>` o `-i <ruta>`.",
    );
  }

  if (options.system_paths.length === 0) {
    throw new CliError(
      "Debes indicar al menos un archivo del sistema con `--system <ruta>` o `-s <ruta>`.",
    );
  }
}

export function buildHelpMessage(): string {
  return [
    "invrev",
    "",
    "Uso:",
    "  invrev --invoice ./47088.xls --system ./EAG.xlsx",
    "  invrev --invoice ./47088.xls --system ./EAG.xlsx --system ./ACC.xlsx",
    "  invrev --invoice ./47088.xls --system ./EAG.xlsx --sensitivity -1",
    "  invrev -i ./47088.xls -s ./EAG.xlsx -o ./reporte.txt --json ./reporte.json",
    "",
    "Desarrollo:",
    "  deno run --allow-read --allow-write main.ts -i ./47088.xls -s ./EAG.xlsx",
    "",
    "Opciones:",
    "  -i, --invoice <ruta>    Archivo principal de factura",
    "  -s, --system <ruta>     Archivo del sistema. Puede repetirse",
    "      --system=a,b,c      Variante compacta separada por comas",
    "  -o, --out <ruta>        Exporta el reporte en texto",
    "      --json <ruta>       Exporta el reporte en JSON",
    "      --sensitivity <n>   Oculta diferencias en el rango (n, 0] si n es negativo",
    "  -h, --help              Muestra esta ayuda",
    "",
    "Reglas fijas:",
    "  - Factura: clave en columna C, precio en columna E",
    "  - Sistema: clave en columna A, precio en columna E",
    "  - Fórmula de variación: (precio_factura / precio_sistema) * 100 - 100",
  ].join("\n");
}

function splitSystemPaths(value: string): string[] {
  return value
    .split(",")
    .map((entry) => entry.trim())
    .filter(Boolean);
}

function parseSensitivityValue(raw_value: string | undefined): number {
  const sensitivity = Number(raw_value);

  if (!Number.isFinite(sensitivity) || sensitivity >= 0) {
    throw new CliError(
      "`--sensitivity` requiere un valor numérico negativo, por ejemplo `--sensitivity -1`.",
    );
  }

  return sensitivity;
}
