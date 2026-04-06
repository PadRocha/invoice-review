export function registerSignalHandlers(): void {
  const abort_controller = new AbortController();

  const handler = () => {
    console.error("\nProceso interrumpido.");
    abort_controller.abort();
    Deno.exit(130);
  };

  try {
    Deno.addSignalListener("SIGINT", handler);
    Deno.addSignalListener("SIGTERM", handler);
  } catch {
    // Algunos entornos no soportan señales. No pasa nada.
  }
}
