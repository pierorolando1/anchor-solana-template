import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { AnchorTemplate } from '../target/types/anchor_template';

describe('anchor-template', () => {

  const provider = anchor.Provider.env()
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);
 
  //@ts-ignore
  const program = anchor.workspace.AnchorTemplate as Program<AnchorTemplate>;

  const counterAccount = anchor.web3.Keypair.generate()

  it('Is initialized!', async () => {
    const tx = await program.rpc.initialize({
      accounts: {
        counterAccount: counterAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId
      },
      signers: [counterAccount] // DONT FORGET THIS SHIT
    });
    console.log("Your transaction signature", tx);
  });
});
