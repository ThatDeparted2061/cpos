//! Local workspace management: where solution files live on disk, plus
//! scaffolding new solutions from a per-language template.
//!
//! The workspace stays clean — just one flat source file per problem:
//!
//! ```text
//! cpos/
//!   codeforces/
//!     1095F.cpp
//!     1A.cpp
//!   cses/
//!     1068.cpp
//!     WeirdAlgorithm.cpp
//!   templates/
//!     template.cpp
//! ```
//!
//! Sample tests and compiler build artifacts are kept out of the workspace,
//! in the app's data directory, so they never clutter the user's folders.

use std::path::PathBuf;

use anyhow::Result;

use crate::data::config::Config;
use crate::data::models::{Platform, Problem, TestCase};

/// Expand a leading `~/` to the user's home directory.
pub fn expand_tilde(s: &str) -> PathBuf {
    if let Some(rest) = s.strip_prefix("~/") {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(rest)
    } else {
        PathBuf::from(s)
    }
}

/// Root directory that holds all per-problem workspaces.
pub fn root(config: &Config) -> PathBuf {
    config
        .workspace_dir
        .as_ref()
        .filter(|s| !s.trim().is_empty())
        .map(|s| expand_tilde(s))
        .unwrap_or_else(|| {
            dirs::home_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("cpos")
        })
}

fn platform_slug(platform: Platform) -> &'static str {
    match platform {
        Platform::Codeforces => "codeforces",
        Platform::Cses => "cses",
        Platform::AtCoder => "atcoder",
    }
}

/// Directory that holds a platform's flat list of solution files.
pub fn platform_dir(config: &Config, platform: Platform) -> PathBuf {
    root(config).join(platform_slug(platform))
}

/// A filesystem-safe version of a problem id (CF/CSES ids are already safe,
/// but AtCoder and pasted ids can contain slashes/colons).
fn safe_id(id: &str) -> String {
    id.chars()
        .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else { '_' })
        .collect()
}

/// Readable PascalCase slug from a problem title — used for CSES filenames.
fn slug_from_name(name: &str) -> String {
    name.split_whitespace()
        .filter_map(|word| {
            let clean: String = word.chars().filter(|c| c.is_alphanumeric()).collect();
            if clean.is_empty() {
                return None;
            }
            let mut chars = clean.chars();
            let first = chars.next()?.to_uppercase().collect::<String>();
            Some(format!("{}{}", first, chars.as_str().to_lowercase()))
        })
        .collect()
}

fn solution_basename(problem: &Problem) -> String {
    match problem.platform {
        Platform::Cses => {
            let slug = slug_from_name(&problem.name);
            if slug.is_empty() {
                safe_id(&problem.id)
            } else {
                slug
            }
        }
        _ => safe_id(&problem.id),
    }
}

/// Path to the solution source file for a problem in the given language —
/// a single flat file like `codeforces/1095F.cpp` or `cses/WeirdAlgorithm.cpp`.
pub fn solution_path(config: &Config, problem: &Problem, ext: &str) -> PathBuf {
    platform_dir(config, problem.platform).join(format!("{}.{}", solution_basename(problem), ext))
}

/// Path to the JSON file that caches sample tests — kept in the data dir,
/// outside the user's workspace.
pub fn tests_path(config: &Config, problem: &Problem) -> PathBuf {
    let _ = config;
    Config::data_dir()
        .join("tests")
        .join(platform_slug(problem.platform))
        .join(format!("{}.json", safe_id(&problem.id)))
}

/// Create the platform directory and a solution file from the template if one
/// doesn't already exist. Never overwrites existing user code.
pub fn scaffold(config: &Config, problem: &Problem, ext: &str, template: &str) -> Result<PathBuf> {
    let dir = platform_dir(config, problem.platform);
    std::fs::create_dir_all(&dir)?;
    let path = solution_path(config, problem, ext);
    if !path.exists() {
        std::fs::write(&path, template)?;
    }
    Ok(path)
}

/// Persist sample tests to disk so the runner can use them offline.
pub fn save_tests(config: &Config, problem: &Problem, tests: &[TestCase]) -> Result<()> {
    let path = tests_path(config, problem);
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    let json = serde_json::to_string_pretty(tests)?;
    std::fs::write(path, json)?;
    Ok(())
}

/// Load previously-saved sample tests, or an empty list if none exist.
pub fn load_tests(config: &Config, problem: &Problem) -> Vec<TestCase> {
    std::fs::read_to_string(tests_path(config, problem))
        .ok()
        .and_then(|s| serde_json::from_str::<Vec<TestCase>>(&s).ok())
        .unwrap_or_default()
}

/// Resolve the template to scaffold with: the user's configured template file
/// if set and readable, otherwise the built-in per-language template.
pub fn template_content(config: &Config, lang: &str) -> String {
    if let Some(path) = config
        .template_file
        .as_ref()
        .filter(|s| !s.trim().is_empty())
    {
        if let Ok(content) = std::fs::read_to_string(expand_tilde(path)) {
            return content;
        }
    }
    template_for(lang)
}

/// A minimal starter template for the given language key.
pub fn template_for(lang: &str) -> String {
    match lang {
        "c" => "#include <stdio.h>\n\nint main(void) {\n\n    return 0;\n}\n".to_string(),
        "cpp" => "#include <bits/stdc++.h>\nusing namespace std;\n\nint main() {\n    ios::sync_with_stdio(false);\n    cin.tie(nullptr);\n\n    return 0;\n}\n"
            .to_string(),
        "python" | "pypy" => "import sys\ninput = sys.stdin.readline\n\n\ndef main():\n    pass\n\n\nif __name__ == \"__main__\":\n    main()\n"
            .to_string(),
        // The class is package-private (no `public`), so the file can be named
        // anything (problem ids start with digits, which Java forbids for the
        // file name only when the class is public).
        "java" => "import java.util.*;\nimport java.io.*;\n\nclass Main {\n    public static void main(String[] args) throws IOException {\n        BufferedReader br = new BufferedReader(new InputStreamReader(System.in));\n\n    }\n}\n"
            .to_string(),
        "rust" => "use std::io::{self, Read, Write};\n\nfn main() {\n    let mut input = String::new();\n    io::stdin().read_to_string(&mut input).unwrap();\n    let mut it = input.split_whitespace();\n\n}\n"
            .to_string(),
        "go" => "package main\n\nimport (\n\t\"bufio\"\n\t\"fmt\"\n\t\"os\"\n)\n\nfunc main() {\n\treader := bufio.NewReader(os.Stdin)\n\twriter := bufio.NewWriter(os.Stdout)\n\tdefer writer.Flush()\n\t_ = reader\n\t_ = fmt.Fprintln\n}\n"
            .to_string(),
        "kotlin" => "import java.io.BufferedReader\nimport java.io.InputStreamReader\n\nfun main() {\n    val br = BufferedReader(InputStreamReader(System.`in`))\n\n}\n"
            .to_string(),
        "csharp" => "using System;\nusing System.IO;\n\nclass Main {\n    static void Main() {\n        var input = Console.In;\n\n    }\n}\n"
            .to_string(),
        "javascript" => "const data = require('fs').readFileSync(0, 'utf8');\nconst lines = data.split('\\n');\nlet idx = 0;\nconst next = () => lines[idx++];\n\n"
            .to_string(),
        "ruby" => "# read input with gets / STDIN.read\n\n".to_string(),
        "haskell" => "import Data.List\n\nmain :: IO ()\nmain = do\n    contents <- getContents\n    let ws = words contents\n    return ()\n"
            .to_string(),
        "pascal" => "program solution;\nbegin\n\nend.\n".to_string(),
        _ => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::models::{Platform, SolveStatus};

    fn cses_problem(id: &str, name: &str) -> Problem {
        Problem {
            platform: Platform::Cses,
            id: id.to_string(),
            name: name.to_string(),
            url: format!("https://cses.fi/problemset/task/{id}"),
            rating: None,
            tags: vec![],
            category: None,
            solved_count: None,
            status: SolveStatus::Unsolved,
        }
    }

    #[test]
    fn cses_slug_from_name() {
        assert_eq!(slug_from_name("Weird Algorithm"), "WeirdAlgorithm");
        assert_eq!(slug_from_name("Coin Combinations I"), "CoinCombinationsI");
        assert_eq!(slug_from_name("Sum of Two Values"), "SumOfTwoValues");
    }

    #[test]
    fn cses_solution_uses_slug() {
        let p = cses_problem("1068", "Weird Algorithm");
        let path = solution_path(&Config::default(), &p, "cpp");
        assert!(path.to_string_lossy().ends_with("cses/WeirdAlgorithm.cpp"));
    }

    #[test]
    fn codeforces_solution_uses_id() {
        let p = Problem {
            platform: Platform::Codeforces,
            id: "2232F".to_string(),
            name: "Magical Tiered Cake".to_string(),
            url: "https://codeforces.com/contest/2232/problem/F".to_string(),
            rating: None,
            tags: vec![],
            category: None,
            solved_count: None,
            status: SolveStatus::Unsolved,
        };
        let path = solution_path(&Config::default(), &p, "cpp");
        assert!(path.to_string_lossy().ends_with("codeforces/2232F.cpp"));
    }
}
