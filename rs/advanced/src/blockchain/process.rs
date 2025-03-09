use crate::{BlockRef, TransactionRef};

#[allow(dead_code)]
pub fn process<'a>(blocks: impl Iterator<Item = BlockRef<'a>>, mut f: impl FnMut(&TransactionRef)) {
    blocks.for_each(|block| block.transactions.iter().for_each(&mut f))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Block, Transaction};

    fn make_block() -> Block {
        Block {
            id: 10,
            timestamp: 987654321,
            transactions: vec![
                Transaction {
                    data: vec![5, 6, 7],
                },
                Transaction { data: vec![8, 9] },
            ],
            prev_hash: [1u8; 32],
        }
    }

    #[test]
    fn test() {
        let blocks = (0..20).map(|_| make_block()).collect::<Vec<_>>();

        let mut count = 0;
        process(blocks.iter().map(|b| b.into()), |t| {
            count += 1;
            println!("{t:?}")
        });
        println!("{count} transactions found");
    }
}
