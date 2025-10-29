// prediction_market/tests/prediction_market.ts

import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PredictionMarket } from "../target/types/prediction_market";
import { assert } from "chai";

describe("prediction_market", () => {
  // Configura o cliente para usar o cluster local (localnet).
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.PredictionMarket as Program<PredictionMarket>;
  const authority = provider.wallet.publicKey;

  // Chaves para as contas que vamos criar
  const market = anchor.web3.Keypair.generate();
  const yesTokenMint = anchor.web3.Keypair.generate();
  const noTokenMint = anchor.web3.Keypair.generate();

  it("Cria um novo mercado (create_market)", async () => {
    // Argumentos para a nossa instrução
    const description = "O Brasil vai ganhar a Copa de 2026?";
    // Timestamp para daqui a 1 hora (em segundos)
    const resolutionTimestamp = new anchor.BN(Math.floor(Date.now() / 1000) + 3600);

    // Chama a instrução `createMarket`
    const tx = await program.methods
      .createMarket(description, resolutionTimestamp)
      .accounts({
        // ✅ --- CORREÇÃO AQUI ---
        // Apenas as contas que NÃO são Programas ou Sysvars
        // são necessárias aqui.
        market: market.publicKey,
        authority: authority,
        yesTokenMint: yesTokenMint.publicKey,
        noTokenMint: noTokenMint.publicKey,
        // As contas 'tokenProgram', 'systemProgram' e 'rent'
        // são resolvidas e adicionadas automaticamente pelo Anchor.
      })
      .signers([market, yesTokenMint, noTokenMint]) // Os 'signers' são necessários para contas 'init'
      .rpc();

    console.log("Assinatura da transação (create_market):", tx);

    // Agora, vamos verificar se a conta foi criada corretamente
    const marketAccount = await program.account.market.fetch(market.publicKey);

    // Verifica os dados
    assert.ok(marketAccount.authority.equals(authority));
    assert.equal(marketAccount.description, description);
    assert.ok(marketAccount.resolutionTimestamp.eq(resolutionTimestamp));
    assert.equal(marketAccount.isResolved, false);
    assert.ok(marketAccount.yesTokenMint.equals(yesTokenMint.publicKey));
    assert.ok(marketAccount.noTokenMint.equals(noTokenMint.publicKey));
  });
});