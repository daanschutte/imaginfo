use std::path::{Path, PathBuf};

use ignore::types::{Types, TypesBuilder};
use ignore::WalkBuilder;
use log::{debug, error};

const FILE_TYPES: [(&str, &str); 37] = [
    ("3FR", "*.3FR"),
    ("ARI", "*.ARI"),
    ("ARW", "*.ARW"),
    ("BAY", "*.BAY"),
    ("CR2", "*.CR2"),
    ("CR3", "*.CR3"),
    ("CRW", "*.CRW"),
    ("CS1", "*.CS1"),
    ("CXI", "*.CXI"),
    ("DCR", "*.DCR"),
    ("DNG", "*.DNG"),
    ("EIP", "*.EIP"),
    ("ERF", "*.ERF"),
    ("FFF", "*.FFF"),
    ("IIQ", "*.IIQ"),
    ("J6I", "*.J6I"),
    ("K25", "*.K25"),
    ("KC2", "*.KC2"),
    ("KDC", "*.KDC"),
    ("MDC", "*.MDC"),
    ("MEF", "*.MEF"),
    ("MFW", "*.MFW"),
    ("MOS", "*.MOS"),
    ("MRW", "*.MRW"),
    ("NEF", "*.NEF"),
    ("NRW", "*.NRW"),
    ("ORF", "*.ORF"),
    ("PEF", "*.PEF"),
    ("RAF", "*.RAF"),
    ("RAW", "*.RAW"),
    ("RW2", "*.RW2"),
    ("RWL", "*.RWL"),
    ("RWZ", "*.RWZ"),
    ("SR2", "*.SR2"),
    ("SRF", "*.SRF"),
    ("SRW", "*.SRW"),
    ("X3F", "*.X3F"),
];

pub(crate) fn find_files(
    path: &Path,
    debug: bool,
    follow_links: bool,
    max_depth: Option<usize>,
) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    debug!(
        "Searching from {} with a max_depth of {}",
        &path.display(),
        max_depth.unwrap_or(usize::MAX)
    );

    let types = get_types().unwrap_or(TypesBuilder::new().add_defaults().build()?);

    let paths = WalkBuilder::new(path)
        .max_depth(max_depth)
        .follow_links(follow_links)
        .types(types)
        .build()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().unwrap().is_file())
        .map(|e| e.path().to_path_buf())
        .collect::<Vec<PathBuf>>();

    if debug {
        debug!("Discovered {} image files", paths.len())
    }

    Ok(paths)
}

fn get_types() -> Result<Types, ignore::Error> {
    let mut builder = TypesBuilder::new();

    FILE_TYPES.map(|t| add_def(&mut builder, t.0, t.1));

    builder.select("all");
    debug!(
        "The following {} file types will be searched: {:?}",
        &builder.definitions().len(),
        &builder.definitions()
    );

    builder.build()
}

fn add_def(builder: &mut TypesBuilder, name: &str, glob: &str) {
    let builder = match builder.add(name, glob) {
        Ok(b) => b,
        Err(e) => error!("Could not add {}:{} to file types: {}", name, glob, e),
    };
    builder
}
