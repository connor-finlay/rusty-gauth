# rusty-gauth

## Usage:

* In Google Authenticator -> Menu -> Transfer Accunts -> Export Accounts
* Save QR-code to a file and pass the path to `rusty-gauth`
* Alternatively, extract the link from the QR-code first and pass the link to `rusty-gauth`

```shell
A command line tool to decode Google Authenticator export data

Usage: rusty-gauth [OPTIONS]

Options:
  -p, --path <img file>
          Path to image containing QR-code

  -l, --link <link>
          Migration link decoded from QR-code, starts with: otpauth-migration://offline?data=

  -h, --help
          Print help
```

Decode backup directly from QR code:

```shell
./rusty-gauth -p ~/Downloads/gauth-backup.png
+-----------------------+-------------------+------------------+-----------+------------+---------+
| Name                  | Issuer            | Secret           | Algorithm | DigitCount | OtpType |
+=================================================================================================+
| Cherry's Hack The Box | Hack+The+Box      | MNUGK4TS         | Sha1      | 6          | TOTP    |
|-----------------------+-------------------+------------------+-----------+------------+---------|
| Frankie's Github      | Github            | MZZGC3TL         | Sha1      | 6          | TOTP    |
|-----------------------+-------------------+------------------+-----------+------------+---------|
| Mikey's Steam         | Steam             | ON2GKYLN         | Sha1      | 6          | TOTP    |
|-----------------------+-------------------+------------------+-----------+------------+---------|
| Johnny B. Goode       | Gmail             | NJXWQ3TOPFRGO33P | Sha1      | 6          | TOTP    |
|-----------------------+-------------------+------------------+-----------+------------+---------|
| My outlook            | Microsoft+Outlook | OBQXA2LD         | Sha1      | 6          | TOTP    |
|-----------------------+-------------------+------------------+-----------+------------+---------|
| @finnersio            | X                 | MZUW43TF         | Sha1      | 6          | TOTP    |
+-----------------------+-------------------+------------------+-----------+------------+---------+
```

Generate backup from link decoded from QR code:

```shell
./rusty-gauth -l "otpauth-migration://offline?data=CjIKBWNoZXJyEhVDaGVycnkncyBIYWNrIFRoZSBCb3gaDEhhY2srVGhlK0JveCABKAEwAgonCgVmcmFuaxIQRnJhbmtpZSdzIEdpdGh1YhoGR2l0aHViIAEoATACCiMKBXN0ZWFtEg1NaWtleSdzIFN0ZWFtGgVTdGVhbSABKAEwAgoqCgpqb2hubnliZ29vEg9Kb2hubnkgQi4gR29vZGUaBUdtYWlsIAEoATACCiwKBXBhcGljEgpNeSBvdXRsb29rGhFNaWNyb3NvZnQrT3V0bG9vayABKAEwAgocCgVmaW5uZRIKQGZpbm5lcnNpbxoBWCABKAEwAhACGAEgAA%3D%3D"
+-----------------------+-------------------+------------------+-----------+------------+---------+
| Name                  | Issuer            | Secret           | Algorithm | DigitCount | OtpType |
+=================================================================================================+
| Cherry's Hack The Box | Hack+The+Box      | MNUGK4TS         | Sha1      | 6          | TOTP    |
|-----------------------+-------------------+------------------+-----------+------------+---------|
| Frankie's Github      | Github            | MZZGC3TL         | Sha1      | 6          | TOTP    |
|-----------------------+-------------------+------------------+-----------+------------+---------|
| Mikey's Steam         | Steam             | ON2GKYLN         | Sha1      | 6          | TOTP    |
|-----------------------+-------------------+------------------+-----------+------------+---------|
| Johnny B. Goode       | Gmail             | NJXWQ3TOPFRGO33P | Sha1      | 6          | TOTP    |
|-----------------------+-------------------+------------------+-----------+------------+---------|
| My outlook            | Microsoft+Outlook | OBQXA2LD         | Sha1      | 6          | TOTP    |
|-----------------------+-------------------+------------------+-----------+------------+---------|
| @finnersio            | X                 | MZUW43TF         | Sha1      | 6          | TOTP    |
+-----------------------+-------------------+------------------+-----------+------------+---------+
```
