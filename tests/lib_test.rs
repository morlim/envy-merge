#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::collections::HashMap;
    use anyhow::Result;
    use envy_merge::{merge_env_files, read_env_file};

    fn get_test_file_path(filename: &str) -> String {
        Path::new("tests/test_data").join(filename).to_str().unwrap().to_string()
    }

    #[test]
    fn test_merge_env_files_basic() -> Result<()> {
        let env1 = get_test_file_path("env1.env");
        let env2 = get_test_file_path("env2.env");

        let result = merge_env_files(&[&env1, &env2], None)?;
        let expected = "BAZ=qux\nCOMMON=value2\nFOO=override\nNEW_VAR=new_value";

        let mut result_lines: Vec<&str> = result.split('\n').collect();
        result_lines.sort();
        let mut expected_lines: Vec<&str> = expected.split('\n').collect();
        expected_lines.sort();

        assert_eq!(result_lines, expected_lines);
        Ok(())
    }

    #[test]
    fn test_merge_env_files_with_priority() -> Result<()> {
        let env1 = get_test_file_path("env1.env");
        let env2 = get_test_file_path("env2.env");
        let priority = get_test_file_path("priority.env");

        let result = merge_env_files(&[&env1, &env2], Some(&priority))?;
        let expected = "FOO=priority\nBAZ=priority_baz\nCOMMON=value2\nNEW_VAR=new_value\nKEEP=this_value";

        let mut result_lines: Vec<&str> = result.split('\n').collect();
        result_lines.sort();
        let mut expected_lines: Vec<&str> = expected.split('\n').collect();
        expected_lines.sort();

        assert_eq!(result_lines, expected_lines);
        Ok(())
    }

    #[test]
    fn test_merge_env_files_with_empty_file() -> Result<()> {
        let env1 = get_test_file_path("env1.env");
        let empty_env = get_test_file_path("empty.env");

        let result = merge_env_files(&[&env1, &empty_env], None)?;
        let expected = "FOO=bar\nBAZ=qux\nCOMMON=value1";

        let mut result_lines: Vec<&str> = result.split('\n').collect();
        result_lines.sort();
        let mut expected_lines: Vec<&str> = expected.split('\n').collect();
        expected_lines.sort();

        assert_eq!(result_lines, expected_lines);
        Ok(())
    }

    #[test]
    fn test_merge_env_files_with_missing_file() -> Result<()> {
        let env1 = get_test_file_path("env1.env");
        let missing_file = "tests/test_data/missing.env".to_string(); // Simulating a missing file

        let result = merge_env_files(&[&env1, &missing_file], None)?;
        let expected = "FOO=bar\nBAZ=qux\nCOMMON=value1";

        let mut result_lines: Vec<&str> = result.split('\n').collect();
        result_lines.sort();
        let mut expected_lines: Vec<&str> = expected.split('\n').collect();
        expected_lines.sort();

        assert_eq!(result_lines, expected_lines);
        Ok(())
    }

    #[test]
    fn test_read_env_file() -> Result<()> {
        let env1 = get_test_file_path("env1.env");

        let vars = read_env_file(&env1)?;

        let mut expected_vars = HashMap::new();
        expected_vars.insert("FOO".to_string(), "bar".to_string());
        expected_vars.insert("BAZ".to_string(), "qux".to_string());
        expected_vars.insert("COMMON".to_string(), "value1".to_string());

        assert_eq!(vars, expected_vars);
        Ok(())
    }

    #[test]
    fn test_read_env_file_empty() -> Result<()> {
        let empty_file = get_test_file_path("empty.env");

        let vars = read_env_file(&empty_file)?;
        assert!(vars.is_empty());

        Ok(())
    }

    #[test]
    fn test_read_env_file_non_existent() -> Result<()> {
        let missing_file = "tests/test_data/missing.env".to_string();

        let vars = read_env_file(&missing_file)?;
        assert!(vars.is_empty());

        Ok(())
    }
}