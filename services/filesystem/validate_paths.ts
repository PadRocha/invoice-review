import { FileSystemError } from "@errors";

export async function validateReadableFile(path: string): Promise<void> {
  let file_info: Deno.FileInfo;

  try {
    file_info = await Deno.stat(path);
  } catch (error) {
    if (error instanceof Deno.errors.NotFound) {
      throw new FileSystemError(`No existe el archivo: ${path}`);
    }

    throw new FileSystemError(`No se pudo acceder al archivo: ${path}`);
  }

  if (!file_info.isFile) {
    throw new FileSystemError(`La ruta no es un archivo: ${path}`);
  }
}
