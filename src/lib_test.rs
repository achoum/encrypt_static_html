use super::*;
use rstest::rstest;
use std::fs;
use tempfile;

#[rstest]
#[case(true, true)]
#[case(false, true)]
#[case(true, false)]
#[case(false, false)]
fn test_e2e(#[case] embed: bool, #[case] encrypt: bool) {
    let src_content = "<html><body>hello the world</body></html>";
    let tmp_dir = tempfile::tempdir().unwrap().into_path();
    let src = tmp_dir.join("src.html");
    let dst = tmp_dir.join("dst.html");
    fs::write(&src, &src_content).unwrap();
    encrypt_html(&src, "test", &dst, embed, encrypt, "my message", "my title").unwrap();
}
