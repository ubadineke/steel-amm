use litesvm::LiteSVM;
use solana_keypair::Keypair;
use solana_message::Message;
use solana_pubkey::{pubkey, Pubkey};
use solana_signer::Signer;
use solana_system_interface::instruction::transfer;
use solana_transaction::Transaction;
use steel_amm_api::prelude::*;

#[test]
fn litesvm_test() {
    let from_keypair = Keypair::new();
    let from = from_keypair.pubkey();
    let to = Pubkey::new_unique();

    let program_id = pubkey!("z7msBPQHDJjTvdQRoEcKyENgXDhSRYeHieN1ZMTqo35");
    let program_bytes = include_bytes!("../../target/deploy/steel_amm_program.so");

    let mut svm = LiteSVM::new();
    svm.airdrop(&from, 10_000).unwrap();
    svm.add_program( program_id, program_bytes );

    let instruction = transfer(&from, &to, 64);

    let tx = Transaction::new(
        &[&from_keypair],
        Message::new(&[instruction], Some(&from)),
        svm.latest_blockhash(),
    );
    let _tx_res = svm.send_transaction(tx).unwrap();

    let from_account = svm.get_account(&from);
    let to_account = svm.get_account(&to);
    assert_eq!(from_account.unwrap().lamports, 4936);
    assert_eq!(to_account.unwrap().lamports, 64)
    
}