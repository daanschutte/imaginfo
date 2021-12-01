# Imaginfo

> This project is very much a work in progress.

This is a toy project with which I intend to learn Rust.

The idea of this application is that we can specify a directory that contains RAW image files, which contain metadata
therein. Using this metadata we want to display interesting stats and insights about the images as a whole, to guide the
photographer regarding what works and does not work for them, or where they have technical gaps in their workflows.

Unfortunately, of the Rust crates available at this time, and including the one I have gone with for this project,
[exif parser][crates_io_exif] can read Sony's `ARW` format, but cannot parse for example Canon `CR3` or FujiFilm `RAF`
files. After some research it appears to be possible to read other manufacturers' metadata, it is not going to be a
trivial task (see [this GitHub issue][github_issue_parsing_library]).

Be that as it may, for me this is still useful since the main aim is to learn Rust. Secondly, fortunately for me, I have
a Sony camera, so I can read all the images taken with my Sony camera. It also appears that HEIF is also supported and
so hopefully mobile photography can benefit from this application. Lastly, with the intended design of the application
it should be very doable to simply switch crates in the future should the need/solution arise.


[crates_io_exif]: https://docs.rs/kamadak-exif/0.5.4/exif/ "kamadak-exif"
[github_issue_parsing_library]: https://github.com/daanschutte/imaginfo/issues/1 "Re-evaluate parsing library"
