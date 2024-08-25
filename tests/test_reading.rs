use anyhow::Result;
use filewatcher::paths::canonicalize_paths;

#[test]
fn test_valid_input() -> Result<()> {
    let p = vec![String::from(".")];
    canonicalize_paths(&p)?;
    Ok(())
}

#[test]
fn test_multiple_paths() -> Result<()> {
    let p = vec![String::from("./src"), String::from("./target")];
    canonicalize_paths(&p)?;
    Ok(())
}

#[test]
fn test_spaces_simple() -> Result<()> {
    let p = vec![String::from("./data/file with spaces.txt")];
    canonicalize_paths(&p)?;
    Ok(())
}

#[test]
fn test_spaces_complex() -> Result<()> {
    let p = vec![
        String::from("./data/directory/file.txt"),
        String::from("./data/directory with spaces/file with spaces without extension"),
    ];
    canonicalize_paths(&p)?;
    Ok(())
}
