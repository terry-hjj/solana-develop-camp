
import { WalletAdapterNetwork } from "@solana/wallet-adapter-base"
import { SolongWalletAdapter } from "@solana/wallet-adapter-wallets"
import { PhantomWalletAdapter } from "@solana/wallet-adapter-phantom"
import { WalletDisconnectButton, WalletModalProvider, WalletMultiButton } from "@solana/wallet-adapter-react-ui"
import { 
  SystemProgram,
  clusterApiUrl, 
  PublicKey,
  LAMPORTS_PER_SOL,
  TransactionMessage,
  VersionedTransaction
} from "@solana/web3.js"
import { 
  ConnectionProvider,   
  WalletProvider, 
  useConnection, 
  useWallet
} from "@solana/wallet-adapter-react"
import "@solana/wallet-adapter-react-ui/styles.css";
import { useState } from "react"


export default function App() {
  const endpoint = clusterApiUrl(WalletAdapterNetwork.Devnet)
  const wallets = [
    new SolongWalletAdapter(),
    new PhantomWalletAdapter()
  ]

  return <div>
    <h1>Wallet Demo</h1>
    <ConnectionProvider endpoint={ endpoint }>
      <WalletProvider wallets={ wallets } autoConnect>
        <WalletModalProvider>
          <WalletMultiButton />
          <br/>
          <br/>
          Click here to disconnect:
          <WalletDisconnectButton />
          <br/>
          <br/>
          <Page/>
        </WalletModalProvider>
      </WalletProvider>
    </ConnectionProvider>
  </div>
}

// <WalletMultiButton/> 需要先加载, 所以其它内容单独放在一个组件里,在前者加载完毕后再加载
function Page() {
  const [ balance, setBalance ] = useState(0)
  const [ amount, setAmount ] = useState<number>()
  const [ toPublicKey, setToPublicKey ] = useState("")
  
  const { publicKey, sendTransaction } = useWallet()
  const { connection } = useConnection()
  
  const onBalance = ()=>{
    connection.getBalance(publicKey!).then((balance)=>{
      setBalance(balance)
    })
  }
  const onAmount = (e: React.ChangeEvent<HTMLInputElement>)=>{
    setAmount(parseFloat(e.target.value) * LAMPORTS_PER_SOL)
  }
  const onToPublicKey = (e: React.ChangeEvent<HTMLInputElement>)=>{
    setToPublicKey(e.target.value)
  }

  const onTransfer = async ()=>{
    const txInstructions = [
      SystemProgram.transfer({
        fromPubkey: publicKey!,
        toPubkey: new PublicKey(toPublicKey),
        lamports: amount!
      })
    ];
  
    const {
      context: { slot: minContextSlot },
      value: { blockhash },
    } = await connection.getLatestBlockhashAndContext();

    const messageV0 = new TransactionMessage({
      payerKey: publicKey!,
      instructions: txInstructions,
      recentBlockhash: blockhash,
    }).compileToV0Message();

    const trx = new VersionedTransaction(messageV0)
    const signature = await sendTransaction(trx, connection, { minContextSlot })
    console.log("transfer tx signature: ", signature);
  }
  return <div>
    <br/>
    <button onClick={ onBalance }> Query Balance </button>
    <br/>
    <span>Balance: { balance }</span>
    <br/>
    <br/>
    Transfer 
    <input onChange={ onAmount } /> (sol)
    To
    <input onChange={ onToPublicKey } /> (account)
    <br/>
    <button onClick={ onTransfer }> Transfer </button>
    <br/>
  </div>
}