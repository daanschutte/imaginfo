use std::error::Error;
use std::path::PathBuf;

use exif::{Exif, In, Rational, Tag, Value};
use log::{debug, error, info};

pub(crate) fn get_exif_data(path: &PathBuf) -> Result<Exif, Box<dyn Error>> {
    let file = std::fs::File::open(path)?;
    let mut bufreader = std::io::BufReader::new(&file);
    let exifreader = exif::Reader::new();
    let exif = exifreader.read_from_container(&mut bufreader)?;
    for f in exif.fields() {
        if f.tag != Tag::MakerNote && f.ifd_num != In::THUMBNAIL {
            debug!("{} {}", f.tag, f.display_value().with_unit(&exif));
        }
    }

    Ok(exif)
}

pub(crate) fn get_tag_rational(tag: Tag, e: &Exif) -> Option<&Rational> {
    match e.get_field(tag, In::PRIMARY) {
        Some(field) => match field.value {
            Value::Rational(ref v) if !v.is_empty() => v.first(),
            _ => {
                error!("{} is broken", tag);
                Option::None
            }
        },

        None => {
            debug!("No data found for {}", tag);
            Option::None
        }
    }
}

pub(crate) fn get_field_as_str(tag: Tag, e: &Exif) -> Option<String> {
    match e.get_field(tag, In::PRIMARY) {
        Some(make) => match make.value.display_as(tag) {
            val => {
                info!("{}: {}", tag, val);
                Option::Some(val.to_string())
            }
        },
        None => {
            error!("{} is missing", tag);
            Option::None
        }
    }
}
