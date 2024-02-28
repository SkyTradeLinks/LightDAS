use sqlx::{Pool, Postgres};

use crate::{config::queue::pop_front, rpc::rpc::get_transaction_with_retries};

use super::transaction::process_transaction;

pub async fn process_transactions_queue(database_pool: Pool<Postgres>) {
  loop {
    let transaction_signature = pop_front();

    match transaction_signature {
        Some(txs) =>{
          let transaction =
            get_transaction_with_retries(&txs.transaction_signature)
                .await
                .unwrap();

        process_transaction(transaction, database_pool.clone()).await;
        }
        None => {
            // println!("No transactions in queue");
        }
    }
  }
}
