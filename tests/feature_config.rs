use assert_cmd::{assert::Assert, Command};

static SUBCOMMAND_ARGS_INDEX: usize = 1;

struct CargoSemverChecks {
    args: Vec<String>,
}

impl CargoSemverChecks {
    fn new(current_path: &str, baseline_path: &str) -> Self {
        Self {
            args: vec![
                String::from("semver-checks"),
                String::from("check-release"),
                format!("--manifest-path={current_path}"),
                format!("--baseline-root={baseline_path}"),
            ],
        }
    }
    
    fn command(&self) -> Command {
        Command::cargo_bin("cargo-semver-checks").unwrap()

    }

    fn add_arg(&mut self, arg: &str) -> &mut Self {
        self.args.push(String::from(arg));
        self
    }

    fn run_all(&mut self) -> Vec<Assert> {
        vec![self.run_without_subcommand(), self.run_with_subcommand()]
    }

    fn run_without_subcommand(&mut self) -> Assert {
        let mut args = self.args.clone();
        args.remove(SUBCOMMAND_ARGS_INDEX);
        self.command().args(&args).assert()
    }

    fn run_with_subcommand(&mut self) -> Assert {
        self.command().args(&self.args).assert()
    }
}

#[test]
fn simple_only_explicit_feature() {
    CargoSemverChecks::new(
        "test_crates/features_simple/new/",
        "test_crates/features_simple/old/Cargo.toml",
    )
    .add_arg("--only-explicit-features")
    .run_all()
    .into_iter()
    .for_each(|a| {
        a.success();
    });
}

#[test]
fn simple_default_features() {
    CargoSemverChecks::new(
        "test_crates/features_simple/new/",
        "test_crates/features_simple/old/Cargo.toml",
    )
    .add_arg("--default-features")
    .run_all()
    .into_iter()
    .for_each(|a| {
        a.failure();
    });
}

#[test]
fn simple_heuristic_features() {
    CargoSemverChecks::new(
        "test_crates/features_simple/new/",
        "test_crates/features_simple/old/Cargo.toml",
    )
    // make sure 'foo' is added to current
    .add_arg("--baseline-features")
    .add_arg("foo")
    .run_all()
    .into_iter()
    .for_each(|a| {
        a.success();
    });
}

#[test]
fn simple_all_features() {
    CargoSemverChecks::new(
        "test_crates/features_simple/new/",
        "test_crates/features_simple/old/Cargo.toml",
    )
    .add_arg("--all-features")
    .run_all()
    .into_iter()
    .for_each(|a| {
        a.failure();
    });
}

#[test]
fn function_moved_only_explicit_features() {
    CargoSemverChecks::new(
        "test_crates/function_feature_changed/new/",
        "test_crates/function_feature_changed/old/Cargo.toml",
    )
    .add_arg("--only-explicit-features")
    .run_all()
    .into_iter()
    .for_each(|a| {
        a.success();
    });

    CargoSemverChecks::new(
        "test_crates/function_feature_changed/new/",
        "test_crates/function_feature_changed/old/Cargo.toml",
    )
    .add_arg("--only-explicit-features")
    .add_arg("--baseline-features")
    .add_arg("C")
    .run_all()
    .into_iter()
    .for_each(|a| {
        a.success();
    });

    CargoSemverChecks::new(
        "test_crates/function_feature_changed/new/",
        "test_crates/function_feature_changed/old/Cargo.toml",
    )
    .add_arg("--only-explicit-features")
    .add_arg("--baseline-features")
    .add_arg("A")
    .add_arg("--current-features")
    .add_arg("B")
    .run_all()
    .into_iter()
    .for_each(|a| {
        a.success();
    });

    CargoSemverChecks::new(
        "test_crates/function_feature_changed/new/",
        "test_crates/function_feature_changed/old/Cargo.toml",
    )
    .add_arg("--only-explicit-features")
    .add_arg("--features")
    .add_arg("B")
    .run_all()
    .into_iter()
    .for_each(|a| {
        a.failure();
    });

    CargoSemverChecks::new(
        "test_crates/function_feature_changed/new/",
        "test_crates/function_feature_changed/old/Cargo.toml",
    )
    .add_arg("--only-explicit-features")
    .add_arg("--features")
    .add_arg("A")
    .add_arg("--features")
    .add_arg("B")
    .add_arg("--features")
    .add_arg("C")
    .run_all()
    .into_iter()
    .for_each(|a| {
        a.success();
    });
}

#[test]
fn function_moved_default_features() {
    CargoSemverChecks::new(
        "test_crates/function_feature_changed/new/",
        "test_crates/function_feature_changed/old/Cargo.toml",
    )
    .add_arg("--default-features")
    .run_all()
    .into_iter()
    .for_each(|a| {
        a.failure();
    });

    CargoSemverChecks::new(
        "test_crates/function_feature_changed/new/",
        "test_crates/function_feature_changed/old/Cargo.toml",
    )
    .add_arg("--default-features")
    .add_arg("--current-features")
    .add_arg("B")
    .run_all()
    .into_iter()
    .for_each(|a| {
        a.success();
    });

    CargoSemverChecks::new(
        "test_crates/function_feature_changed/new/",
        "test_crates/function_feature_changed/old/Cargo.toml",
    )
    .add_arg("--default-features")
    .add_arg("--features")
    .add_arg("B")
    .run_all()
    .into_iter()
    .for_each(|a| {
        a.failure();
    });

    CargoSemverChecks::new(
        "test_crates/function_feature_changed/new/",
        "test_crates/function_feature_changed/old/Cargo.toml",
    )
    .add_arg("--default-features")
    .add_arg("--features")
    .add_arg("B")
    .add_arg("--current-features")
    .add_arg("C")
    .run_all()
    .into_iter()
    .for_each(|a| {
        a.success();
    });
}

#[test]
fn function_moved_heuristic_features() {
    CargoSemverChecks::new(
        "test_crates/function_feature_changed/new/",
        "test_crates/function_feature_changed/old/Cargo.toml",
    )
    .run_all()
    .into_iter()
    .for_each(|a| {
        a.success();
    });
}

#[test]
fn function_moved_all_features() {
    CargoSemverChecks::new(
        "test_crates/function_feature_changed/new/",
        "test_crates/function_feature_changed/old/Cargo.toml",
    )
    .add_arg("--all-features")
    .run_all()
    .into_iter()
    .for_each(|a| {
        a.success();
    });
}

#[test]
fn default_features_when_default_undefined() {
    CargoSemverChecks::new(
        "test_crates/features_no_default/new/",
        "test_crates/features_no_default/old/Cargo.toml",
    )
    .add_arg("--default-features")
    .run_all()
    .into_iter()
    .for_each(|a| {
        a.success();
    });

    CargoSemverChecks::new(
        "test_crates/features_no_default/new/",
        "test_crates/features_no_default/old/Cargo.toml",
    )
    .add_arg("--default-features")
    .add_arg("--features")
    .add_arg("A")
    .run_all()
    .into_iter()
    .for_each(|a| {
        a.success();
    });

    CargoSemverChecks::new(
        "test_crates/features_no_default/new/",
        "test_crates/features_no_default/old/Cargo.toml",
    )
    .add_arg("--default-features")
    .add_arg("--baseline-features")
    .add_arg("A")
    .run_all()
    .into_iter()
    .for_each(|a| {
        a.success();
    });

    CargoSemverChecks::new(
        "test_crates/features_no_default/new/",
        "test_crates/features_no_default/old/Cargo.toml",
    )
    .add_arg("--default-features")
    .add_arg("--current-features")
    .add_arg("B")
    .run_all()
    .into_iter()
    .for_each(|a| {
        a.success();
    });

    CargoSemverChecks::new(
        "test_crates/features_no_default/new/",
        "test_crates/features_no_default/old/Cargo.toml",
    )
    .add_arg("--default-features")
    .add_arg("--features")
    .add_arg("B")
    .run_all()
    .into_iter()
    .for_each(|a| {
        a.failure();
    });
}

#[test]
fn feature_does_not_exist() {
    CargoSemverChecks::new(
        "test_crates/function_feature_changed/new/",
        "test_crates/function_feature_changed/old/Cargo.toml",
    )
    .add_arg("--features")
    .add_arg("new_feature")
    .run_all()
    .into_iter()
    .for_each(|a| {
        a.success();
    });

    CargoSemverChecks::new(
        "test_crates/function_feature_changed/new/",
        "test_crates/function_feature_changed/old/Cargo.toml",
    )
    .add_arg("--features")
    .add_arg("feature_to_be_removed")
    .run_all()
    .into_iter()
    .for_each(|a| {
        a.failure();
    });
}
