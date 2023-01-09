#[test]
fn reword_protected_fails() {
    let root = snapbox::path::PathFixture::mutable_temp().unwrap();
    let root_path = root.path().unwrap();
    let plan = git_fixture::TodoList {
        init: true,
        sleep: None,
        author: None,
        commands: vec![
            git_fixture::Command::Tree(git_fixture::Tree {
                files: [("a", "a")]
                    .into_iter()
                    .map(|(p, c)| (p.into(), c.into()))
                    .collect::<std::collections::HashMap<_, _>>(),
                message: Some("A".to_owned()),
                author: None,
            }),
            git_fixture::Command::Branch("main".into()),
        ],
    };
    plan.run(root_path).unwrap();

    snapbox::cmd::Command::new(snapbox::cmd::cargo_bin!("git-stack"))
        .arg("amend")
        .arg("--message=hahahaha")
        .current_dir(root_path)
        .assert()
        .failure()
        .stdout_eq(
            "\
",
        )
        .stderr_eq(
            "\
cannot amend protected commits
",
        );
}

#[test]
fn reword() {
    let root = snapbox::path::PathFixture::mutable_temp().unwrap();
    let root_path = root.path().unwrap();
    let plan = git_fixture::TodoList {
        init: true,
        sleep: None,
        author: None,
        commands: vec![
            git_fixture::Command::Tree(git_fixture::Tree {
                files: [("a", "a")]
                    .into_iter()
                    .map(|(p, c)| (p.into(), c.into()))
                    .collect::<std::collections::HashMap<_, _>>(),
                message: Some("A".to_owned()),
                author: None,
            }),
            git_fixture::Command::Branch("main".into()),
            git_fixture::Command::Tree(git_fixture::Tree {
                files: [("a", "a"), ("b", "b")]
                    .into_iter()
                    .map(|(p, c)| (p.into(), c.into()))
                    .collect::<std::collections::HashMap<_, _>>(),
                message: Some("B".to_owned()),
                author: None,
            }),
            git_fixture::Command::Tree(git_fixture::Tree {
                files: [("a", "a"), ("b", "b"), ("c", "c")]
                    .into_iter()
                    .map(|(p, c)| (p.into(), c.into()))
                    .collect::<std::collections::HashMap<_, _>>(),
                message: Some("C".to_owned()),
                author: None,
            }),
            git_fixture::Command::Branch("target".into()),
        ],
    };
    plan.run(root_path).unwrap();

    snapbox::cmd::Command::new(snapbox::cmd::cargo_bin!("git-stack"))
        .arg("amend")
        .arg("--message=new C")
        .current_dir(root_path)
        .assert()
        .success()
        .stdout_eq(
            "\
",
        )
        .stderr_matches(
            "\
Amended to [..]: C
note: to undo, run `git branch-stash pop git-stack`
",
        );

    root.close().unwrap();
}

#[test]
fn reword_rebases() {
    let root = snapbox::path::PathFixture::mutable_temp().unwrap();
    let root_path = root.path().unwrap();
    let plan = git_fixture::TodoList {
        init: true,
        sleep: None,
        author: None,
        commands: vec![
            git_fixture::Command::Tree(git_fixture::Tree {
                files: [("a", "a")]
                    .into_iter()
                    .map(|(p, c)| (p.into(), c.into()))
                    .collect::<std::collections::HashMap<_, _>>(),
                message: Some("A".to_owned()),
                author: None,
            }),
            git_fixture::Command::Branch("main".into()),
            git_fixture::Command::Tree(git_fixture::Tree {
                files: [("a", "a"), ("b", "b")]
                    .into_iter()
                    .map(|(p, c)| (p.into(), c.into()))
                    .collect::<std::collections::HashMap<_, _>>(),
                message: Some("B".to_owned()),
                author: None,
            }),
            git_fixture::Command::Branch("target".into()),
            git_fixture::Command::Tree(git_fixture::Tree {
                files: [("a", "a"), ("b", "b"), ("c", "c")]
                    .into_iter()
                    .map(|(p, c)| (p.into(), c.into()))
                    .collect::<std::collections::HashMap<_, _>>(),
                message: Some("C".to_owned()),
                author: None,
            }),
            git_fixture::Command::Branch("local".into()),
        ],
    };
    plan.run(root_path).unwrap();

    snapbox::cmd::Command::new(snapbox::cmd::cargo_bin!("git-stack"))
        .arg("amend")
        .arg("--message=new B")
        .current_dir(root_path)
        .assert()
        .success()
        .stdout_eq(
            "\
",
        )
        .stderr_matches(
            "\
Amended to [..]: C
note: to undo, run `git branch-stash pop git-stack`
",
        );

    root.close().unwrap();
}
