use std::{collections::HashMap, path::PathBuf};

use crate::{args::CliArgs, lints::all_rules_and_safety};

#[derive(Clone)]
pub struct Config<'a> {
    /// Paths to files to lint.
    pub paths: Vec<PathBuf>,
    /// List of rules and whether they have an associated safe fix, passed by
    /// the user and/or recovered from the config file. Those will
    /// not necessarily all be used, for instance if we disable unsafe fixes.
    pub rules: HashMap<&'a str, bool>,
    /// List of rules to use. If we lint only, then this is equivalent to the
    /// field `rules`. If we apply fixes too, then this might be different from
    /// `rules` because it may filter out rules that have unsafe fixes.
    pub rules_to_apply: Vec<&'a str>,
    /// Did the user pass the --fix flag?
    pub should_fix: bool,
    /// Did the user pass the --unsafe-fixes flag?
    pub unsafe_fixes: bool,
}

pub fn build_config(args: &CliArgs, paths: Vec<PathBuf>) -> Config {
    let rules = parse_rules_cli(&args.rules);
    let rules_to_apply: Vec<&str> = if args.fix && !args.unsafe_fixes {
        rules
            .iter()
            .filter(|(_, v)| **v)
            .map(|(k, _)| *k)
            .collect::<Vec<&str>>()
    } else {
        rules.keys().copied().collect()
    };

    Config {
        paths,
        rules,
        rules_to_apply,
        should_fix: args.fix,
        unsafe_fixes: args.unsafe_fixes,
    }
}

pub fn parse_rules_cli(rules: &str) -> HashMap<&'static str, bool> {
    if rules.is_empty() {
        all_rules_and_safety()
    } else {
        let passed_by_user = rules.split(",").collect::<Vec<&str>>();
        all_rules_and_safety()
            .iter()
            .filter(|(k, _)| passed_by_user.contains(*k))
            .map(|(k, v)| (*k, *v))
            .collect::<HashMap<&'static str, bool>>()
    }
}
