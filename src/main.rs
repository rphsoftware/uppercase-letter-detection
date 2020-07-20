use std::io;

mod acs_provider;

fn read_bit(byte: u8, index: u8) -> u8 {
    // build mask
    let mask = 0b1 << index;

    return (byte & mask) >> index;
}

fn generate_mask(amount: u32) -> u32 {
    let mut mask : u32 = 0;
    mask = !mask;
    mask = mask << amount;
    mask = !mask;

    return mask
}

fn main() {
    let data: &[u8; 139264];

    unsafe {
        data = acs_provider::get_data();
    }

    // Read user input (evil 0_0)
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)
        .expect("An error occured while reading user input.");

    // Convert user input to unicode vector
    let utf8data = buffer.as_bytes();
    let mut caps_chars = 0;
    let mut total_chars = 0;

    let mut complex_bytes_left = 0;
    let mut complex_value_so_far = 0;

    for byte in utf8data {
        let first_bit = read_bit(*byte, 7);
        if first_bit == 0 && complex_bytes_left == 0 { // We are in a single byte character with previous sequence closed cleanly.
            let index = byte >> 3;
            let offset = byte & 0b00000111;

            caps_chars += (data[index as usize] >> offset) & 0b1;
            total_chars += 1;
            continue
        }
        if first_bit == 0 && complex_bytes_left != 0 {
            panic!("Invalid UTF-8 string");
        }
        if first_bit == 1 && complex_bytes_left == 0 { // Start of a complex sequence!
            let mut depth = 0;
            for n in (2..7).rev() { // 2 to 6, for some reason
                if read_bit(*byte, n) == 1 && read_bit(*byte, n - 1) == 0 {
                    depth = 7 - n;
                    complex_bytes_left = depth;
                    let basic_codepoint = *byte as u32 & generate_mask((n - 1) as u32);

                    complex_value_so_far = basic_codepoint << (6 * depth);
                    break;
                }
            }

            if depth == 0 {
                panic!("invalid utf-8 string");
            }
            continue;
        }
        if first_bit == 1 && complex_bytes_left != 0 {
            if read_bit(*byte, 6) != 0 {
                panic!("invalid utf-8 string");
            }

            complex_bytes_left -= 1;

            complex_value_so_far += (*byte as u32 & generate_mask(6)) << (6 * complex_bytes_left);

            if complex_bytes_left == 0 {
                let index = complex_value_so_far >> 3;
                let offset = complex_value_so_far & 0b00000111;

                caps_chars += (data[index as usize] >> offset) & 0b1;
                total_chars += 1;
            }
        }
    }

    println!("{} caps characters out of {} total characters", caps_chars, total_chars);
}
