import fs from "fs";

export const devpair = Uint8Array.from(
  JSON.parse(fs.readFileSync("/home/chizor/.config/solana/id.json", "utf-8"))
);

export const devpair2 = Uint8Array.from(
  JSON.parse(fs.readFileSync("/home/chizor/.config/solana/dev.json", "utf-8"))
);

export const devpair3 = Uint8Array.from(
  JSON.parse(
    fs.readFileSync("/home/chizor/.config/solana/devnet.json", "utf-8")
  )
);
