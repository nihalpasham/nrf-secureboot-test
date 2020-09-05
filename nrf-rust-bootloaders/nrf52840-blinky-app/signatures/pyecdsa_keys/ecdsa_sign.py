
import hashlib
from ecdsa import SigningKey, VerifyingKey, NIST256p

path = "C:/Users/Nil/devspace/rust/projects/nrf52840-mdk-rs/target/thumbv7em-none-eabihf/release/examples/"

sk = SigningKey.generate(curve=NIST256p)
vk = sk.verifying_key
# with open("private.pem", "wb") as f:
#     f.write(sk.to_pem())
with open("public.pem", "wb") as f:
    f.write(vk.to_pem())
# Load the contents of the file to be signed.
bin_file = path + "s-blinky"
with open(bin_file, 'rb') as f:
    payload = f.read()

signature = sk.sign(payload, hashfunc=hashlib.sha256)

with open("sig.bin", "wb") as f:
   f.write(signature)

assert vk.verify(signature, payload, hashlib.sha256)
# signature = b"\x8c\x37\xa5\x34\xd7\x96\xd5\x83\x4a\xbf\xd7\x3a\x8f\x7a\x09\x11\xb1\x5c\xea\xc5\x0a\x34\xd1\xc5\xb3\xf7\x1b\xc8\xc5\x52\xec\xa9\x32\x94\xb5\x0f\x98\xc9\xb9\xd2\x32\x89\x2b\x56\x23\x46\x7e\x82\x31\x5a\x5d\x05\xa9\x97\x0c\x77\x07\x57\xb9\x29\x23\x33\xb6\x69"

# output_file=open("sig.bin", "wb")
# output_file.write(signature)

# output_file.close()
