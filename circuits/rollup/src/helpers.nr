use dep::std;
use dep::std::collections::vec::Vec;

pub fn u8_array_to_u64(arr: [u8; 8]) -> u64 {
    (arr[0] as u64) * 72057594037927936 +  // 0x100000000000000 in decimal
    (arr[1] as u64) * 281474976710656 +    // 0x1000000000000 in decimal
    (arr[2] as u64) * 1099511627776 +      // 0x10000000000 in decimal
    (arr[3] as u64) * 4294967296 +         // 0x100000000 in decimal
    (arr[4] as u64) * 16777216 +           // 0x1000000 in decimal
    (arr[5] as u64) * 65536 +              // 0x10000 in decimal
    (arr[6] as u64) * 256 +                // 0x100 in decimal
    (arr[7] as u64)
}

pub fn combine(left: [u8;32], right: [u8;32]) -> [u8]{
    let mut merged_vec: [u8] = [];
    for byte in left{
        merged_vec = merged_vec.push_back(byte);
    };
    for byte in right{
        merged_vec = merged_vec.push_back(byte);
    };
    merged_vec
}