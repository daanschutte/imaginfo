use std::path::{Path, PathBuf};

use ignore::types::{Types, TypesBuilder};
use ignore::WalkBuilder;
use log::debug;

pub(crate) fn find_files_recurse(
    path: &Path,
    debug: bool,
    follow_links: bool,
    hidden: bool,
    max_depth: Option<usize>,
) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    debug!(
        "Searching from {} with a max_depth of {}",
        &path.display(),
        max_depth.unwrap_or(usize::MAX)
    );

    let types = get_types().unwrap_or(TypesBuilder::new().add_defaults().build()?);

    let paths = WalkBuilder::new(path)
        .hidden(hidden)
        .max_depth(max_depth)
        .follow_links(follow_links)
        .types(types)
        .build()
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path().to_path_buf())
        .collect::<Vec<PathBuf>>();

    if debug {
        let _ = &paths
            .iter()
            .for_each(|p| debug!("Found {}", p.display().to_string()));
    }

    Ok(paths)
}

fn get_types() -> Result<Types, ignore::Error> {
    let mut builder = TypesBuilder::new();
    builder.add("RWZ", ".RWZ");
    builder.add("RW2", ".RW2");
    builder.add("CR2", ".CR2");
    builder.add("DNG", ".DNG");
    builder.add("ERF", ".ERF");
    builder.add("NRW", ".NRW");
    builder.add("RAF", ".RAF");
    builder.add("ARW", ".ARW");
    builder.add("NEF", ".NEF");
    builder.add("K25", ".K25");
    builder.add("DNG", ".DNG");
    builder.add("SRF", ".SRF");
    builder.add("EIP", ".EIP");
    builder.add("DCR", ".DCR");
    builder.add("RAW", ".RAW");
    builder.add("CRW", ".CRW");
    builder.add("3FR", ".3FR");
    builder.add("BAY", ".BAY");
    builder.add("MEF", ".MEF");
    builder.add("CS1", ".CS1");
    builder.add("KDC", ".KDC");
    builder.add("ORF", ".ORF");
    builder.add("ARI", ".ARI");
    builder.add("SR2", ".SR2");
    builder.add("MOS", ".MOS");
    builder.add("MFW", ".MFW");
    builder.add("FFF", ".FFF");
    builder.add("CR3", ".CR3");
    builder.add("SRW", ".SRW");
    builder.add("J6I", ".J6I");
    builder.add("X3F", ".X3F");
    builder.add("KC2", ".KC2");
    builder.add("RWL", ".RWL");
    builder.add("MRW", ".MRW");
    builder.add("PEF", ".PEF");
    builder.add("IIQ", ".IIQ");
    builder.add("CXI", ".CXI");
    builder.add("MDC", ".MD;C");

    builder.build()
}
