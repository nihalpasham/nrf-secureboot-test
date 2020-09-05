#[path = "bl_cc310.rs"]
#[allow(dead_code, warnings)]
pub mod bl_cc310;

// #[allow(unused_imports)]
// use cortex_m_semihosting::hprintln;

// Constansts to hold CRYS HASH Module Error-codes 
pub const CRYS_HASH_INVALID_USER_CONTEXT_POINTER_ERROR:     u32  = bl_cc310::CRYS_HASH_MODULE_ERROR_BASE + 0x0;
pub const CRYS_HASH_ILLEGAL_OPERATION_MODE_ERROR:           u32  = bl_cc310::CRYS_HASH_MODULE_ERROR_BASE + 0x1;
pub const CRYS_HASH_USER_CONTEXT_CORRUPTED_ERROR:           u32  = bl_cc310::CRYS_HASH_MODULE_ERROR_BASE + 0x2;
pub const CRYS_HASH_DATA_IN_POINTER_INVALID_ERROR:          u32  = bl_cc310::CRYS_HASH_MODULE_ERROR_BASE + 0x3;
pub const CRYS_HASH_DATA_SIZE_ILLEGAL:                      u32  = bl_cc310::CRYS_HASH_MODULE_ERROR_BASE + 0x4;
pub const CRYS_HASH_INVALID_RESULT_BUFFER_POINTER_ERROR:    u32  = bl_cc310::CRYS_HASH_MODULE_ERROR_BASE + 0x5; 
pub const CRYS_HASH_LAST_BLOCK_ALREADY_PROCESSED_ERROR:     u32  = bl_cc310::CRYS_HASH_MODULE_ERROR_BASE + 0xc;
pub const CRYS_HASH_ILLEGAL_PARAMS_ERROR:                   u32  = bl_cc310::CRYS_HASH_MODULE_ERROR_BASE + 0xd; 
pub const CRYS_HASH_CTX_SIZES_ERROR:                        u32  = bl_cc310::CRYS_HASH_MODULE_ERROR_BASE + 0xe;
pub const CRYS_HASH_IS_NOT_SUPPORTED:                       u32  = bl_cc310::CRYS_HASH_MODULE_ERROR_BASE + 0xf; 

// Constansts to hold CRYS ECDSA Verifying Error-codes 
pub const CRYS_ECDSA_VERIFY_INVALID_DOMAIN_ID_ERROR:              u32 = bl_cc310::CRYS_ECPKI_MODULE_ERROR_BASE + 0x70;
pub const CRYS_ECDSA_VERIFY_INVALID_USER_CONTEXT_PTR_ERROR:       u32 = bl_cc310::CRYS_ECPKI_MODULE_ERROR_BASE + 0x71;
pub const CRYS_ECDSA_VERIFY_INVALID_SIGNER_PUBL_KEY_PTR_ERROR:    u32 = bl_cc310::CRYS_ECPKI_MODULE_ERROR_BASE + 0x72;
pub const CRYS_ECDSA_VERIFY_ILLEGAL_HASH_OP_MODE_ERROR:           u32 = bl_cc310::CRYS_ECPKI_MODULE_ERROR_BASE + 0x73;
pub const CRYS_ECDSA_VERIFY_INVALID_SIGNATURE_IN_PTR_ERROR:       u32 = bl_cc310::CRYS_ECPKI_MODULE_ERROR_BASE + 0x76;
pub const CRYS_ECDSA_VERIFY_INVALID_SIGNATURE_SIZE_ERROR:         u32 = bl_cc310::CRYS_ECPKI_MODULE_ERROR_BASE + 0x77;
pub const CRYS_ECDSA_VERIFY_INVALID_MESSAGE_DATA_IN_PTR_ERROR:    u32 = bl_cc310::CRYS_ECPKI_MODULE_ERROR_BASE + 0x80;
pub const CRYS_ECDSA_VERIFY_INVALID_MESSAGE_DATA_IN_SIZE_ERROR:   u32 = bl_cc310::CRYS_ECPKI_MODULE_ERROR_BASE + 0x81;
pub const CRYS_ECDSA_VERIFY_USER_CONTEXT_VALIDATION_TAG_ERROR:    u32 = bl_cc310::CRYS_ECPKI_MODULE_ERROR_BASE + 0x82;
pub const CRYS_ECDSA_VERIFY_SIGNER_PUBL_KEY_VALIDATION_TAG_ERROR: u32 = bl_cc310::CRYS_ECPKI_MODULE_ERROR_BASE + 0x83;
pub const CRYS_ECDSA_VERIFY_INCONSISTENT_VERIFY_ERROR:            u32 = bl_cc310::CRYS_ECPKI_MODULE_ERROR_BASE + 0x84;
pub const CRYS_ECC_ILLEGAL_PARAMS_ACCORDING_TO_PRIV_ERROR:        u32 = bl_cc310::CRYS_ECPKI_MODULE_ERROR_BASE + 0xD3;
pub const CRYS_ECC_ILLEGAL_HASH_MODE_ERROR:                       u32 = bl_cc310::CRYS_ECPKI_MODULE_ERROR_BASE + 0xE0;

#[derive(Debug)]
pub enum HashError  {}
pub enum ECDSAError {}

impl HashError {
    fn from_u32(value: u32) -> &'static str {
        match value {
            CRYS_HASH_INVALID_USER_CONTEXT_POINTER_ERROR    => ("CRYS HASH Module Error: Illegal context pointer."),
            CRYS_HASH_ILLEGAL_OPERATION_MODE_ERROR          => ("CRYS HASH Module Error: Illegal operation mode."),
            CRYS_HASH_USER_CONTEXT_CORRUPTED_ERROR          => ("CRYS HASH Module Error: HASH Context is corrupted."),
            CRYS_HASH_DATA_IN_POINTER_INVALID_ERROR         => ("CRYS HASH Module Error: Illegal data in pointer."),
            CRYS_HASH_DATA_SIZE_ILLEGAL                     => ("CRYS HASH Module Error: Illegal data in size."),
            CRYS_HASH_INVALID_RESULT_BUFFER_POINTER_ERROR   => ("CRYS HASH Module Error: Illegal result buffer pointer."),
            CRYS_HASH_LAST_BLOCK_ALREADY_PROCESSED_ERROR    => ("CRYS HASH Module Error: Last block was already processed (may happen if previous block was not a multiple of block size)."),
            CRYS_HASH_ILLEGAL_PARAMS_ERROR                  => ("CRYS HASH Module Error: Illegal parameter."),
            CRYS_HASH_CTX_SIZES_ERROR                       => ("CRYS HASH Module Error: Illegal context size."),
            CRYS_HASH_IS_NOT_SUPPORTED                      => ("CRYS HASH Module Error: HASH is not supported."), 
            _                                               => ("CRYS HASH Module Error: Unknown Error"),
        }
    }
}

impl ECDSAError {
    fn from_u32(value: u32) -> &'static str {
        match value {
            CRYS_ECDSA_VERIFY_INVALID_DOMAIN_ID_ERROR               => ("CRYS ECDSA Verifying Error: Illegal domain ID."),
            CRYS_ECDSA_VERIFY_INVALID_USER_CONTEXT_PTR_ERROR        => ("CRYS ECDSA Verifying Error: Illegal user's context pointer."),
            CRYS_ECDSA_VERIFY_INVALID_SIGNER_PUBL_KEY_PTR_ERROR     => ("CRYS ECDSA Verifying Error: Illegal public key pointer."),
            CRYS_ECDSA_VERIFY_ILLEGAL_HASH_OP_MODE_ERROR            => ("CRYS ECDSA Verifying Error: Illegal hash operation mode."),
            CRYS_ECDSA_VERIFY_INVALID_SIGNATURE_IN_PTR_ERROR        => ("CRYS ECDSA Verifying Error: Illegal signature pointer."),
            CRYS_ECDSA_VERIFY_INVALID_SIGNATURE_SIZE_ERROR          => ("CRYS ECDSA Verifying Error: Illegal signature size."),
            CRYS_ECDSA_VERIFY_INVALID_MESSAGE_DATA_IN_PTR_ERROR     => ("CRYS ECDSA Verifying Error: Illegal data in pointer."),
            CRYS_ECDSA_VERIFY_INVALID_MESSAGE_DATA_IN_SIZE_ERROR    => ("CRYS ECDSA Verifying Error: Illegal data in size."),
            CRYS_ECDSA_VERIFY_USER_CONTEXT_VALIDATION_TAG_ERROR     => ("CRYS ECDSA Verifying Error: Context validation failed."),
            CRYS_ECDSA_VERIFY_SIGNER_PUBL_KEY_VALIDATION_TAG_ERROR  => ("CRYS ECDSA Verifying Error: public key validation failed."), 
            CRYS_ECDSA_VERIFY_INCONSISTENT_VERIFY_ERROR             => ("CRYS ECDSA Verifying Error: Verification failed."),
            CRYS_ECC_ILLEGAL_PARAMS_ACCORDING_TO_PRIV_ERROR         => ("CRYS ECDSA Verifying Error: Illegal parameters."),
            CRYS_ECC_ILLEGAL_HASH_MODE_ERROR                        => ("CRYS ECDSA Verifying Error: Illegal hash mode."),
            _                                                       => ("CRYS ECDSA Verifying Error: Unknown Error")

        }
    }
}
// Start of test functions
#[allow(dead_code)]
pub fn get_test_hash() -> [u8; 32] {
    let mut hash: [u8; 32] = [0; 32];
    let hash_string = "5ca0c50bd7d94c7cfcfd5f702ab18cbddc2f1002e06811c104a2b91d6c3803c2";
    for i in (0..hash_string.len()).step_by(2) {
        if i > 62 {
            break;
        }
        // hprintln!("substring {:?}, {:?}", i, &hash_string[i..i+2]).unwrap();
        let substring = &hash_string[i..i + 2];
        let z = (u8::from_str_radix(substring, 16)).unwrap();
        // hprintln!("{:?}", z).unwrap();
        hash[(i - (i / 2))] = z;
        // hprintln!("i {:?}", &c[3..5]).unwrap()
    }
    // hprintln!("hash_bytes : {:?})", hash).unwrap();
    return hash;
}

#[allow(dead_code)]
pub fn get_test_signature() -> bl_cc310::nrf_cc310_bl_ecc_signature_secp256r1_t {
    let signature = bl_cc310::nrf_cc310_bl_ecc_signature_secp256r1_t {
        r: [
            0x8c, 0x37, 0xa5, 0x34, 0xd7, 0x96, 0xd5, 0x83, 0x4a, 0xbf, 0xd7, 0x3a, 0x8f, 0x7a,
            0x09, 0x11, 0xb1, 0x5c, 0xea, 0xc5, 0x0a, 0x34, 0xd1, 0xc5, 0xb3, 0xf7, 0x1b, 0xc8,
            0xc5, 0x52, 0xec, 0xa9,
        ],
        s: [
            0x32, 0x94, 0xb5, 0x0f, 0x98, 0xc9, 0xb9, 0xd2, 0x32, 0x89, 0x2b, 0x56, 0x23, 0x46,
            0x7e, 0x82, 0x31, 0x5a, 0x5d, 0x05, 0xa9, 0x97, 0x0c, 0x77, 0x07, 0x57, 0xb9, 0x29,
            0x23, 0x33, 0xb6, 0x69,
        ],
    };
    return signature;
}
// End of test functions

pub fn get_pubkey() -> bl_cc310::nrf_cc310_bl_ecc_public_key_secp256r1_t {
    let pub_key = bl_cc310::nrf_cc310_bl_ecc_public_key_secp256r1_t {
        x: [
            0x1f, 0xe6, 0x54, 0xc1, 0x80, 0x88, 0xe7, 0xfe, 0xf0, 0x84, 0xf9, 0x8a, 0x1a, 0x12,
            0xdb, 0x84, 0x69, 0x54, 0x34, 0x25, 0x06, 0xf5, 0x17, 0x69, 0x18, 0x9e, 0x3a, 0x90,
            0x79, 0x2f, 0xd3, 0x28,
        ],
        y: [
            0xcf, 0x51, 0x5d, 0x1e, 0x44, 0xbb, 0xa4, 0x9d, 0x34, 0xde, 0x3b, 0x99, 0xca, 0x4c,
            0x5e, 0x7e, 0xf4, 0x3a, 0xf6, 0xda, 0x41, 0x3c, 0x91, 0xc7, 0x98, 0x70, 0xd4, 0x87,
            0x68, 0xac, 0x74, 0x5b,
        ],
    };
    // hprintln!("pub_key :  {:?}", pub_key).unwrap();
    return pub_key;
}

pub fn get_signature(
    start_of_signature_address: u32,
) -> bl_cc310::nrf_cc310_bl_ecc_signature_secp256r1_t {
    unsafe {
        let signature = bl_cc310::nrf_cc310_bl_ecc_signature_secp256r1_t {
            r: *(start_of_signature_address as *const [u8; 32usize]),
            s: *((start_of_signature_address + 32) as *const [u8; 32usize]),
        };
        // hprintln!("signature: {:?}", signature).unwrap();
        return signature;
    }
}

pub fn calc_sha256_hash(
    start_of_firmware_address: u32,
) -> Result<*const bl_cc310::nrf_cc310_bl_hash_digest_sha256_t, &'static str> {
    // Structures to hold hash_ctx, hash_digest and length is the size (in bytes) of the firmware image.
    let mut hash_ctx = bl_cc310::nrf_cc310_bl_hash_context_sha256_t {
        init_val : 0,
        context_buffer : [0; 112]
    };
    let mut hash_digest: bl_cc310::nrf_cc310_bl_hash_digest_sha256_t = [0; 32];
    let length: u32 = 0x2d0;

    unsafe {

        let ret_val = bl_cc310::nrf_cc310_bl_hash_sha256_init(&mut hash_ctx);
        if ret_val == bl_cc310::CRYS_OK {
            let image = *(start_of_firmware_address as *const [u8; 720]); // pull image into RAM.
            let mut remaining = length as usize;
            let mut n: usize = 0x40; // sha256 takes a 512-bit block of data or 64 bytes at a time.
            let mut counter = 0;
            while remaining > 0 {
                if remaining < n {
                    n = remaining;
                }
                let data = &(image[counter]) as *const u8;
                let ret = bl_cc310::nrf_cc310_bl_hash_sha256_update(&mut hash_ctx, data, n as u32);
                if ret == bl_cc310::CRYS_OK {
                    remaining -= n;
                    counter += n;
                } else {
                    return Err(HashError::from_u32(ret))
                }        
            }
            let ret = bl_cc310::nrf_cc310_bl_hash_sha256_finalize(&mut hash_ctx, &mut hash_digest);
            if ret == bl_cc310::CRYS_OK {
                return Ok(&mut hash_digest);
            } else {
                return Err(HashError::from_u32(ret))
            }        
        } else {
            return Err(HashError::from_u32(ret_val));
        }
     
    }
}

pub fn cc310_ecdsa_verify(
    public_key: *const bl_cc310::nrf_cc310_bl_ecc_public_key_secp256r1_t,
    signature: *const bl_cc310::nrf_cc310_bl_ecc_signature_secp256r1_t,
    hash: *const u8,
    len: u32,
) -> Result<u32, &'static str>  {
    let mut signature_ctx = bl_cc310::nrf_cc310_bl_ecdsa_verify_context_secp256r1_t {
        init_val: 0,
        context_buffer: [0; 160usize],
    };
    unsafe {
        let ret = bl_cc310::nrf_cc310_bl_ecdsa_verify_secp256r1(
            &mut signature_ctx,
            public_key,
            signature,
            hash,
            len,
        );  
        if ret == bl_cc310::CRYS_OK {
            return Ok(ret)
        } else { 
            return Err(ECDSAError::from_u32(ret))
        }
    }
}
