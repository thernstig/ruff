//! Module for Ruff-specific rules.

pub mod checks;

#[cfg(test)]
mod tests {
    use std::convert::AsRef;
    use std::path::Path;

    use anyhow::Result;
    use test_case::test_case;

    use crate::checks::CheckCode;
    use crate::linter::test_path;
    use crate::settings;

    #[test_case(CheckCode::RUF001, Path::new("RUF001.py"); "RUF001")]
    #[test_case(CheckCode::RUF002, Path::new("RUF002.py"); "RUF002")]
    #[test_case(CheckCode::RUF003, Path::new("RUF003.py"); "RUF003")]
    fn checks(check_code: CheckCode, path: &Path) -> Result<()> {
        let snapshot = format!("{}_{}", check_code.as_ref(), path.to_string_lossy());
        let mut checks = test_path(
            Path::new("./resources/test/fixtures/ruff")
                .join(path)
                .as_path(),
            &settings::Settings::for_rule(check_code),
            true,
        )?;
        checks.sort_by_key(|check| check.location);
        insta::assert_yaml_snapshot!(snapshot, checks);
        Ok(())
    }

    #[test]
    fn ruf100() -> Result<()> {
        let mut checks = test_path(
            Path::new("./resources/test/fixtures/ruff/RUF100.py"),
            &settings::Settings::for_rules(vec![
                CheckCode::RUF100,
                CheckCode::E501,
                CheckCode::F841,
            ]),
            true,
        )?;
        checks.sort_by_key(|check| check.location);
        insta::assert_yaml_snapshot!(checks);
        Ok(())
    }

    #[test]
    fn flake8_noqa() -> Result<()> {
        let mut checks = test_path(
            Path::new("./resources/test/fixtures/ruff/flake8_noqa.py"),
            &settings::Settings::for_rules(vec![CheckCode::F401, CheckCode::F841]),
            true,
        )?;
        checks.sort_by_key(|check| check.location);
        insta::assert_yaml_snapshot!(checks);
        Ok(())
    }

    #[test]
    fn ruff_noqa() -> Result<()> {
        let mut checks = test_path(
            Path::new("./resources/test/fixtures/ruff/ruff_noqa.py"),
            &settings::Settings::for_rules(vec![CheckCode::F401, CheckCode::F841]),
            true,
        )?;
        checks.sort_by_key(|check| check.location);
        insta::assert_yaml_snapshot!(checks);
        Ok(())
    }

    #[test]
    fn redirects() -> Result<()> {
        let mut checks = test_path(
            Path::new("./resources/test/fixtures/ruff/redirects.py"),
            &settings::Settings::for_rules(vec![CheckCode::UP007]),
            true,
        )?;
        checks.sort_by_key(|check| check.location);
        insta::assert_yaml_snapshot!(checks);
        Ok(())
    }
}
