import { WalletAdapterNetwork } from "@solana/wallet-adapter-base";
import { ConnectionProvider, WalletProvider, useConnection, useWallet } from "@solana/wallet-adapter-react";
import { WalletDisconnectButton, WalletModalProvider, WalletMultiButton } from "@solana/wallet-adapter-react-ui";

import "@solana/wallet-adapter-react-ui/styles.css"
import { PhantomWalletAdapter, SolongWalletAdapter } from "@solana/wallet-adapter-wallets";
import { LAMPORTS_PER_SOL, PublicKey, TransactionInstruction, TransactionMessage, VersionedTransaction, clusterApiUrl } from "@solana/web3.js";
import { useState } from "react";

const TOKEN_PROGRAM_ID= "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
const SPL_TOKEN_ADDR = "CEbaSH6yMvusuEVKrSqteorac3gfWMqA5dCewNFWZb8L"
const ATA_PUBKEY_KEY = "HczZGUWCBQCbjgaiQUkyJ713fUeSXeQ4uTNKdHhdqcVJ"


export default function App() {
  const wallets = [
    new SolongWalletAdapter(),
    new PhantomWalletAdapter(),
  ]
  const endpoint = clusterApiUrl(WalletAdapterNetwork.Devnet)
  
  return <ConnectionProvider endpoint={endpoint}>
    <WalletProvider wallets={wallets} autoConnect>
      <WalletModalProvider>
        <h2>SPL token Transfer Demo</h2>
        connect wallet:<br/>
        <WalletMultiButton/>
        <br/>
        <br/>
        Click the button below to disconnect current wallet:
        <WalletDisconnectButton/>
        <br/>
        <br/>
        <br/>
        <OperationPage/>
        </WalletModalProvider>
      </WalletProvider>
  </ConnectionProvider>
}


function OperationPage() {
  const { connection } = useConnection()
  const { publicKey, sendTransaction } = useWallet();

  const [balance, setBalance] = useState<number>()
  const onBalance = ()=>{
    connection.getTokenAccountBalance(new PublicKey(ATA_PUBKEY_KEY)).then((balance)=>{
      setBalance(balance.value.uiAmount!)
    })
  }

  const [amount, setAmount] = useState<number>()
  const onAmount = (e: React.ChangeEvent<HTMLInputElement>)=>{
    setAmount(parseInt(e.target.value) * LAMPORTS_PER_SOL)
  }

  const [toPublicKey, setToPublicKey] = useState<string>()
  const onToPublicKey = (e: React.ChangeEvent<HTMLInputElement>)=>{
    setToPublicKey(e.target.value)
  }

  const onTransfer = async ()=>{
    alert("... wait for transfer " + amount + " token(s) to " + toPublicKey)
    
    const txInstructions = [
      createTransactionInstruction(
        new PublicKey(TOKEN_PROGRAM_ID),
        new PublicKey(ATA_PUBKEY_KEY),
        new PublicKey(toPublicKey!),
        new PublicKey(publicKey!),
        amount!
      )
    ]

    const { 
      context: { slot: minContextSlot }, value: { blockhash } 
    } = await connection.getLatestBlockhashAndContext()

    const messageV0 = new TransactionMessage({
      payerKey: publicKey!,
      instructions: txInstructions,
      recentBlockhash: blockhash
    }).compileToV0Message()

    const trx = new VersionedTransaction(messageV0)
    const signature = await sendTransaction(trx, connection, { minContextSlot })
    console.log("signature: ", signature)
  }

  const createTransactionInstruction = (
    programId: PublicKey,
    source: PublicKey,
    destination: PublicKey,
    owner: PublicKey,
    amount: number
  )=>{
    const keys = [
      { pubkey: source, isSigner: false, isWritable: true },
      { pubkey: destination, isSigner: false, isWritable: true },
      { pubkey: owner, isSigner: true, isWritable: false }
    ]
    const data = Buffer.alloc(9)
    data.writeUInt8(3)
    data.writeBigInt64LE(BigInt(amount), 1)
    return new TransactionInstruction({ keys, programId, data })
  }
  
  return <div>
    <p>
    <strong>SPL Token addr:</strong><br/>
    {SPL_TOKEN_ADDR}
    </p>
    <label>Balance: {balance}</label><br/>
    <button onClick={ onBalance }>Query Balance</button>
    <br/>
    <br/>
    Transfer 
    <input onChange={ onAmount }/> (token amount)
    <br/>
    To 
    <input onChange={ onToPublicKey }/> (ATA public key)
    <br/>
    <button onClick={ onTransfer }>Transfer</button>
  </div>
}