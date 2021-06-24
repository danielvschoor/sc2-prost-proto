#[cfg(feature = "prost-build")]
use std::{env, ffi::OsStr, fs, io::prelude::*, path::Path};

#[cfg(feature = "prost-build")]
fn proto_modules(proto_dir: &Path) -> Vec<String> {
	fs::read_dir(proto_dir)
		.expect("Could not read protobuf directory")
		.filter_map(|entry| {
			let path = entry.ok()?.path();
			if path.is_file() && path.extension() == Some(OsStr::new("proto")) {
				path.file_stem().and_then(|n| n.to_os_string().into_string().ok())
			} else {
				None
			}
		})
		.collect()
}

#[cfg(feature = "prost-build")]
fn main() {
	let in_dir = "./s2client-proto/s2clientprotocol";
	let out_dir = &env::var("OUT_DIR").unwrap();

	// Read list of all input protobuf files
	let input_mods = proto_modules(Path::new(in_dir));
	let input_files: Vec<String> = input_mods
		.iter()
		.map(|s| format!("{}/{}.proto", in_dir, s))
		.collect();
	prost_build::Config::new()
		.compile_protos(&input_files, &["s2client-proto/".to_string()]).unwrap();
	println!("protobufs were generated successfully");

	// Generate the lib.rs source code
	fs::write(
		format!("{}/{}", out_dir, "lib.rs"),
		input_mods
			.iter()
			.map(|s| format!("pub mod {};", s))
			.collect::<Vec<_>>()
			.join("\n"),
	)
		.unwrap();
}

#[cfg(not(feature = "prost-build"))]
fn main() {
	println!("using pre-generated *.rs files in 'src/'");
}

