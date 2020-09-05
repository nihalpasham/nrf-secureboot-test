

## This repository contains the following components:
 - A test boot-loader for an nrf52840 board. (soft-device not included for simplicity's sake)
 - Barebones key generator and image signing tools (requires python 3.x and OpenSSL)
 - Bare-metal test application (blinky)

 ## Repo Objectives:
 - Generate `rust bindings` with rust-bindgen for nRF52840's HW crypto accelerator i.e. the ARM CryptoCell 310.
 - Test authentication or verification of firmware image(s) using HW accelerated ECDSA or Elliptic Curve Digital Signature Algorithms (for boards that include ARM's Cryptocell IP.)
 - Assess the pros and cons of the CC310 on nRF boards. 


## Notes: Should NOT be used for real production work for the following reasons:
 - This implementation does not include a valid `chain of trust`
 - Implementation relies on a public key (used for verification) that's stored in the clear in firmware and easily accessible.
 - Although this implementation does not require dynamic memory allocation as the complete firmware image is pulled into RAM for signature verification, the size of the test firmware is only 720 bytes. CC310 requires data to be loaded into RAM (probably uses DMA) for all crypto processing work.

## Usage Instructions:
    - Compile both projects separately and flash the bootloader according to the memory layout specified in its `memory.x` file. 
    - Then flash the firmware image. It'll be loaded into memory at the memory offset 0x8000.
    - Upon reset, the board should run the bootloader, verify blinky firmware and jump to address 0x8000, where it loops over a blinking LED.
    
  **Signatures:**
    - There is a signatures folder that contains python scripts to generate ECC keys (private and public) and sign test firmware.
    - We use the private key and final build artefact `XX-blinky.bin` (stored in the examples folder of target directory) to generate a firmware signature. 
    - Finally, flash the signature generated onto the board at the address 0x82d0 using pyocd (or OpenOCD if you're comfortable with it). 
            - `pyocd flash -t nrf52840 --base-address 0x82d0 .\REAL_raw64byte_sig_gen_from_openssl.bin` 
    - I've experiemented with 2 options pyECDSA and pyOpenSSL (just as a double check) and both methods should work when generating signatures.
    - Note: OpenSSL generated signatures need some extra formatting. We'll need to extract the raw ECC signature (r + s) values from OpenSSL's DER formatted signature. (pyopenssl folder has 2 examples - use the one with `REAL suffix` for a quick test).
