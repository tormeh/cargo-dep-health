use cargo_metadata::MetadataCommand;

struct PackageHealth {
    maintainers_last_month: i32,
}

fn main() {
    let metadata = MetadataCommand::new().exec().unwrap();
    metadata.packages;
}
