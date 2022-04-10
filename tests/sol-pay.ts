import * as anchor from "@project-serum/anchor";
import {Program} from "@project-serum/anchor";
import {SolPay} from "../target/types/sol_pay";
import {
    createAssociatedTokenAccount,
    createMint, getAccount,
    getAssociatedTokenAddress,
    getOrCreateAssociatedTokenAccount, mintTo, mintToChecked,
    TOKEN_PROGRAM_ID, transfer
} from "@solana/spl-token";
import {clusterApiUrl, Connection, Keypair, LAMPORTS_PER_SOL} from '@solana/web3.js';


describe("sol-pay", () => {

    // Configure the client to use the local cluster.
    const provider = anchor.Provider.local();
    anchor.setProvider(provider);
    const program = anchor.workspace.SolPay as Program<SolPay>;
    
    it("Transfer token", async () => {
        const accountStoringAmount = anchor.web3.Keypair.generate()
        const receiving = anchor.web3.Keypair.generate()
        const giving = anchor.web3.Keypair.generate()
        const mintAuthority = anchor.web3.Keypair.generate()
        
        
        await provider.connection.confirmTransaction(
            await provider.connection.requestAirdrop(
                receiving.publicKey,
                LAMPORTS_PER_SOL * 10
            ),
            'confirmed'
        )

        await provider.connection.confirmTransaction(
            await provider.connection.requestAirdrop(
                giving.publicKey,
                LAMPORTS_PER_SOL * 10
            ),
            'confirmed'
        )

        const mint = await createMint(
            provider.connection,
            receiving,
            mintAuthority.publicKey,
            null,
            9,
            mintAuthority,
            null,
            TOKEN_PROGRAM_ID
        )
        
        const receivingTokenAccount = await createAssociatedTokenAccount(
            provider.connection,
            receiving,
            mint,
            receiving.publicKey
        )

        const givingTokenAccount = await createAssociatedTokenAccount(
            provider.connection,
            giving,
            mint,
            giving.publicKey
        )

        await mintTo(
            provider.connection,
            giving,
            mint,
            givingTokenAccount,
            mintAuthority,
            100000
        )

        const txInitialise = await program.rpc.initialize(
            new anchor.BN(123),{
                accounts: {
                    receiver: accountStoringAmount.publicKey,
                    user: receiving.publicKey,
                    receiverTokenAccount: receivingTokenAccount,
                    tokenProgram: mint,
                    systemProgram: anchor.web3.SystemProgram.programId,
                }, signers: [
                    receiving, accountStoringAmount
                ]
            });
        console.log(txInitialise)
        
        const account = await program.account.receiverDetails.fetch(accountStoringAmount.publicKey)
        const txTransfer = await program.rpc.transfer(
            new anchor.BN(account.amount.toString()),
            {
                accounts: {
                    sender: giving.publicKey,
                    receiver: accountStoringAmount.publicKey,
                    receiverTokenAccount: account.tokenAccount,
                    senderTokenAccount: givingTokenAccount,
                    tokenProgram: TOKEN_PROGRAM_ID,
                },
                signers: [
                    giving
                ]
            }
        )
        
        const givingA = await getAccount(
            provider.connection,
            givingTokenAccount
        )
        
        const receivingA = await getAccount(
            provider.connection,
            receivingTokenAccount
        )

        console.log(
            givingA
        )

        console.log(
            receivingA
        )
    })
    
    it("Transfer lamports", async () => {
        const receiving = anchor.web3.Keypair.generate()
        const giving = anchor.web3.Keypair.generate()
        const account = anchor.web3.Keypair.generate()
        console.log(giving.publicKey.toBase58())
        console.log(receiving.publicKey.toBase58())

        await provider.connection.confirmTransaction(
            await provider.connection.requestAirdrop(
                receiving.publicKey,
                LAMPORTS_PER_SOL * 10
            ),
            'confirmed'
        )

        await provider.connection.confirmTransaction(
            await provider.connection.requestAirdrop(
                giving.publicKey,
                LAMPORTS_PER_SOL * 10
            ),
            'confirmed'
        )
        
        const txInitialise = await program.rpc.initializeLamports(
            new anchor.BN(LAMPORTS_PER_SOL * 2),
            {
                accounts: {
                    receiver: account.publicKey,
                    user: receiving.publicKey,
                    systemProgram: anchor.web3.SystemProgram.programId
                }, 
                signers: [receiving, account]
            }
        )
        console.log(txInitialise)
        
        const accountDetails = await program.account.receiverLamportDetails.fetch(account.publicKey)
        const receiverAccount = await provider.connection.getAccountInfo(receiving.publicKey)
        /*console.log(receiverAccount.data)
        const txPay = await program.rpc.transferLamports(
            new anchor.BN(10000000),
            {
                accounts: {
                    receiver: account.publicKey,
                    getter: accountDetails.receiverPubkey,
                    sender: giving.publicKey,
                },
                signers: [giving]
            }
        )*/
        const transaction = new anchor.web3.Transaction().add(
            anchor.web3.SystemProgram.transfer({
                fromPubkey: giving.publicKey,
                toPubkey: accountDetails.receiverPubkey,
                lamports: accountDetails.amount.toNumber()
            })
        )
        await anchor.web3.sendAndConfirmTransaction(provider.connection, transaction, [giving], {
            commitment: "confirmed"
        })

        console.log(giving)
        console.log(receiving)

        const gaccount = await provider.connection.getAccountInfo(giving.publicKey)
        const rAccount = await provider.connection.getAccountInfo(receiving.publicKey)

        console.log("Giving account: ", gaccount.lamports)
        console.log("Receiving account: ", rAccount.lamports)
    })

    it("Transfer token", async () => {
        const accountStoringAmount = anchor.web3.Keypair.generate()
        const receiving = anchor.web3.Keypair.generate()
        const giving = anchor.web3.Keypair.generate()
        const mintAuthority = anchor.web3.Keypair.generate()


        await provider.connection.confirmTransaction(
            await provider.connection.requestAirdrop(
                receiving.publicKey,
                LAMPORTS_PER_SOL * 10
            ),
            'confirmed'
        )

        await provider.connection.confirmTransaction(
            await provider.connection.requestAirdrop(
                giving.publicKey,
                LAMPORTS_PER_SOL * 10
            ),
            'confirmed'
        )

        const mint = await createMint(
            provider.connection,
            receiving,
            mintAuthority.publicKey,
            null,
            9,
            mintAuthority,
            null,
            TOKEN_PROGRAM_ID
        )

        const receivingTokenAccount = await createAssociatedTokenAccount(
            provider.connection,
            receiving,
            mint,
            receiving.publicKey
        )

        const givingTokenAccount = await createAssociatedTokenAccount(
            provider.connection,
            giving,
            mint,
            giving.publicKey
        )

        await mintTo(
            provider.connection,
            giving,
            mint,
            givingTokenAccount,
            mintAuthority,
            100000
        )

        const txInitialise = await program.rpc.initialize(
            new anchor.BN(123),{
                accounts: {
                    receiver: accountStoringAmount.publicKey,
                    user: receiving.publicKey,
                    receiverTokenAccount: receivingTokenAccount,
                    tokenProgram: mint,
                    systemProgram: anchor.web3.SystemProgram.programId,
                }, signers: [
                    receiving, accountStoringAmount
                ]
            });
        console.log(txInitialise)

        const account = await program.account.receiverDetails.fetch(accountStoringAmount.publicKey)
        const txTransfer = await program.rpc.transfer(
            new anchor.BN(account.amount.toString()),
            {
                accounts: {
                    sender: giving.publicKey,
                    receiver: accountStoringAmount.publicKey,
                    receiverTokenAccount: account.tokenAccount,
                    senderTokenAccount: givingTokenAccount,
                    tokenProgram: TOKEN_PROGRAM_ID,
                },
                signers: [
                    giving
                ]
            }
        )

        const givingA = await getAccount(
            provider.connection,
            givingTokenAccount
        )

        const receivingA = await getAccount(
            provider.connection,
            receivingTokenAccount
        )

        console.log(
            givingA
        )

        console.log(
            receivingA
        )
    })
});
