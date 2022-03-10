import fs from "fs";
import path from "path";

export const devpair = Uint8Array.from(
  JSON.parse(fs.readFileSync("/home/chizor/.config/solana/dev.json", "utf-8"))
);
