use assert_cmd::Command;
use std::fs::remove_file;
use tempfile::TempDir;

fn prepare_project(git: bool) -> TempDir {
    let temp_dir = TempDir::new().unwrap();
    let mut cmd = Command::cargo_bin("ucelofka").unwrap();
    let cmd = cmd
        .arg("project")
        .arg("make")
        .arg("--target")
        .arg(temp_dir.path());
    let cmd = if git { cmd.arg("--git") } else { cmd };
    cmd.assert().success();

    remove_file(temp_dir.path().join("invoices").join(".gitkeep")).unwrap();
    temp_dir
}

fn test_cmd(
    cmd: &str,
    subcmd: &str,
    path: &str,
    args: &[&str],
    outputs: &[&str],
) -> (String, String) {
    let assert = Command::cargo_bin("ucelofka")
        .unwrap()
        .arg(cmd)
        .arg("--path")
        .arg(path)
        .arg(subcmd)
        .args(args.iter())
        .assert()
        .success();

    let stdout = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    let stderr = String::from_utf8(assert.get_output().stderr.clone()).unwrap();

    for output in outputs {
        assert!(stdout.contains(output));
    }
    (stdout, stderr)
}

mod project {
    use super::prepare_project;

    #[test]
    fn make() {
        prepare_project(true);
        prepare_project(false);
    }
}

mod account {
    use super::{prepare_project, test_cmd};

    #[test]
    fn list() {
        let project_dir = prepare_project(false);

        test_cmd(
            "account",
            "list",
            project_dir.path().to_str().unwrap(),
            &[],
            &["accounts:", "id: first_account"],
        );
    }

    #[test]
    fn get() {
        let project_dir = prepare_project(false);

        test_cmd(
            "account",
            "get",
            project_dir.path().to_str().unwrap(),
            &["--id", "first_account"],
            &["id: first_account"],
        );
    }
}

mod customer {
    use super::{prepare_project, test_cmd};

    #[test]
    fn list() {
        let project_dir = prepare_project(false);

        test_cmd(
            "customer",
            "list",
            project_dir.path().to_str().unwrap(),
            &[],
            &["customers:", "id: first_customer"],
        );
    }

    #[test]
    fn get() {
        let project_dir = prepare_project(false);

        test_cmd(
            "customer",
            "get",
            project_dir.path().to_str().unwrap(),
            &["--id", "first_customer"],
            &["id: first_customer"],
        );
    }
}

mod entry {
    use super::{prepare_project, test_cmd};

    #[test]
    fn list() {
        let project_dir = prepare_project(false);

        test_cmd(
            "entry",
            "list",
            project_dir.path().to_str().unwrap(),
            &[],
            &["entries:", "id: 001_first_entry"],
        );
    }

    #[test]
    fn get() {
        let project_dir = prepare_project(false);

        test_cmd(
            "entry",
            "get",
            project_dir.path().to_str().unwrap(),
            &["--id", "001_first_entry"],
            &["id: 001_first_entry"],
        );
    }

    #[test]
    fn create() {
        let project_dir = prepare_project(false);

        test_cmd(
            "entry",
            "create",
            project_dir.path().to_str().unwrap(),
            &[
                "--id",
                "002_second_entry",
                "--currency",
                "CZK",
                "--name",
                "hard work 1",
                "--price",
                "99.9",
            ],
            &[],
        );

        test_cmd(
            "entry",
            "list",
            project_dir.path().to_str().unwrap(),
            &[],
            &["entries:", "id: 002_second_entry"],
        );

        let project_dir = prepare_project(true);

        test_cmd(
            "entry",
            "create",
            project_dir.path().to_str().unwrap(),
            &[
                "--id",
                "002_second_entry",
                "--currency",
                "CZK",
                "--name",
                "hard work 1",
                "--price",
                "99.9",
                "--git",
            ],
            &[],
        );

        test_cmd(
            "entry",
            "list",
            project_dir.path().to_str().unwrap(),
            &[],
            &["entries:", "id: 002_second_entry"],
        );
    }
}

mod identity {
    use super::{prepare_project, test_cmd};

    #[test]
    fn get() {
        let project_dir = prepare_project(false);

        test_cmd(
            "identity",
            "get",
            project_dir.path().to_str().unwrap(),
            &["--id", "first_identity"],
            &["id: first_identity"],
        );
    }

    #[test]
    fn list() {
        let project_dir = prepare_project(false);

        test_cmd(
            "identity",
            "list",
            project_dir.path().to_str().unwrap(),
            &[],
            &["identities:", "id: first_identity"],
        );
    }
}

mod invoice {
    use super::{prepare_project, test_cmd};

    fn invoice(path: &str) -> String {
        let (output, _) = test_cmd(
            "invoice",
            "create",
            path,
            &[
                "--account",
                "first_account",
                "--customer",
                "first_customer",
                "--identity",
                "first_identity",
                "--entry",
                "001_first_entry",
            ],
            &[],
        );

        output
            .split_whitespace()
            .map(String::from)
            .collect::<Vec<String>>()
            .last()
            .unwrap()
            .into()
    }

    #[test]
    fn create() {
        let project_dir = prepare_project(false);
        let invoice_id = invoice(project_dir.path().to_str().unwrap());

        test_cmd(
            "invoice",
            "list",
            project_dir.path().to_str().unwrap(),
            &[],
            &["invoices:", &format!("id: {}", invoice_id)],
        );
    }

    #[test]
    fn list() {
        let project_dir = prepare_project(false);

        test_cmd(
            "invoice",
            "list",
            project_dir.path().to_str().unwrap(),
            &[],
            &["invoices:"],
        );
    }

    #[test]
    fn get() {
        let project_dir = prepare_project(false);
        let invoice_id = invoice(project_dir.path().to_str().unwrap());

        test_cmd(
            "invoice",
            "get",
            project_dir.path().to_str().unwrap(),
            &["--id", &invoice_id],
            &[&format!("id: {}", invoice_id)],
        );
    }

    #[test]
    fn render() {
        let project_dir = prepare_project(false);
        let invoice_id = invoice(project_dir.path().to_str().unwrap());

        test_cmd(
            "invoice",
            "render",
            project_dir.path().to_str().unwrap(),
            &["--invoice", &invoice_id, "--template", "default.html"],
            &[],
        );
    }
}
