import React, { useState, useRef } from "react";
import { 
  Keypair,
  Connection,
  PublicKey,
  LAMPORTS_PER_SOL,
  SystemProgram,
  TransactionMessage,
  VersionedTransaction
} from "@solana/web3.js"

import * as buffer from "buffer"
window.Buffer = buffer.Buffer

const ENDPOINT_URL = "https://api.devnet.solana.com";

function App() {
  const [privateKey, setPrivateKey] = useState("")
  const [publicKey, setPublicKey] = useState("")
  const [balance, setBalance] = useState(0)
  const [toAmount, setToAmount] = useState<number>()
  const [account, setAccount] = useState("")
  const keyPair = useRef<Keypair>()

  const connection = new Connection(ENDPOINT_URL)


  const onPrivateKey = (e: React.ChangeEvent<HTMLInputElement>)=>{
    setPrivateKey(e.target.value)
  }
  const onImport = ()=>{
    let secretKey = Uint8Array.from(JSON.parse(privateKey))
    keyPair.current = Keypair.fromSecretKey(secretKey)
    setPublicKey(keyPair.current.publicKey.toString())
  }

  const onBalance = ()=>{
    alert("wait...")
    connection.getBalance((keyPair.current as Keypair).publicKey as PublicKey).then((balance)=>{
      setBalance(balance)
    })
  }

  const onToAmount = (e: React.ChangeEvent<HTMLInputElement>)=>{
    setToAmount(parseFloat(e.target.value) * LAMPORTS_PER_SOL)
  }
  const onAccount = (e: React.ChangeEvent<HTMLInputElement>)=>{
    setAccount(e.target.value)
  }

  const onTransfer = async ()=>{
    alert("transfer " + toAmount + " to " + account)

    const txInstructions = [
      SystemProgram.transfer({
        fromPubkey: (keyPair.current as Keypair).publicKey as PublicKey,
        toPubkey: new PublicKey(account),
        lamports: toAmount as number,
      }),
    ];

    let latestBlockhash = await connection.getLatestBlockhash("finalized")

    const messageV0 = new TransactionMessage({
      payerKey: keyPair.current?.publicKey as PublicKey,
      instructions: txInstructions,
      recentBlockhash: latestBlockhash.blockhash
    }).compileToV0Message();

    const trx = new VersionedTransaction(messageV0)
    trx.sign([keyPair.current!]);
    return await connection.sendTransaction(trx);

  }



  return (
    <div className="App">
      <h1>Web3JS Demo</h1>

      <hr/>
      <br/>
      <label>Enter Private Key:</label>
      <input onChange={ onPrivateKey }></input>
      <button onClick={ onImport }>Import Private Key</button>
      <br/>
      <label>Public Key:</label>
      <label    >{ publicKey }</label>
      <hr/>
      <br/>
      <button onClick={ onBalance }>Query Balance</button>
      <br/>
      <label>Balance:</label>
      <label    >{ balance }</label>
      <hr/>
      <br/>
      <label>Transfer</label>
      <input onChange={ onToAmount }></input>(sol) to 
      <input onChange={ onAccount }></input>(account)
      <button onClick={ onTransfer }>Transfer</button>
      <br/>

    </div>
  );
}

export default App;
