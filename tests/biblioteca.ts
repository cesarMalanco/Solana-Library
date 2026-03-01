import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Biblioteca } from "../target/types/biblioteca";
import { assert } from "chai";

describe("biblioteca-devnet", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Biblioteca as Program<Biblioteca>;
  const owner = provider.wallet;

  let bibliotecaPda: anchor.web3.PublicKey;

  it("1. Crea biblioteca", async () => {
    await program.methods
      .crearBiblioteca("Mi Biblioteca")
      .accounts({
        owner: owner.publicKey,
      })
      .rpc();

    bibliotecaPda = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("biblioteca"), owner.publicKey.toBuffer()],
      program.programId
    )[0];

    const cuenta = await program.account.biblioteca.fetch(bibliotecaPda);
    assert.equal(cuenta.nombre, "Mi Biblioteca");
  });

  it("2. Agrega libro", async () => {
    await program.methods
      .agregarLibro("El Quijote", 300)
      .accounts({
        owner: owner.publicKey,
        biblioteca: bibliotecaPda, // 👈 IMPORTANTE
      })
      .rpc();

    const cuenta = await program.account.biblioteca.fetch(bibliotecaPda);
    assert.equal(cuenta.libros.length, 1);
    assert.equal(cuenta.libros[0].nombre, "El Quijote");
  });

  it("3. Alterna estado", async () => {
    await program.methods
      .alternarEstado("El Quijote")
      .accounts({
        owner: owner.publicKey,
        biblioteca: bibliotecaPda,
      })
      .rpc();

    const cuenta = await program.account.biblioteca.fetch(bibliotecaPda);
    assert.equal(cuenta.libros[0].disponible, false);
  });

  it("4. Ver libros (solo ejecuta)", async () => {
    await program.methods
      .verLibros()
      .accounts({
        owner: owner.publicKey,
        biblioteca: bibliotecaPda,
      })
      .rpc();
  });

  it("5. Elimina libro", async () => {
    await program.methods
      .eliminarLibro("El Quijote")
      .accounts({
        owner: owner.publicKey,
        biblioteca: bibliotecaPda,
      })
      .rpc();

    const cuenta = await program.account.biblioteca.fetch(bibliotecaPda);
    assert.equal(cuenta.libros.length, 0);
  });
});