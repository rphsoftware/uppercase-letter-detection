use std::io::{Cursor, Read};

static mut DATA_CACHE: [u8; 139264] = [0; 139264];
static mut DATA_LOADED: bool = false;
static DATA_COMPRESSED: &str = &"X/8fAuJ6wHavgaCKEosRPKQDNsLWjtl5/VkLQUnwfL/fOuDtJbJpCWb530xnCE18OqHQSHho/BDxyLbdqpHhIg5uZJEFnNwkp5gKgK7qcb7eHDjn6hIkorr/AfbJ9odr7LapUzVrYCwU4pznHjDzPDo2TtD4PE8ca9WalHWMAVYgrBZqm3+C4NKSYVM1BNHkAABvfAAAQA2HABL8amUZnX3svth8uWqSS2TU01lhnMoo/3eDEULmnyoUWDiHFXCdEeqtD5EhgvUzjxCpd2ZOiErxJS5gIK+CltMGrzxi4wbK+MPPTPZkDqDAD2XbfxCi7BD54rjtK24A4Em5CRGTAIiIhxKwBIeStqQq2S9PAYB+S/6dIITzvD+DQLS/sH8Zi9UeNj7BurDZgrU7AABrNwGAa+vCF0i7Lt/kxsBpz2kPfQB0Wy/HzIE4meDAK/GVhSDYxRA/hPO7KtJTNd9hrYPXnQ+zAwI=";

pub unsafe fn get_data() -> &'static[u8; 139264] {
    if DATA_LOADED == true {
        return &DATA_CACHE;
    } else {
        // Decompress data
        let compressed = base64::decode(DATA_COMPRESSED.as_bytes())
            .expect("An error occured while decoding Base64");

        let mut compressed_cursor = Cursor::new(compressed);

        let mut input = brotli::Decompressor::new(&mut compressed_cursor, 4096);

        let mut decompressed_buffer = Vec::new();
        input.read_to_end(&mut decompressed_buffer).expect("Failed to de-brotli");

        DATA_CACHE.copy_from_slice(&*decompressed_buffer);
        DATA_LOADED = true;

        return &DATA_CACHE;
    }
}