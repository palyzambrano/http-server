use anyhow::{Error, Result};
use compression::prelude::{Action, CompressionError, EncodeExt, GZipEncoder};

pub fn gzip(bytes: &[u8]) -> Result<Vec<u8>> {
    bytes
        .into_iter()
        .cloned()
        .encode(&mut GZipEncoder::new(), Action::Finish)
        .collect::<Result<Vec<u8>, CompressionError>>()
        .map_err(Error::from)
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn compresses_gzip() {
        let raw = b"aabbaabbaabbaabb\n";
        let compressed = gzip(raw).unwrap();
        let expect: [u8; 26] = [
            31, 139, 8, 0, 0, 0, 0, 0, 0, 255, 75, 76, 76, 74, 66, 198, 92, 0, 169, 225, 127, 69,
            17, 0, 0, 0,
        ];

        assert_eq!(compressed, expect);
    }
}
