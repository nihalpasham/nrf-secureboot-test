# Extract the raw 32 byte values of 'r and s' from OpenSSL's DER formatted signature. bytelen('r + s') == 64 
from asn1crypto.core import Sequence
import binascii

raw64byte_sig = ''
with open("REAL_DERformat_openssl_gen_sig.bin", "rb") as f:
   signature = f.read()
# parse the ASN.1 sequence from this signature
seq = Sequence.load(signature)
# print the native (Pythonic) representation of this ASN.1 object
dict = seq.native
for k,v in dict.items():
      hexed = hex(v).strip('0x')
      # print(hexed)
      raw64byte_sig += hexed

# print(raw64byte_sig)
with open("REAL_raw64byte_sig_gen_from_openssl.bin", "wb") as f:
   f.write(binascii.unhexlify(raw64byte_sig))