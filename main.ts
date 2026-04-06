import { reviewInvoiceController } from "@controllers";
import {
  buildHelpMessage,
  buildVersionMessage,
  parseCliArgs,
  validateCliOptions,
} from "@cli";
import { CliError } from "@errors";
import { registerSignalHandlers } from "./signals.ts";

if (import.meta.main) {
  registerSignalHandlers();

  try {
    const options = parseCliArgs(Deno.args);

    if ("command" in options) {
      if (options.command === "help") {
        console.log(buildHelpMessage());
      }

      if (options.command === "version") {
        console.log(buildVersionMessage());
      }

      Deno.exit(0);
    }

    validateCliOptions(options);
    const text_report = await reviewInvoiceController(options);
    console.log(text_report);
    Deno.exit(0);
  } catch (error) {
    if (error instanceof CliError) {
      console.error(`Error: ${error.message}`);
      console.error("");
      console.error(buildHelpMessage());
      Deno.exit(1);
    }

    if (error instanceof Error) {
      console.error(`Error: ${error.message}`);
      Deno.exit(1);
    }

    console.error("Error desconocido.");
    Deno.exit(1);
  }
}
