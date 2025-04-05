#[test]
fn f0() {
    given!("an empty repository");
    when!("adding task {0}");
    then!("the repository contains task {0}");
}

#[test]
fn f1() {
    given!("an empty repository");
    when!("removing task {0}");
    then!("the repository is empty");
}

#[test]
fn f2() {
    given!("a repository containing tasks {0} to {1024}");
    when!("removing task {444}");
    then!("the repository contains task {0} to {1024} except {444}");
}
