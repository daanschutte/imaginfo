# Imaginfo

> This project is very much a work in progress.

This is a toy project with which I intend to learn Rust.

The idea of this application is that we can specify a directory that contains RAW image files, which contain metadata
therein. Using this metadata we want to display interesting stats and insights about the images as a whole, to guide the
photographer regarding what works and does not work for them, or where they have technical gaps in their workflows.

Unfortunately, from my brief research, it appears that RAW files formats of camera manufacturers are mostly proprietary.
From a quick test, the [exif parser][crates_io_exif] that I am using can read Sony's `ARW` format, but cannot parse
Canon `CR3` or FujiFilm `RAF` files. While this may be limited to this library only, a cursory look around seems to
indicate that this is indeed going to be a problem.

Be that as it may, for me this is still useful since the main aim is to learn Rust. Secondly, fortunately for me, I have
a Sony camera, so I can read all the images taken with my Sony camera. It also appears that HEIF is also supported and
so hopefully mobile photography can benefit from this application.


[crates_io_exif]: https://docs.rs/kamadak-exif/0.5.4/exif/ "kamadak-exif"
