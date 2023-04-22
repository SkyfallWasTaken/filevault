# Header Info
## 10-character software identifier

A **10-character**, alphanumeric identifier that identifies the software.

## Key derivation function (KDF) identifier
A **3-character**, 8-bit number that identifies the key derivation function.
 - The code **must** be _exactly_ 3 characters long, e.g. `001`, `073` or `211`

## Salt
The salt for the file vault, encoded in Base64.
 - Currently `344` characters long. 