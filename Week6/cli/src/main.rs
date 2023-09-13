use std::str::FromStr;
use borsh::{BorshSerialize, BorshDeserialize, BorshSchema};
use solana_sdk::signature::Signer;
use solana_rpc_client::rpc_client;
use solana_sdk::signer::keypair;
use solana_sdk::transaction;
use solana_program::instruction;
// use solana_program::pubkey;
use solana_program::pubkey::Pubkey;





const RPC_ADDR: &str = "https://api.devnet.solana.com";

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, PartialEq, BorshSchema)]
pub enum ExtSplInstruction {
    Mint {
        name: String, 
        symbol: String,
        icon: String
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct ExtMint {
    pub mint: Pubkey,
    pub name: String,
    pub symbol: String,
    pub icon: String
}



fn main() {
    let extspl = Pubkey::from_str("2JbZAici8HCWnBsJd39VNJUKSYhMA2JKYodYpA8EjcP9").unwrap();
    let spltoken = Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap();
    let sysproj = Pubkey::from_str("11111111111111111111111111111111").unwrap();
    let mint = Pubkey::from_str("5jXVnYxQ89Yw5xjSxcKykFh2H8T78J7jFao56PhyG6u9").unwrap();
    let me = keypair::Keypair::from_base58_string("HAHGzHj6yo6muNRpNaZvBzbhckjsGCFiBhxFbmCTwazq");
    println!("me is {}", me.pubkey());
    let seeds = [
        &spltoken.to_bytes()[..],
        &mint.to_bytes()[..]
    ];

    let (ext_mint, _seed) = Pubkey::find_program_address(&seeds[..], &extspl);
    let client = rpc_client::RpcClient::new(RPC_ADDR);
    let account_metas = vec![
        instruction::AccountMeta::new(me.pubkey(), true),
        instruction::AccountMeta::new_readonly(spltoken, false),
        instruction::AccountMeta::new_readonly(sysproj, false),
        instruction::AccountMeta::new_readonly(mint, false),
        instruction::AccountMeta::new(ext_mint, false)
    ];
    
    let mint_ix = ExtSplInstruction::Mint{
        name: "test".to_string(),
        symbol: "TEST".to_string(),
        icon: "http://www.baidu.com".to_string()
    };
    
    let instruction = instruction::Instruction::new_with_bytes(
        extspl, 
        &(mint_ix.try_to_vec().unwrap()), 
        account_metas
    );
    
    let ixs = vec![instruction];
    let latest_blockhash = client.get_latest_blockhash().unwrap();
    let sig = client.send_and_confirm_transaction(&transaction::Transaction::new_signed_with_payer(
        &ixs,
        Some(&me.pubkey()),
        &[&me],
        latest_blockhash
    )).unwrap();

    println!("tx:{}", sig);

    let state = client.get_account(&ext_mint).unwrap();
    let extmint_info = ExtMint::try_from_slice(&state.data).unwrap();
    println!("extmint_info: {:#?}", extmint_info);

}
