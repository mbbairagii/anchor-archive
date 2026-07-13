import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SplTinyTokenLab } from "../target/types/spl_tiny_token_lab";

describe("spl-tiny-token-lab", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .splTinyTokenLab as Program<SplTinyTokenLab>;

  it("loads the workspace", async () => {
    console.log("Program ID:", program.programId.toBase58());
  });
});