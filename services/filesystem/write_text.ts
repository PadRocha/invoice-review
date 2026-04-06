import { ensureParentDirectory } from "./ensure_parent.ts";
import type { WriteTextOptions } from "./types.ts";

export async function writeTextFile(
  path: string,
  content: string,
  options: WriteTextOptions = {},
): Promise<void> {
  await ensureParentDirectory(path);

  const final_content = options.ensure_trailing_newline === false
    ? content
    : content.endsWith("\n")
    ? content
    : `${content}\n`;

  await Deno.writeTextFile(path, final_content);
}
