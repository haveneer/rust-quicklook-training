use crate::{BlockRef, ParseError, TransactionRef};

unsafe fn read_slice<const N: usize>(input: &[u8], offset: usize) -> &[u8; N] {
    let slice = &input[offset..offset + N];
    &*(slice.as_ptr().cast::<[u8; N]>())
}

impl<'a> BlockRef<'a> {
    pub fn from_bytes_unsafe(input: &'a [u8]) -> Result<(Self, usize), ParseError> {
        // Check that the fixed header (id, timestamp, transaction count) is present.
        let total_len = input.len();
        if total_len < 48 {
            return Err(ParseError::UnexpectedEOF);
        }

        // Read block id (4 bytes, little-endian)
        let id = unsafe { read_slice::<4>(input, 0) };
        let id = u32::from_le_bytes(*id);

        // Read timestamp (8 bytes, little-endian)
        let timestamp = unsafe { read_slice::<8>(input, 4) };
        let timestamp = u64::from_le_bytes(*timestamp);

        // Read previous block hash (32 bytes)
        let prev_hash = unsafe { read_slice::<32>(input, 12) };

        // Read the number of transactions (4 bytes, little-endian)
        let num_tx = unsafe { read_slice::<4>(input, 44) };
        let num_tx = u32::from_le_bytes(*num_tx);

        // Read each transaction
        let mut transactions = Vec::with_capacity(num_tx as usize);
        let mut offset = 48;
        for _ in 0..num_tx {
            // Read the transaction length (4 bytes)
            if total_len < offset + 4 {
                return Err(ParseError::UnexpectedEOF);
            }
            let tx_len = unsafe { read_slice::<4>(input, offset) };
            let tx_len = u32::from_le_bytes(*tx_len) as usize;
            offset += 4;

            // Verify that the transaction data is available
            if total_len < offset + tx_len {
                return Err(ParseError::UnexpectedEOF);
            }
            let tx_data = &input[offset..offset + tx_len];
            offset += tx_len;

            transactions.push(TransactionRef { data: tx_data });
        }

        Ok((
            BlockRef {
                id,
                timestamp,
                transactions,
                prev_hash,
            },
            offset,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use memmap2::MmapOptions;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_blockref_from_bytes_valid() {
        // Build a test block:
        // id = 1, timestamp = 100
        // prev_hash: 32 bytes of 0
        // number of transactions = 2
        // Transaction 1: length = 3, data = [10, 20, 30]
        // Transaction 2: length = 2, data = [40, 50]
        let mut buf = Vec::new();
        buf.extend_from_slice(&1u32.to_le_bytes());
        buf.extend_from_slice(&100u64.to_le_bytes());
        buf.extend_from_slice(&[0u8; 32]);
        buf.extend_from_slice(&2u32.to_le_bytes());

        buf.extend_from_slice(&3u32.to_le_bytes());
        buf.extend_from_slice(&[10, 20, 30]);

        buf.extend_from_slice(&2u32.to_le_bytes());
        buf.extend_from_slice(&[40, 50]);

        assert_eq!(buf.len(), 61);

        let (block_ref, consumed) =
            BlockRef::from_bytes_unsafe(&buf).expect("Block should be parsed successfully");
        assert_eq!(block_ref.id, 1);
        assert_eq!(block_ref.timestamp, 100);
        assert_eq!(block_ref.transactions.len(), 2);
        assert_eq!(block_ref.transactions[0].data, &[10, 20, 30]);
        assert_eq!(block_ref.transactions[1].data, &[40, 50]);
        assert_eq!(block_ref.prev_hash, &[0u8; 32]);
        assert_eq!(consumed, 61);
    }

    #[test]
    fn test_blockref_from_bytes_insufficient_data() {
        let buf = vec![0u8; 10];
        let res = BlockRef::from_bytes_unsafe(&buf);
        assert!(res.is_err());
    }

    #[test]
    fn test_parse_block_from_mmap() {
        let mut tmpfile = NamedTempFile::new().expect("Failed to create temporary file");
        let mut buf = Vec::new();
        buf.extend_from_slice(&42u32.to_le_bytes());
        buf.extend_from_slice(&123456789u64.to_le_bytes());
        buf.extend_from_slice(&[255u8; 32]);
        buf.extend_from_slice(&1u32.to_le_bytes());
        buf.extend_from_slice(&4u32.to_le_bytes());
        buf.extend_from_slice(&[1, 2, 3, 4]);

        tmpfile
            .write_all(&buf)
            .expect("Failed to write to temporary file");

        let mmap = unsafe {
            MmapOptions::new()
                .map(tmpfile.as_file())
                .expect("Failed to map the file")
        };

        let (block_ref, consumed) =
            BlockRef::from_bytes_unsafe(&mmap).expect("Block should be parsed from mmap");
        assert_eq!(block_ref.id, 42);
        assert_eq!(block_ref.timestamp, 123456789);
        assert_eq!(block_ref.transactions.len(), 1);
        assert_eq!(block_ref.transactions[0].data, &[1, 2, 3, 4]);
        assert_eq!(block_ref.prev_hash, &[255u8; 32]);
        assert_eq!(consumed, mmap.len());
    }
}
