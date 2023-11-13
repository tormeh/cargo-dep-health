use cargo::{ops, core::{Package, Workspace}};

fn main() {
    let ws = Workspace::new()?;
    let package_set = ops::resolve_ws(ws)?.0;
    let package_statuses = package_set.packages().map(status_of_package).collect();
}

fn status_of_package(pkg: &Package) -> String {

}
