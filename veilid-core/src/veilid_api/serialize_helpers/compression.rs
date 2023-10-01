use super::*;
use lz4_flex::block;

pub fn compress_prepend_size(input: &[u8]) -> Vec<u8> {
    block::compress_prepend_size(input)
}

pub fn decompress_size_prepended(
    input: &[u8],
    max_size: Option<usize>,
) -> VeilidAPIResult<Vec<u8>> {
    let (uncompressed_size, input) =
        block::uncompressed_size(input).map_err(VeilidAPIError::generic)?;
    if let Some(max_size) = max_size {
        if uncompressed_size > max_size {
            apibail_generic!(format!(
                "decompression exceeded maximum size: {} > {}",
                uncompressed_size, max_size
            ));
        }
    }
    block::decompress(input, uncompressed_size).map_err(VeilidAPIError::generic)
}
