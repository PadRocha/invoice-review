import { ensureDir } from "@std/fs";
import { dirname } from "@std/path";

export async function ensureParentDirectory(path: string): Promise<void> {
  const parent_path = dirname(path);
  await ensureDir(parent_path);
}
