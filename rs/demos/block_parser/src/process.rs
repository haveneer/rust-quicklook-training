// use crate::{BlockDesc, BlockRef, TransactionRef, TransactionsDesc};

// #[allow(dead_code)]
// pub fn process<'a>(blocks: impl Iterator<Item = BlockRef<'a>>, mut f: impl FnMut(TransactionRef<'_>)) {
//     blocks.for_each(|block| block.transactions().iter().for_each(&mut f))
// }

#[cfg(test)]
mod tests {
    // use crate::block::{Block, Transaction};
    // use super::*;
    //
    // fn make_block() -> Block::<'_> {
    //     Block::<'_> {
    //         id: 10,
    //         timestamp: 987654321,
    //         transactions: vec![
    //             Transaction {
    //                 data: vec![5, 6, 7],
    //             },
    //             Transaction { data: vec![8, 9] },
    //         ],
    //         prev_hash: [1u8; 32],
    //     }
    // }
    //
    // #[test]
    // fn test() {
    //     let blocks = (0..20).map(|_| make_block()).collect::<Vec<_>>();
    //
    //     let mut count = 0;
    //     process(blocks.iter().map(|b| b.into()), |t| {
    //         count += 1;
    //         println!("{t:?}")
    //     });
    //     println!("{count} transactions found");
    // }
}
