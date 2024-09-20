Verify Digest can read digests generated with openssl. It can also read digests generated with
md5sum and shasum formated using the `--tag` parameter. It writes digest files using the openssl format.
Openssl represents digest algorithms slightly differently than the other two programs.

| openssl     |              | shasum |            | md5sum |
|-------------|--------------|--------|------------|--------|
| -md5        | MD5          |        |            | MD5    |
| -md5-sha1   | MD5-SHA1*    |        |            |        |
| -sha1       | SHA1         | 1      | SHA1       |        |
| -sha224     | SHA2-224     | 224    | SHA224     |        |
| -sha256     | SHA2-256     | 256    | SHA256     |        |
| -sha384     | SHA2-384     | 384    | SHA384     |        |
| -sha512     | SHA2-512     | 512    | SHA512     |        |
| -sha512-224 | SHA2-512/224 | 512224 | SHA512/224 |        |
| -sha512-256 | SHA2-512/256 | 512256 | SHA512/256 |        |
| -sha3-224   | SHA3-224     |        |            |        |
| -sha3-256   | SHA3-256     |        |            |        |
| -sha3-384   | SHA3-384     |        |            |        |
| -sha3-512   | SHA3-512     |        |            |        |

\* This appears to be the MD5 digest with the SHA1 digest appended to it.
So far, we don't support it.