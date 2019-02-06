// Huffman Encoder/Decoder to deal with compressed D2GS packages


const INDEX_TABLE: [u16; 256] = [
        0x0247, 0x0236, 0x0225, 0x0214, 0x0203, 0x01F2, 0x01E1, 0x01D0,
        0x01BF, 0x01AE, 0x019D, 0x018C, 0x017B, 0x016A, 0x0161, 0x0158,
        0x014F, 0x0146, 0x013D, 0x0134, 0x012B, 0x0122, 0x0119, 0x0110,
        0x0107, 0x00FE, 0x00F5, 0x00EC, 0x00E3, 0x00DA, 0x00D1, 0x00C8,
        0x00BF, 0x00B6, 0x00AD, 0x00A8, 0x00A3, 0x009E, 0x0099, 0x0094,
        0x008F, 0x008A, 0x0085, 0x0080, 0x007B, 0x0076, 0x0071, 0x006C,
        0x0069, 0x0066, 0x0063, 0x0060, 0x005D, 0x005A, 0x0057, 0x0054,
        0x0051, 0x004E, 0x004B, 0x0048, 0x0045, 0x0042, 0x003F, 0x003F,
        0x003C, 0x003C, 0x0039, 0x0039, 0x0036, 0x0036, 0x0033, 0x0033,
        0x0030, 0x0030, 0x002D, 0x002D, 0x002A, 0x002A, 0x0027, 0x0027,
        0x0024, 0x0024, 0x0021, 0x0021, 0x001E, 0x001E, 0x001B, 0x001B,
        0x0018, 0x0018, 0x0015, 0x0015, 0x0012, 0x0012, 0x0012, 0x0012,
        0x000F, 0x000F, 0x000F, 0x000F, 0x000C, 0x000C, 0x000C, 0x000C,
        0x0009, 0x0009, 0x0009, 0x0009, 0x0006, 0x0006, 0x0006, 0x0006,
        0x0003, 0x0003, 0x0003, 0x0003, 0x0003, 0x0003, 0x0003, 0x0003,
        0x0003, 0x0003, 0x0003, 0x0003, 0x0003, 0x0003, 0x0003, 0x0003,
        0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
        0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
        0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
        0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
        0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
        0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
        0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
        0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
        0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
        0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
        0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
        0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
        0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
        0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
        0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
        0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000];

const CHARACTER_TABLE: [u8; 600] = [
        0x00, 0x00, 0x01, 0x00, 0x01, 0x04, 0x00, 0xFF, 0x06, 0x00, 0x14, 0x06,
        0x00, 0x13, 0x06, 0x00, 0x05, 0x06, 0x00, 0x02, 0x06, 0x00, 0x80, 0x07,
        0x00, 0x6D, 0x07, 0x00, 0x69, 0x07, 0x00, 0x68, 0x07, 0x00, 0x67, 0x07,
        0x00, 0x1E, 0x07, 0x00, 0x15, 0x07, 0x00, 0x12, 0x07, 0x00, 0x0D, 0x07,
        0x00, 0x0A, 0x07, 0x00, 0x08, 0x07, 0x00, 0x07, 0x07, 0x00, 0x06, 0x07,
        0x00, 0x04, 0x07, 0x00, 0x03, 0x07, 0x00, 0x6C, 0x08, 0x00, 0x51, 0x08,
        0x00, 0x20, 0x08, 0x00, 0x1F, 0x08, 0x00, 0x1D, 0x08, 0x00, 0x18, 0x08,
        0x00, 0x17, 0x08, 0x00, 0x16, 0x08, 0x00, 0x11, 0x08, 0x00, 0x10, 0x08,
        0x00, 0x0F, 0x08, 0x00, 0x0C, 0x08, 0x00, 0x0B, 0x08, 0x00, 0x09, 0x08,
        0x01, 0x96, 0x09, 0x97, 0x09, 0x01, 0x90, 0x09, 0x95, 0x09, 0x01, 0x64,
        0x09, 0x6B, 0x09, 0x01, 0x62, 0x09, 0x63, 0x09, 0x01, 0x56, 0x09, 0x58,
        0x09, 0x01, 0x52, 0x09, 0x55, 0x09, 0x01, 0x4D, 0x09, 0x50, 0x09, 0x01,
        0x45, 0x09, 0x4C, 0x09, 0x01, 0x40, 0x09, 0x43, 0x09, 0x01, 0x31, 0x09,
        0x3B, 0x09, 0x01, 0x28, 0x09, 0x30, 0x09, 0x01, 0x1A, 0x09, 0x25, 0x09,
        0x01, 0x0E, 0x09, 0x19, 0x09, 0x02, 0xE2, 0x0A, 0xE8, 0x0A, 0xF0, 0x0A,
        0xF8, 0x0A, 0x02, 0xC0, 0x0A, 0xC2, 0x0A, 0xCE, 0x0A, 0xE0, 0x0A, 0x02,
        0xA0, 0x0A, 0xA2, 0x0A, 0xB0, 0x0A, 0xB8, 0x0A, 0x02, 0x8A, 0x0A, 0x8F,
        0x0A, 0x93, 0x0A, 0x98, 0x0A, 0x02, 0x81, 0x0A, 0x82, 0x0A, 0x83, 0x0A,
        0x89, 0x0A, 0x02, 0x7C, 0x0A, 0x7D, 0x0A, 0x7E, 0x0A, 0x7F, 0x0A, 0x02,
        0x77, 0x0A, 0x78, 0x0A, 0x79, 0x0A, 0x7A, 0x0A, 0x02, 0x73, 0x0A, 0x74,
        0x0A, 0x75, 0x0A, 0x76, 0x0A, 0x02, 0x6E, 0x0A, 0x6F, 0x0A, 0x70, 0x0A,
        0x72, 0x0A, 0x02, 0x61, 0x0A, 0x65, 0x0A, 0x66, 0x0A, 0x6A, 0x0A, 0x02,
        0x5D, 0x0A, 0x5E, 0x0A, 0x5F, 0x0A, 0x60, 0x0A, 0x02, 0x57, 0x0A, 0x59,
        0x0A, 0x5A, 0x0A, 0x5B, 0x0A, 0x02, 0x4A, 0x0A, 0x4B, 0x0A, 0x4E, 0x0A,
        0x53, 0x0A, 0x02, 0x46, 0x0A, 0x47, 0x0A, 0x48, 0x0A, 0x49, 0x0A, 0x02,
        0x3F, 0x0A, 0x41, 0x0A, 0x42, 0x0A, 0x44, 0x0A, 0x02, 0x3A, 0x0A, 0x3C,
        0x0A, 0x3D, 0x0A, 0x3E, 0x0A, 0x02, 0x36, 0x0A, 0x37, 0x0A, 0x38, 0x0A,
        0x39, 0x0A, 0x02, 0x32, 0x0A, 0x33, 0x0A, 0x34, 0x0A, 0x35, 0x0A, 0x02,
        0x2B, 0x0A, 0x2C, 0x0A, 0x2D, 0x0A, 0x2E, 0x0A, 0x02, 0x26, 0x0A, 0x27,
        0x0A, 0x29, 0x0A, 0x2A, 0x0A, 0x02, 0x21, 0x0A, 0x22, 0x0A, 0x23, 0x0A,
        0x24, 0x0A, 0x03, 0xFB, 0x0B, 0xFC, 0x0B, 0xFD, 0x0B, 0xFE, 0x0B, 0x1B,
        0x0A, 0x1B, 0x0A, 0x1C, 0x0A, 0x1C, 0x0A, 0x03, 0xF2, 0x0B, 0xF3, 0x0B,
        0xF4, 0x0B, 0xF5, 0x0B, 0xF6, 0x0B, 0xF7, 0x0B, 0xF9, 0x0B, 0xFA, 0x0B,
        0x03, 0xE9, 0x0B, 0xEA, 0x0B, 0xEB, 0x0B, 0xEC, 0x0B, 0xED, 0x0B, 0xEE,
        0x0B, 0xEF, 0x0B, 0xF1, 0x0B, 0x03, 0xDE, 0x0B, 0xDF, 0x0B, 0xE1, 0x0B,
        0xE3, 0x0B, 0xE4, 0x0B, 0xE5, 0x0B, 0xE6, 0x0B, 0xE7, 0x0B, 0x03, 0xD6,
        0x0B, 0xD7, 0x0B, 0xD8, 0x0B, 0xD9, 0x0B, 0xDA, 0x0B, 0xDB, 0x0B, 0xDC,
        0x0B, 0xDD, 0x0B, 0x03, 0xCD, 0x0B, 0xCF, 0x0B, 0xD0, 0x0B, 0xD1, 0x0B,
        0xD2, 0x0B, 0xD3, 0x0B, 0xD4, 0x0B, 0xD5, 0x0B, 0x03, 0xC5, 0x0B, 0xC6,
        0x0B, 0xC7, 0x0B, 0xC8, 0x0B, 0xC9, 0x0B, 0xCA, 0x0B, 0xCB, 0x0B, 0xCC,
        0x0B, 0x03, 0xBB, 0x0B, 0xBC, 0x0B, 0xBD, 0x0B, 0xBE, 0x0B, 0xBF, 0x0B,
        0xC1, 0x0B, 0xC3, 0x0B, 0xC4, 0x0B, 0x03, 0xB2, 0x0B, 0xB3, 0x0B, 0xB4,
        0x0B, 0xB5, 0x0B, 0xB6, 0x0B, 0xB7, 0x0B, 0xB9, 0x0B, 0xBA, 0x0B, 0x03,
        0xA9, 0x0B, 0xAA, 0x0B, 0xAB, 0x0B, 0xAC, 0x0B, 0xAD, 0x0B, 0xAE, 0x0B,
        0xAF, 0x0B, 0xB1, 0x0B, 0x03, 0x9F, 0x0B, 0xA1, 0x0B, 0xA3, 0x0B, 0xA4,
        0x0B, 0xA5, 0x0B, 0xA6, 0x0B, 0xA7, 0x0B, 0xA8, 0x0B, 0x03, 0x92, 0x0B,
        0x94, 0x0B, 0x99, 0x0B, 0x9A, 0x0B, 0x9B, 0x0B, 0x9C, 0x0B, 0x9D, 0x0B,
        0x9E, 0x0B, 0x03, 0x86, 0x0B, 0x87, 0x0B, 0x88, 0x0B, 0x8B, 0x0B, 0x8C,
        0x0B, 0x8D, 0x0B, 0x8E, 0x0B, 0x91, 0x0B, 0x03, 0x2F, 0x0B, 0x4F, 0x0B,
        0x54, 0x0B, 0x5C, 0x0B, 0x71, 0x0B, 0x7B, 0x0B, 0x84, 0x0B, 0x85, 0x0B];


const BIT_MASKS: [u16; 16] = [
        0x0000, 0x0001, 0x0003, 0x0007, 0x000F, 0x001F, 0x003F, 0x007F,
        0x00FF, 0x01FF, 0x03FF, 0x07FF, 0x0FFF, 0x1FFF, 0x3FFF, 0x7FFF];


const COMPRESSION_TABLE: [u32; 256] = [
        0x80010000, 0x70040000, 0x5C060000, 0x3E070000, 0x40070000, 0x60060000, 0x42070000, 0x44070000,
        0x46070000, 0x30080000, 0x48070000, 0x31080000, 0x32080000, 0x4A070000, 0x23080100, 0x33080000,
        0x34080000, 0x35080000, 0x4C070000, 0x64060000, 0x68060000, 0x4E070000, 0x36080000, 0x37080000,
        0x38080000, 0x23080101, 0x24080100, 0x0D070306, 0x0D070307, 0x39080000, 0x50070000, 0x3A080000,
        0x3B080000, 0x0E080200, 0x0E080201, 0x0E080202, 0x0E080203, 0x24080101, 0x0F080200, 0x0F080201,
        0x25080100, 0x0F080202, 0x0F080203, 0x10080200, 0x10080201, 0x10080202, 0x10080203, 0x00080300,
        0x25080101, 0x26080100, 0x11080200, 0x11080201, 0x11080202, 0x11080203, 0x12080200, 0x12080201,
        0x12080202, 0x12080203, 0x13080200, 0x26080101, 0x13080201, 0x13080202, 0x13080203, 0x14080200,
        0x27080100, 0x14080201, 0x14080202, 0x27080101, 0x14080203, 0x28080100, 0x15080200, 0x15080201,
        0x15080202, 0x15080203, 0x16080200, 0x16080201, 0x28080101, 0x29080100, 0x16080202, 0x00080301,
        0x29080101, 0x3C080000, 0x2A080100, 0x16080203, 0x00080302, 0x2A080101, 0x2B080100, 0x17080200,
        0x2B080101, 0x17080201, 0x17080202, 0x17080203, 0x00080303, 0x18080200, 0x18080201, 0x18080202,
        0x18080203, 0x19080200, 0x2C080100, 0x2C080101, 0x2D080100, 0x19080201, 0x19080202, 0x52070000,
        0x54070000, 0x56070000, 0x19080203, 0x2D080101, 0x3D080000, 0x58070000, 0x1A080200, 0x1A080201,
        0x1A080202, 0x00080304, 0x1A080203, 0x1B080200, 0x1B080201, 0x1B080202, 0x1B080203, 0x1C080200,
        0x1C080201, 0x1C080202, 0x1C080203, 0x00080305, 0x1D080200, 0x1D080201, 0x1D080202, 0x1D080203,
        0x5A070000, 0x1E080200, 0x1E080201, 0x1E080202, 0x00080306, 0x00080307, 0x01080300, 0x01080301,
        0x01080302, 0x1E080203, 0x1F080200, 0x01080303, 0x01080304, 0x01080305, 0x01080306, 0x1F080201,
        0x2E080100, 0x01080307, 0x02080300, 0x1F080202, 0x02080301, 0x2E080101, 0x2F080100, 0x2F080101,
        0x1F080203, 0x02080302, 0x02080303, 0x02080304, 0x02080305, 0x02080306, 0x02080307, 0x03080300,
        0x20080200, 0x03080301, 0x20080201, 0x03080302, 0x03080303, 0x03080304, 0x03080305, 0x03080306,
        0x03080307, 0x04080300, 0x04080301, 0x04080302, 0x04080303, 0x04080304, 0x04080305, 0x04080306,
        0x20080202, 0x04080307, 0x05080300, 0x05080301, 0x05080302, 0x05080303, 0x05080304, 0x05080305,
        0x20080203, 0x05080306, 0x05080307, 0x06080300, 0x06080301, 0x06080302, 0x06080303, 0x06080304,
        0x21080200, 0x06080305, 0x21080201, 0x06080306, 0x06080307, 0x07080300, 0x07080301, 0x07080302,
        0x07080303, 0x07080304, 0x07080305, 0x07080306, 0x07080307, 0x08080300, 0x21080202, 0x08080301,
        0x08080302, 0x08080303, 0x08080304, 0x08080305, 0x08080306, 0x08080307, 0x09080300, 0x09080301,
        0x09080302, 0x09080303, 0x09080304, 0x09080305, 0x09080306, 0x09080307, 0x0A080300, 0x0A080301,
        0x21080203, 0x0A080302, 0x22080200, 0x0A080303, 0x0A080304, 0x0A080305, 0x0A080306, 0x0A080307,
        0x22080201, 0x0B080300, 0x0B080301, 0x0B080302, 0x0B080303, 0x0B080304, 0x0B080305, 0x0B080306,
        0x22080202, 0x0B080307, 0x0C080300, 0x0C080301, 0x0C080302, 0x0C080303, 0x0C080304, 0x0C080305,
        0x22080203, 0x0C080306, 0x0C080307, 0x0D080300, 0x0D080301, 0x0D080302, 0x0D080303, 0x6C060000];


pub fn decompress(input: &[u8], output: &mut [u8]) {
    if Some(input) == None {
        panic!("decode(): Invalid input reference");
    }
    // result needs to be a res
    //let mut result: [u8; 1024] = [0; 1024];
    let mut result: Vec<u8> = Vec::with_capacity(1024);
    let mut size        = input.len();
    let mut b: usize    = 0;
    let mut i: usize    = 0;
    let mut max: usize  = result.capacity(); // available storage indicator
    let mut count: i32  = 0x20;

    loop {
        let mut a: usize;
        if count >= 8 {
            while size > 0 && count >= 8 {
                count -= 8;
                size -= 1;
                a = (input[i] as usize)  << count as i32;
                b |= a;
                i+=1;
            }
        }
        dbg!(b); dbg!(b >> 0x18);
        let index = INDEX_TABLE[b >> 0x18] as usize;
        let mut a = CHARACTER_TABLE[index] as usize;
        let     d = (b >> (0x18 - a)) & BIT_MASKS[a] as usize;
        let     c = CHARACTER_TABLE[index + 2 * d + 2] as u32;

        count += c as i32;
        if count > 0x20 {
            output.copy_from_slice(result.as_slice());
            return;
        }

        max -= 1;
        if max == 0 {
            // double available capacity
            let len = result.len()*2;
            result.resize(len, 0);
        }
        a = CHARACTER_TABLE[index + 2 * d + 1] as usize;

        result.push(a as u8);

        b <<= c & 0xFF;
    }
}

pub fn get_chunk_params(raw: &[u8], header_size: &mut usize) -> usize {
    if raw[0] < 0xF0 {
        *header_size = 1 as usize;
        return raw[0] as usize - 1;
    }
    *header_size = 2;
    // only bottom 3 nibbles due to header offset (-2)?
    return ( (( (raw[0] as u16) << 8) | raw[1] as u16) & 0xFFF ) as usize
}
