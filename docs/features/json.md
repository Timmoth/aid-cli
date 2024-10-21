## aid json

### aid json jwt-decode
```
  aid json jwt-decode   Decode a JWT
            -j, --jwt <TOKEN>  Specify JWT to decode.

-----input-----
aid json jwt-decode -j eyJ0eXAiOiJKV1QiLCJhbGciOiJFUzI1NiIsImtpZCI6IjM5OWZkM2E5MmI3YTJiNDZjMzQzMDNiOTViOGNhMmMyIn0.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNzI5NTQ2MzUxfQ.BSvGKZUlSPDWtnVjeJG45eUz1JqYZbBYVKPp4EiV23gs8hE92LvnlxnfnZP-QNfb1JTFCPikQKmkAhp5QInTDg
-----output-----
{
  "header": {
    "alg": "ES256",
    "kid": "399fd3a92b7a2b46c34303b95b8ca2c2",
    "typ": "JWT"
  },
  "payload": {
    "iat": 1729546351,
    "name": "John Doe",
    "sub": "1234567890"
  }
}
```