import { Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js"
import { AccountLayout, createMint, getAccount, getMint, getOrCreateAssociatedTokenAccount, mintTo, TOKEN_PROGRAM_ID, transfer } from "@solana/spl-token"
import { useRef, useState } from "react"

export default function App() {
  const whiteList = ["2P1ZZz3bzYi5yYKDtroCRmPcQoqZRJpBGvhhoeT9Ud15", "HAHGzHj6yo6muNRpNaZvBzbhckjsGCFiBhxFbmCTwazq"]
  const connection = new Connection("http://192.168.78.129:8899", "confirmed")
  const [privateKey, setPrivateKey] = useState<string>()
  const [publicKey, setPublicKey] = useState<PublicKey>()
  const keypair = useRef<Keypair>()
  const [balance, setBalance] = useState<number>()
  const [token, setToken] = useState<string>()
  const [tokenStr, setTokenStr] = useState<string>()
  const [viewPublicKey, setViewPublicKey] = useState<string>()
  const [toPublicKey, setToPublicKey] = useState<string>()
  const [amount, setAmount] = useState<number>()
  const [mintPublicKey, setMintPublicKey] = useState<string>()

  const onViewPublicKey = (e: React.ChangeEvent<HTMLInputElement>)=>{
    setViewPublicKey(e.target.value)
  }
  const onToPublicKey = (e: React.ChangeEvent<HTMLInputElement>)=>{
    setToPublicKey(e.target.value)
  }
  const onAmount = (e: React.ChangeEvent<HTMLInputElement>)=>{
    setAmount(parseInt(e.target.value) * LAMPORTS_PER_SOL)
  }

  const onPrivateKey = (e: React.ChangeEvent<HTMLInputElement>)=>{
    setPrivateKey(e.target.value)
  }

  const onConnect = ()=>{
    keypair.current = Keypair.fromSecretKey(Uint8Array.from(JSON.parse(privateKey!)))
    setPublicKey(keypair.current.publicKey)
  }

  const onBalance = ()=>{
    if (connection === undefined) {
      alert("connect fail")
    } else {
      alert("connect success")
    }
    connection.getBalance(publicKey!).then((balance)=>{
      setBalance(balance)
    })
  }

  const onCreateToken = async ()=>{
    const mint = await createMint(
      connection,
      keypair.current!,
      keypair.current!.publicKey!,
      keypair.current!.publicKey!,
      9
    )
    
    const t = mint.toBase58()
    setToken(t)
    console.log("created token: ", t);
    setMintPublicKey(t)

    let mintInfo = await getMint(connection, mint)
    console.log("now, this token's init supply is: ", mintInfo.supply);
    
    const tokenAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      keypair.current!,
      mint,
      keypair.current!.publicKey
    )
    console.log("a new token account: ", tokenAccount.address.toBase58());
    
    let tokenAccountInfo = await getAccount(connection, tokenAccount.address)
    console.log("this token account amount: ", tokenAccountInfo.amount);

    await mintTo(
      connection,
      keypair.current!,
      mint,
      tokenAccount.address,
      keypair.current!,
      100000000000
    )
    mintInfo = await getMint(connection, mint)
    console.log("now, this token's supply is:", mintInfo.supply);
    
    tokenAccountInfo = await getAccount(connection, tokenAccount.address)
    console.log("now this token account amount: ", tokenAccountInfo.amount);
  }



  const onViewTokens = async ()=>{
    let publicKey: string
    if (viewPublicKey === undefined) {
      publicKey = keypair.current?.publicKey!.toString()!
      console.log("view publickey: ", publicKey);
    } else {
      publicKey = new PublicKey(viewPublicKey).toString()
      console.log("view publickey: ", publicKey);
    }

    const tokenAccounts = await connection.getTokenAccountsByOwner(
      new PublicKey(publicKey!),
      { programId: TOKEN_PROGRAM_ID }
    )
    let str = ""
    tokenAccounts.value.forEach(tokenAccount=>{
      const accountData = AccountLayout.decode(tokenAccount.account.data)
      str += `${new PublicKey(accountData.mint)} : ${accountData.amount}\n`
    })
    setTokenStr(str)
    console.log("token list: ", str)
  }

  const onTransfer = async ()=>{
    console.log("transfer "+ amount + " token(s) to " + toPublicKey);
    const fromATA = await getOrCreateAssociatedTokenAccount(
      connection,
      keypair.current!,
      new PublicKey(mintPublicKey!),
      keypair.current?.publicKey!
    )
    const toAta = await getOrCreateAssociatedTokenAccount(
      connection,
      keypair.current!,
      new PublicKey(mintPublicKey!),
      new PublicKey(toPublicKey!)
    )
    const signature = await transfer(
      connection,
      keypair.current!,
      fromATA.address,
      toAta.address,
      keypair.current?.publicKey!,
      amount!
    )
    console.log("transfer signature: ", signature);
    
  }


  return <div>
    <h1>SPL token Transfer Demo</h1>
    <label>input private key:</label>
    <input onChange={ onPrivateKey }/>
    <button onClick={ onConnect }>connect</button>
    <br/>
    <br/>
    
    <label>public key: {publicKey?.toString()}</label>
    <br/>
    <br/>
    <button onClick={ onBalance }>query balance</button>
    <br/>
    <br/>
    <label>balance: {balance}</label>
    <br/>
    <br/>
    
    <button onClick={ onCreateToken }>create token</button>
    <label>created token is: {token}</label>
    <br/>
    <br/>

    <label><strong>AirDrop WhiteList:</strong></label>
    <br/>
    <label>{whiteList.join("\n")}</label>
    <br/>
    <br/>

    Transfer 
    <input onChange={ onAmount }></input>(amount)
    To 
    <input onChange={ onToPublicKey }></input>(to PublicKey)
    <button onClick={ onTransfer } >Transfer</button>
    <br/>
    <br/>

    <input onChange={ onViewPublicKey }></input>
    <button onClick={ onViewTokens }>view token balance:</button>
    <br/>
    <label>{ tokenStr }</label>
    <br/>
    <br/>

  </div>
}