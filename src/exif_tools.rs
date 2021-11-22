use std::error::Error;
use std::path::Path;

use exif::{Exif, In, Rational, Tag, Value};
use log::{debug, error};

use crate::database::Image;

/// Reads the specified file from disk and attempts to parse all the exif data from the image file.
/// For greater verbosity the `debug_image_info` flag should be set.
pub(crate) fn get_exif_data(
    path: &Path,
    debug_image_info: bool,
) -> Result<(&Path, Exif), Box<dyn Error>> {
    debug!("Reading file {}", path.display());
    let file = std::fs::File::open(path)?;
    let mut bufreader = std::io::BufReader::new(&file);
    let exifreader = exif::Reader::new();
    let exif = exifreader.read_from_container(&mut bufreader)?;
    if debug_image_info {
        debug!("{} has the following fields:", path.display().to_string());
        for f in exif.fields() {
            // TODO remove filters
            if f.tag != Tag::MakerNote && f.ifd_num != In::THUMBNAIL {
                debug!("{}: {}", f.tag, f.display_value().with_unit(&exif));
            }
        }
    }

    Ok((path, exif))
}

/// Attempt to create an `Image` from an `Exif`, useful for persisting relevant image information.
/// Returns a `std::Error` if any of there are any conversion failures.
pub(crate) fn exif_to_image(path: &Path, exif: &Exif) -> Result<Image, Box<dyn Error>> {
    let id = -1;
    let filename = get_filename(path).unwrap().to_string();
    let f_number = get_f_number(exif).unwrap();

    let image = Image {
        id,
        filename,
        f_number,
    };

    Ok(image)
}

/// Returns the value of an `exif::Tag` that is stored as a `Value::Rational` value `In::PRIMARY`, if present.
fn get_tag_rational(tag: Tag, e: &Exif) -> Option<&Rational> {
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

fn _get_field_as_str(tag: Tag, exif: &Exif) -> Option<String> {
    match exif.get_field(tag, In::PRIMARY) {
        Some(field) => Option::Some(field.display_value().with_unit(exif).to_string()),
        None => {
            error!("{} is missing", tag);
            Option::None
        }
    }
}

/// Returns the last part of the `OsString` defined by a `PathBuf` as a `String` value.
pub(crate) fn get_filename(path: &Path) -> Result<&str, Box<dyn Error>> {
    let f = path.to_str().unwrap();

    Ok(f)
}

/// Returns the `Tag::FNumber` from an `&Exif` value, if present.
pub(crate) fn get_f_number(exif: &Exif) -> Option<f64> {
    get_tag_rational(Tag::FNumber, exif).map(|r| r.to_f64())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exif_to_image() {
        todo!()
    }

    #[test]
    fn test_get_filename() {
        todo!()
    }

    #[test]
    fn test_get_f_number() {
        todo!()
    }

    #[test]
    fn test_get_tag_rational() {
        todo!()
    }

    #[test]
    fn test_get_field_as_string() {
        todo!()
    }
}