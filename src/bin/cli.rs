use clap::Parser;
use encrypt_static_html::encrypt_html;

/// A tool for encrypting a static HTML page, including its images, CSS, and JavaScript, allowing for the page to be hosted securely on a static web server.
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Path to input html file to encrypt.
    #[arg(short, long)]
    src: std::path::PathBuf,

    /// Path to output encrypted html file.
    #[arg(short, long)]
    dst: std::path::PathBuf,

    /// Encryption password.
    #[arg(short, long)]
    password: String,

    /// If true (default), embed the images and css into the html. If false, external images and css remain external.
    #[arg(long, default_value_t = true)]
    embed: std::primitive::bool,

    /// If true (default), encrypt the html content (including the embedded assets). If false, the exported html is non encrypted.
    #[arg(long, default_value_t = true)]
    encrypt: std::primitive::bool,

    /// Password request text.
    #[arg(long, default_value = "Enter the password to view the page")]
    message: String,

    /// Title of the password request page.
    #[arg(long, default_value = "My webpage")]
    title: String,
}

fn main() {
    let args = Args::parse();

    encrypt_html(
        args.src.as_path(),
        &args.password,
        args.dst.as_path(),
        args.embed,
        args.encrypt,
        &args.message,
        &args.title,
    )
    .unwrap();
}
