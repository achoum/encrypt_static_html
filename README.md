# Encrypt Static HTML

A tool for encrypting a static HTML page, including its images and CSS, allowing for the page to be hosted securely on a static web server.

## Usage

**Encrypt Static HTML** (ESH) is available both as a CLI (i.e., you type commands in a terminal) and a GUI (i.e., a window with some buttons). In both cases, first install Rust and ESH:

**1.** Install Rustup at <https://www.rust-lang.org/tools/install>. For example:

```shell
# Install Rust on Linux or MacOs
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**2. (optional)** Add Cargo's bin to your PATH. Instructions should be printed when running the previous command. For example:

```shell
# Add Cargo's bin to the PATH on Linux
export PATH=$PATH:/home/$USER/.cargo/bin/
```

**3.** Install and run `encrypt_static_html`.

```shell
cargo install --git https://github.com/achoum/encrypt_static_html
```

**Note:** This command install both the CLI and GUI version of ESH. If you only want the CLI version (the GUI version is significantly larger), adds `--bin=cli` to the `cargo install` command above.

### CLI

Given an HTML page `index.html`, encode it with:

```shell
encrypt_static_html --src=index.html --dst=index_encrypted.html --password=test
```

**Note:** See `examples` for a full example.

### GUI

Launch the GUI with:

```shell
encrypt_static_html_ui 
```

**Note:** The GUI exposes a subset of the options of the CLI.

## CLI Help

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
