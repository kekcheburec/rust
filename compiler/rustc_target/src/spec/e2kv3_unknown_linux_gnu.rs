use crate::spec::TargetResult;

pub fn target() -> TargetResult {
    let mut target = super::e2k64_unknown_linux_gnu::target()?;
    target.options.cpu = "elbrus-v3".to_string();
    Ok(target)
}