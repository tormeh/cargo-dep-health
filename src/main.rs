use core::fmt;

use cargo_metadata::{MetadataCommand, Package};

struct PackageHealth {
    name: String,
    maintainers_last_month: i32,
}

impl PackageHealth {
    pub fn score(&self) -> f32 {
        self.maintainers_last_month as f32
    }
}

impl fmt::Display for PackageHealth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n  maintainers: {}", self.name, self.maintainers_last_month)
    }
}

fn main() {
    let metadata = MetadataCommand::new().exec().unwrap();
    let mut healths = metadata.packages
        .iter()
        .map(health_of_package)
        .collect::<Vec<_>>();
    healths.sort_by(|pa, pb| pa.score().partial_cmp(&pb.score()).unwrap());
    healths
        .into_iter()
        .for_each(|ph| println!("{}", ph));
}

fn health_of_package(pkg: &Package) -> PackageHealth {
    println!("{:?}", pkg.authors);
    PackageHealth { name: pkg.name.to_owned(), maintainers_last_month: 2 }
}