/* Offsets */
const OFFSET_8: usize = 8;
const OFFSET_16: usize = 16;
const OFFSET_24: usize = 24;


const BLOCK_SIZE: usize = 64;
const COUNTER_SIZE: usize = 16;
const KEY_SIZE: usize = 32;
const CONSTANT_SIZE: usize = 16;
const SIGMA: [u8; CONSTANT_SIZE] = [b'e', b'x', b'p', b'a', b'n', b'd', b' ', b'3', b'2', b'-', b'b', b'y', b't', b'e', b' ', b'k'];
const ROUNDS: usize = 20;

pub fn xor_key_stream(mut output: &mut [u8], mut input: &[u8], key: &[u8; KEY_SIZE]) {
    let mut block = [0u8; BLOCK_SIZE];
    let mut counter = [0u8; COUNTER_SIZE];

    while input.len() >= BLOCK_SIZE {
        core(&mut block, &mut counter, &key);
        for (i, x) in block.iter().enumerate() {
            output[i] = input[i] ^ x;
        }

        let mut u = 1u32;
        for i in 8..16 {
            u += counter[i] as u32;
            counter[i] = u as u8;
            u >>= 8;
        }
        input = &input[64..];
        output = &mut output[64..];
    }

    if input.len() > 0 {
        core(&mut block, &mut counter, &key);
        for (i, v) in input.iter().enumerate() {
            output[i] = v ^ block[i];
        }
    }
}

fn core(output: &mut [u8; BLOCK_SIZE], input: &[u8; COUNTER_SIZE], k: &[u8; KEY_SIZE]) {
    let mut j: [u32; 16] = [0; 16];
    j[0] = convert_u8_array_to_u32(&SIGMA, 0);
    j[1] = convert_u8_array_to_u32(k, 0);
    j[2] = convert_u8_array_to_u32(k, 4);
    j[3] = convert_u8_array_to_u32(k, 8);
    j[4] = convert_u8_array_to_u32(k, 12);
    j[5] = convert_u8_array_to_u32(&SIGMA, 4);
    j[6] = convert_u8_array_to_u32(input, 0);
    j[7] = convert_u8_array_to_u32(input, 4);
    j[8] = convert_u8_array_to_u32(input, 8);
    j[9] = convert_u8_array_to_u32(input, 12);
    j[10] = convert_u8_array_to_u32(&SIGMA ,8);
    j[11] = convert_u8_array_to_u32(k, 16);
    j[12] = convert_u8_array_to_u32(k, 20);
    j[13] = convert_u8_array_to_u32(k, 24);
    j[14] = convert_u8_array_to_u32(k, 28);
    j[15] = convert_u8_array_to_u32(&SIGMA ,12);

    let mut x = j;
    let mut i = 0;

    while i < ROUNDS {
        let mut u: u32 = x[0].wrapping_add(x[12]);
        x[4] ^= u << 7 | u >> (32 - 7);
        u = x[4].wrapping_add(x[0]);
        x[8] ^= u << 9 | u >> (32 - 9);
        u = x[8].wrapping_add(x[4]);
        x[12] ^= u << 13 | u >> (32 - 13);
        u = x[12].wrapping_add(x[8]);
        x[0] ^= u << 18 | u >> (32 - 18);

        u = x[5].wrapping_add(x[1]);
        x[9] ^= u << 7 | u >> (32 - 7);
        u = x[9].wrapping_add(x[5]);
        x[13] ^= u << 9 | u >> (32 - 9);
        u = x[13].wrapping_add(x[9]);
        x[1] ^= u << 13 | u >> (32 - 13);
        u = x[1].wrapping_add(x[13]);
        x[5] ^= u << 18 | u >> (32 - 18);

        u = x[10].wrapping_add(x[6]);
        x[14] ^= u << 7 | u >> (32 - 7);
        u = x[14].wrapping_add(x[10]);
        x[2] ^= u << 9 | u >> (32 - 9);
        u = x[2].wrapping_add(x[14]);
        x[6] ^= u << 13 | u >> (32 - 13);
        u = x[6].wrapping_add(x[2]);
        x[10] ^= u << 18 | u >> (32 - 18);

        u = x[15].wrapping_add(x[11]);
        x[3] ^= u << 7 | u >> (32 - 7);
        u = x[3].wrapping_add(x[15]);
        x[7] ^= u << 9 | u >> (32 - 9);
        u = x[7].wrapping_add(x[3]);
        x[11] ^= u << 13 | u >> (32 - 13);
        u = x[11].wrapping_add(x[7]);
        x[15] ^= u << 18 | u >> (32 - 18);

        u = x[0].wrapping_add(x[3]);
        x[1] ^= u << 7 | u >> (32 - 7);
        u = x[1].wrapping_add(x[0]);
        x[2] ^= u << 9 | u >> (32 - 9);
        u = x[2].wrapping_add(x[1]);
        x[3] ^= u << 13 | u >> (32 - 13);
        u = x[3].wrapping_add(x[2]);
        x[0] ^= u << 18 | u >> (32 - 18);

        u = x[5].wrapping_add(x[4]);
        x[6] ^= u << 7 | u >> (32 - 7);
        u = x[6].wrapping_add(x[5]);
        x[7] ^= u << 9 | u >> (32 - 9);
        u = x[7].wrapping_add(x[6]);
        x[4] ^= u << 13 | u >> (32 - 13);
        u = x[4].wrapping_add(x[7]);
        x[5] ^= u << 18 | u >> (32 - 18);

        u = x[10].wrapping_add(x[9]);
        x[11] ^= u << 7 | u >> (32 - 7);
        u = x[11].wrapping_add(x[10]);
        x[8] ^= u << 9 | u >> (32 - 9);
        u = x[8].wrapping_add(x[11]);
        x[9] ^= u << 13 | u >> (32 - 13);
        u = x[9].wrapping_add(x[8]);
        x[10] ^= u << 18 | u >> (32 - 18);

        u = x[15].wrapping_add(x[14]);
        x[12] ^= u << 7 | u >> (32 - 7);
        u = x[12].wrapping_add(x[15]);
        x[13] ^= u << 9 | u >> (32 - 9);
        u = x[13].wrapping_add(x[12]);
        x[14] ^= u << 13 | u >> (32 - 13);
        u = x[14].wrapping_add(x[13]);
        x[15] ^= u << 18 | u >> (32 - 18);

        i += 2;
    }

    x[0] = x[0].wrapping_add(j[0]);
    x[1] = x[1].wrapping_add(j[1]);
    x[2] = x[2].wrapping_add(j[2]);
    x[3] = x[3].wrapping_add(j[3]);
    x[4] = x[4].wrapping_add(j[4]);
    x[5] = x[5].wrapping_add(j[5]);
    x[6] = x[6].wrapping_add(j[6]);
    x[7] = x[7].wrapping_add(j[7]);
    x[8] = x[8].wrapping_add(j[8]);
    x[9] = x[9].wrapping_add(j[9]);
    x[10] = x[10].wrapping_add(j[10]);
    x[11] = x[11].wrapping_add(j[11]);
    x[12] = x[12].wrapping_add(j[12]);
    x[13] = x[13].wrapping_add(j[13]);
    x[14] = x[14].wrapping_add(j[14]);
    x[15] = x[15].wrapping_add(j[15]);

    convert_u32_to_u8_array(x[0], output, 0);
    convert_u32_to_u8_array(x[1], output, 4);
    convert_u32_to_u8_array(x[2], output, 8);
    convert_u32_to_u8_array(x[3], output, 12);
    convert_u32_to_u8_array(x[4], output, 16);
    convert_u32_to_u8_array(x[5], output, 20);
    convert_u32_to_u8_array(x[6], output, 24);
    convert_u32_to_u8_array(x[7], output, 28);
    convert_u32_to_u8_array(x[8], output, 32);
    convert_u32_to_u8_array(x[9], output, 36);
    convert_u32_to_u8_array(x[10], output, 40);
    convert_u32_to_u8_array(x[11], output, 44);
    convert_u32_to_u8_array(x[12], output, 48);
    convert_u32_to_u8_array(x[13], output, 52);
    convert_u32_to_u8_array(x[14], output, 56);
    convert_u32_to_u8_array(x[15], output, 60);
}

/* Used In Place Of The Manual Moves, Looks To Be Close To Identical In The Tranmission */
fn convert_u32_to_u8_array(src: u32, dst: &mut [u8], offset: usize) {
    dst[offset] = src as u8;
    dst[offset + 1] = (src >> OFFSET_8) as u8;
    dst[offset + 2] = (src >> OFFSET_16) as u8;
    dst[offset + 3] = (src >> OFFSET_24) as u8;
}

/* Used In Place Of The Manual Moves, Looks To Be Close To Identical In The Tranmission */
fn convert_u8_array_to_u32(src: &[u8], offset: usize) -> u32 {
    return src[offset] as u32 | (src[offset + 1] as u32) << OFFSET_8 | (src[offset + 2] as u32) << OFFSET_16 | (src[offset + 3] as u32) << OFFSET_24;
}