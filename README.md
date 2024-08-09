# Encrypt Static HTML

A tool for encrypting a static HTML page, including its images and CSS, allowing for the page to be hosted securely on a static web server.

## Usage

```shell
# Encrypt the page "index.html" with the password "test".
> cargo run -- --src=index.html --dst=index_encrypted.html --password=test
```

See `examples/encrypt.{sh,bat}` for a more complete example.

## Help

```shell
> cargo run -- --help
Usage: encrypt_static_html.exe [OPTIONS] --src <SRC> --dst <DST> --password <PASSWORD>

Options:
  -s, --src <SRC>            Path to input html file to encrypt
  -d, --dst <DST>            Path to output encrypted html file
  -p, --password <PASSWORD>  Encryption password
      --embed <EMBED>        If true (default), embed the images and css into the html. If false, external images and css remain external [default: true] [possible values: true, false]
      --encrypt <ENCRYPT>    If true (default), encrypt the html content (including the embedded assets). If false, the exported html is non encrypted [default: true] [possible values: true, false]
      --message <MESSAGE>    Password request text [default: "Enter the password to view the page"]
      --title <TITLE>        Title of the password request page [default: "My webpage"]
  -h, --help                 Print help
  -V, --version              Print version
```
