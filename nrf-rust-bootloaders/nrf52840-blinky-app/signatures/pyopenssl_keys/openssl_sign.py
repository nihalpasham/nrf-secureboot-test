import hashlib
from ecdsa import SigningKey, VerifyingKey
from ecdsa.util import sigencode_der, sigdecode_der

path = "C:/Users/Nil/devspace/rust/projects/nrf52840-mdk-rs/target/thumbv7em-none-eabihf/release/examples/"

with open("ec-pub.pem") as f:
   vk = VerifyingKey.from_pem(f.read())

bin_file = path + "s-blinky"
with open(bin_file, "rb") as f:
   data = f.read()

with open("sig.bin", "rb") as f:
   signature = f.read()

assert vk.verify(signature, data, hashlib.sha256, sigdecode=sigdecode_der)

with open("ec-priv.pem") as f:
   sk = SigningKey.from_pem(f.read(), hashlib.sha256)

new_signature = sk.sign_deterministic(data)

with open("raw64byte_pyecdsa_gen_sig.bin", "wb") as f:
   f.write(new_signature)


# r = str(dict["0"])
# s = str(dict["1"])
# raw_sig = int(r + s)
# print("%X" % raw_sig)
# print out the key/value pairs embedded in the sequence in hexadecimal
# for k, v in seq.native.items():
#     print("%s => %X" % (k, v))
