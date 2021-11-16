use std::path::{Path, PathBuf};

use ignore::types::{Types, TypesBuilder};
use ignore::WalkBuilder;
use log::{debug, error};

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
    // TODO add tests for the added features

    if debug {
        let _ = &paths
            .iter()
            .for_each(|p| debug!("Found {}", p.display().to_string()));
    }

    Ok(paths)
}

fn get_types() -> Result<Types, ignore::Error> {
    let mut builder = TypesBuilder::new();

    add_def(&mut builder, "RWZ", "*.RWZ");
    add_def(&mut builder, "RW2", "*.RW2");
    add_def(&mut builder, "CR2", "*.CR2");
    add_def(&mut builder, "DNG", "*.DNG");
    add_def(&mut builder, "ERF", "*.ERF");
    add_def(&mut builder, "NRW", "*.NRW");
    add_def(&mut builder, "RAF", "*.RAF");
    add_def(&mut builder, "ARW", "*.ARW");
    add_def(&mut builder, "NEF", "*.NEF");
    add_def(&mut builder, "K25", "*.K25");
    add_def(&mut builder, "DNG", "*.DNG");
    add_def(&mut builder, "SRF", "*.SRF");
    add_def(&mut builder, "EIP", "*.EIP");
    add_def(&mut builder, "DCR", "*.DCR");
    add_def(&mut builder, "RAW", "*.RAW");
    add_def(&mut builder, "CRW", "*.CRW");
    add_def(&mut builder, "3FR", "*.3FR");
    add_def(&mut builder, "BAY", "*.BAY");
    add_def(&mut builder, "MEF", "*.MEF");
    add_def(&mut builder, "CS1", "*.CS1");
    add_def(&mut builder, "KDC", "*.KDC");
    add_def(&mut builder, "ORF", "*.ORF");
    add_def(&mut builder, "ARI", "*.ARI");
    add_def(&mut builder, "SR2", "*.SR2");
    add_def(&mut builder, "MOS", "*.MOS");
    add_def(&mut builder, "MFW", "*.MFW");
    add_def(&mut builder, "FFF", "*.FFF");
    add_def(&mut builder, "CR3", "*.CR3");
    add_def(&mut builder, "SRW", "*.SRW");
    add_def(&mut builder, "J6I", "*.J6I");
    add_def(&mut builder, "X3F", "*.X3F");
    add_def(&mut builder, "KC2", "*.KC2");
    add_def(&mut builder, "RWL", "*.RWL");
    add_def(&mut builder, "MRW", "*.MRW");
    add_def(&mut builder, "PEF", "*.PEF");
    add_def(&mut builder, "IIQ", "*.IIQ");
    add_def(&mut builder, "CXI", "*.CXI");
    add_def(&mut builder, "MDC", "*.MDC");

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
