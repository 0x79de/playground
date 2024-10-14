use aes::Aes128;
use block_modes::block_padding::NoPadding;
use block_modes::{BlockMode, Ecb};

type Aes128Ecb = Ecb<Aes128, NoPadding>;

fn davies_meyer(message: &[u8], initial_value: &[u8; 16]) -> [u8; 16] {
    let mut state = *initial_value;

    for chunk in message.chunks(16) {
        let mut padded_chunk = [0u8; 16];
        padded_chunk[..chunk.len()].copy_from_slice(chunk);

        let cipher = Aes128Ecb::new_from_slices(&padded_chunk, &[0u8; 16]).unwrap();
        let mut block = state;
        cipher.encrypt_block(&mut block);

        for i in 0..16 {
            state[i] ^= block[i];
        }
    }

    state
}

fn main() {
    let message = b"Hello, world!";
    let initial_value = [0u8; 16];

    let result = davies_meyer(message, &initial_value);

    println!("Input: {:?}", message);
    println!("Output: {:?}", result);
}
