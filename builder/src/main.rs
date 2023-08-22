extern crate getopts;
extern crate rand;

mod types;

use getopts::Options;
use rand::Rng;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::time::Instant;

use types::{
	BuildType,
	CMakeDetails,
	CustomBuildDetails,
	CustomBuildStep,
	LibraryDetails,
	VisualStudioToolsetVersion
};

struct BuildOptions {
	toolset_version: VisualStudioToolsetVersion,
}

struct MyError {
	message: String,
}

impl MyError {
	fn new(message: &str) -> Self {
		MyError {
			message: String::from(message),
		}
	}
}

impl From<&str> for MyError {
	fn from(string: &str) -> Self {
		MyError::new(string)
	}
}

/// Return an ordered list of all libraries to be built, with build details
fn get_build_list(artifact_dir: &Path, toolset_version: &VisualStudioToolsetVersion) -> Vec<LibraryDetails> {
	let artifact_dir_string = artifact_dir.to_string_lossy().to_string().replace("\\", "/");
	vec![
		LibraryDetails {
			name: "zlib-1.2.13",
			file_name: "zlib-1.2.13.tar.gz",
			build_type: BuildType::CMake(CMakeDetails {
				cmake_local_path: None,
				extra_configure_args: vec![],
			}),
		},
		LibraryDetails {
			name: "libjpeg-turbo-2.1.5.1",
			file_name: "libjpeg-turbo-2.1.5.1.tar.gz",
			build_type: BuildType::CMake(CMakeDetails {
				cmake_local_path: None,
				extra_configure_args: vec![
					String::from("-DWITH_CRT_DLL=True"),
					String::from("-DWITH_TURBOJPEG=False"),
					String::from("-DENABLE_SHARED=False"),
				],
			}),
		},
		LibraryDetails {
			name: "libpng-1.6.39",
			file_name: "libpng-1.6.39.tar.gz",
			build_type: BuildType::CMake(CMakeDetails {
				cmake_local_path: None,
				extra_configure_args: vec![
					String::from("-DPNG_SHARED=False"),
					format!("-DZLIB_ROOT={}/zlib-1.2.13/", artifact_dir_string),
				],
			}),
		},
		LibraryDetails {
			name: "libtiff-4.5.0",
			file_name: "tiff-4.5.0.tar.gz",
			build_type: BuildType::CMake(CMakeDetails {
				cmake_local_path: None,
				extra_configure_args: vec![
					String::from("-DBUILD_SHARED_LIBS=False"),
					format!("-DZLIB_ROOT={}/zlib-1.2.13/", artifact_dir_string),
				],
			}),
		},
		LibraryDetails {
			name: "openexr-2.5.8",
			file_name: "openexr-2.5.8.tar.gz",
			build_type: BuildType::CMake(CMakeDetails {
				cmake_local_path: None,
				extra_configure_args: vec![
					String::from("-DBUILD_SHARED_LIBS=False"),
					format!("-DZLIB_ROOT={}/zlib-1.2.13/", artifact_dir_string),
				],
			}),
		},
		LibraryDetails {
			name: "freeglut-3.2.2",
			file_name: "freeglut-3.2.2.tar.gz",
			build_type: BuildType::CMake(CMakeDetails {
				cmake_local_path: None,
				extra_configure_args: vec![
					String::from("-DFREEGLUT_BUILD_DEMOS=False"),
					String::from("-DFREEGLUT_BUILD_SHARED_LIBS=False"),
				],
			}),
		},
		LibraryDetails {
			name: "glew-2.1.0",
			file_name: "glew-2.1.0.tgz",
			build_type: BuildType::CMake(CMakeDetails {
				cmake_local_path: Some(vec!["build","cmake"]),
				extra_configure_args: vec![],
			}),
		},
		LibraryDetails {
			name: "glfw-3.3.8",
			file_name: "glfw-3.3.8.zip",
			build_type: BuildType::CMake(CMakeDetails {
				cmake_local_path: None,
				extra_configure_args: vec![
				],
			}),
		},
		LibraryDetails {
			name: "boost-1.80.0",
			file_name: "boost_1_80_0.tar.gz",
			build_type: BuildType::Custom(CustomBuildDetails {
				steps: vec![
					CustomBuildStep {
						name: "bootstrap",
						command: "cmd",
						args: vec![
							String::from("/c"),
							String::from("bootstrap.bat"),
						],
						in_current_dir: false,
					},
					CustomBuildStep {
						name: "b2_build",
						command: "b2.exe",
						args: vec![
							String::from("install"),
							format!("--prefix={}/boost-1.80.0/", artifact_dir_string),
							String::from(
								match toolset_version {
									VisualStudioToolsetVersion::V140 => "toolset=msvc-14.0",
									VisualStudioToolsetVersion::V141 => "toolset=msvc-14.1",
									VisualStudioToolsetVersion::V142 => "toolset=msvc-14.2",
								}
							),
							String::from("variant=release"),
							String::from("link=static"),
							String::from("threading=multi"),
							String::from("address-model=64"),
							String::from("runtime-link=shared"),
							String::from("--with-filesystem"),
							String::from("--with-regex"),
							String::from("--with-system"),
							String::from("--with-thread"),
							String::from("--with-date_time"),
							String::from("--with-chrono"),
							String::from("--with-atomic"),
						],
						in_current_dir: true,
					},
				],
			}),
		},
		LibraryDetails {
			name: "oneTBB-2021.3.0",
			file_name: "oneTBB-2021.3.0-custom.tar.gz",
			build_type: BuildType::CMake(CMakeDetails {
				cmake_local_path: None,
				extra_configure_args: vec![
					String::from("-DBUILD_SHARED_LIBS=False"),
					String::from("-DTBB_TEST=False"),
					String::from("-DTBB_STRICT=False"),
					String::from("-DCMAKE_CXX_FLAGS=\"/D__TBB_SOURCE_DIRECTLY_INCLUDED\""),
				],
			}),
		},
		LibraryDetails {
			name: "imgui-1.88",
			file_name: "imgui-1.88-custom.tar.gz",
			build_type: BuildType::CMake(CMakeDetails {
				cmake_local_path: None,
				extra_configure_args: vec![
					format!("-DGLFW_INCLUDE_DIR={}/glfw-3.3.8/include", artifact_dir_string),
				],
			}),
		},
		LibraryDetails {
			name: "shader_editor",
			file_name: "shader_editor-21_04_04.tar.gz",
			build_type: BuildType::CMake(CMakeDetails {
				cmake_local_path: None,
				extra_configure_args: vec![
					format!("-DIMGUI_INCLUDE_DIR={}/imgui-1.88/include", artifact_dir_string),
					format!("-DGLFW_INCLUDE_DIR={}/glfw-3.3.8/include", artifact_dir_string),
					format!("-DBOOST_ROOT={}/boost-1.80.0", artifact_dir_string),
				],
			}),
		},
		LibraryDetails {
			name: "oiio-2.2.21.0",
			file_name: "oiio-2.2.21.0.tar.gz",
			build_type: BuildType::CMake(CMakeDetails {
				cmake_local_path: None,
				extra_configure_args: vec![
					format!("-DCMAKE_PREFIX_PATH={0}/openexr-2.5.8;{0}/libtiff-4.5.0;{0}/libjpeg-turbo-2.1.5.1", artifact_dir_string),
					format!("-DBOOST_ROOT={}/boost-1.80.0/", artifact_dir_string),
					format!("-DZLIB_ROOT={}/zlib-1.2.13/", artifact_dir_string),
					String::from("-DBUILD_TESTING=False"),
					String::from("-DBUILD_SHARED_LIBS=False"),
					String::from("-DLINKSTATIC=True"),
					String::from("-DUSE_PYTHON=False"),
					String::from("-DUSE_FREETYPE=False"),
					String::from("-DUSE_GIF=False"),
					String::from("-DOIIO_BUILD_TESTS=False"),
					String::from("-DOIIO_BUILD_TOOLS=False"),
				],
			}),
		},
	]
}

/// Returns the path of the only subdirectory within the current working directory
fn get_only_dir_path() -> Result<PathBuf, &'static str> {
	let current_dir = std::env::current_dir().map_err(|_| "Failed to get current directory")?;
	let dir_list = std::fs::read_dir(&current_dir).map_err(|_| "Failed to read contents of current directory")?;
	
	for this_entry in dir_list {
		match this_entry {
			Ok(entry) => {
				let path_buf = entry.path();
				if path_buf.is_dir() {
					return Ok(path_buf);
				}
			},
			Err(_) => ()
		}
	}
	Err("Directory not found")
}

/// Build and install target library
fn build_and_install(library_details: &LibraryDetails, top_dir: &Path, artifact_dir: &Path, options: &BuildOptions) -> Result<(), MyError> {
	// Generate a random unique tag for this build
	let build_tag: String =  rand::thread_rng()
		.sample_iter(&rand::distributions::Alphanumeric)
		.take(5)
		.collect();
	let build_dir_name = format!("_{}", build_tag);

	let archive_dir = top_dir.join("archive");
	let build_dir = top_dir.join(build_dir_name);
	let tmp_archive = build_dir.join("archive.file");

	let install_dir = artifact_dir.join(&library_details.name);

	println!("");
	println!("Beginning build for {}", &library_details.name);

	if install_dir.exists() {
		println!("Already done, skipping");
		return Ok(());
	}

	println!("Build tag: {}", build_tag);

	// Copy source tarball over
	std::fs::create_dir(&build_dir).map_err(|_| "Failed to create top-level build directory")?;
	{
		let original_tgz = archive_dir.join(&library_details.file_name);
		std::fs::copy(&original_tgz, &tmp_archive).map_err(|_| "Failed to copy source tarball to build directory")?;
	}

	println!("Extracting source...");

	// Extract tarball
	std::env::set_current_dir(&build_dir).map_err(|_| "Failed to enter build directory")?;
	std::process::Command::new("tar") .arg("-xf").arg("./archive.file").output()
		.map_err(|_| "Failed to extract source tarball")?;

	// Determine source directory name
	let top_src_dir = get_only_dir_path().map_err(|_| "Failed to find extracted source directory")?;

	match &library_details.build_type {
		BuildType::CMake(cmake_details) => {
			{
				// Make and enter inner build directory
				{
					let inner_build_dir = build_dir.join("build");
					std::fs::create_dir(&inner_build_dir).map_err(|_| "Error creating inner build dir")?;
					std::env::set_current_dir(&inner_build_dir).map_err(|_| "Failed to enter build dir")?;
				}

				// Do a cmake configure
				println!("Running cmake configure...");
				let cmake_src_dir = match &cmake_details.cmake_local_path {
					Some(dirs) => {
						let mut path = top_src_dir;
						for dir in dirs {
							path = path.join(dir);
						}
						path
					},
					None => top_src_dir,
				};
				let arg_cmake_src_dir = format!("{}", cmake_src_dir.to_string_lossy());
				let install_prefix = install_dir.to_string_lossy().to_string();
				let arg_cmake_install_prefix = format!("-DCMAKE_INSTALL_PREFIX={}", &install_prefix);
				let cmake_output = std::process::Command::new("cmake")
					.arg(&arg_cmake_src_dir)
					.arg("-G").arg("Visual Studio 16 2019")
					.arg("-A").arg("x64")
					.arg("-T").arg(
						match &options.toolset_version {
							VisualStudioToolsetVersion::V140 => "v140",
							VisualStudioToolsetVersion::V141 => "v141",
							VisualStudioToolsetVersion::V142 => "v142",
						}
					)
					.arg(arg_cmake_install_prefix)
					.args(&cmake_details.extra_configure_args)
					.output()
					.map_err(|_| "Failed to run cmake configure")?;

				std::fs::write("stdout_configure.txt", &cmake_output.stdout).map_err(|_| "Failed to write configure stdout")?;
				std::fs::write("stderr_configure.txt", &cmake_output.stderr).map_err(|_| "Failed to write configure stderr")?;
				if cmake_output.status.success() == false {
					return Err(MyError::new("Cmake configure reported an error"));
				}
			}

			{
				// Do a cmake build
				println!("Running cmake build...");
				let cmake_output = std::process::Command::new("cmake")
					.arg("--build").arg(".")
					.arg("--target").arg("INSTALL")
					.arg("--config").arg("Release")
					.arg("--parallel").arg("8")
					.output()
					.map_err(|_| "Failed to run cmake build")?;
				std::fs::write("stdout_build.txt", &cmake_output.stdout).map_err(|_| "Failed to write build stdout")?;
				std::fs::write("stderr_build.txt", &cmake_output.stderr).map_err(|_| "Failed to write build stderr")?;
				if cmake_output.status.success() == false {
					return Err(MyError::new("Cmake build reported an error"));
				}
			}
		},
		BuildType::Custom(custom_details) => {
			// Enter main source dir to do the build
			//std::env::set_current_dir(&top_src_dir).map_err(|_| "Failed to enter source dir")?;
			//println!("Entering dir: {}", top_src_dir.into_os_string().into_string().unwrap());
			for build_step in &custom_details.steps {
				println!("Doing custom build step: {}", build_step.name);

				std::env::set_current_dir(&top_src_dir).map_err(|_| "Failed to enter source dir")?;
				let command = if build_step.in_current_dir { std::env::current_dir().unwrap().join(build_step.command).into_os_string() } else { OsString::from(build_step.command) };
				let step_output = std::process::Command::new(command)
					.args(&build_step.args)
					.output()
					.map_err(|_| "Failed to run build step")?;

				let stdout_filename = format!("stdout_{}.txt", build_step.name);
				let stderr_filename = format!("stderr_{}.txt", build_step.name);
				std::fs::write(stdout_filename, &step_output.stdout).map_err(|_| "Failed to write build step stdout")?;
				std::fs::write(stderr_filename, &step_output.stderr).map_err(|_| "Failed to write build step stderr")?;
				if step_output.status.success() == false {
					return Err(MyError::new("Build step reported an error"));
				}
			}
		},
	}

	// Return to original directory
	std::env::set_current_dir(&top_dir).map_err(|_| "Failed to enter original dir")?;
	
	println!("Cleaning up...");

	// Sleep before deleting build dir
	std::thread::sleep(std::time::Duration::from_millis(100));
	let remove_result = std::fs::remove_dir_all(&build_dir);

	match remove_result {
		Ok(_) => (),
		Err(_) => {
			match std::env::current_dir() {
				Ok(dir) => {
					match dir.to_str() {
						Some(the_string) => {
							println!("{}", the_string);
						},
						None => {
							println!("Failed to get current directory as a string");
						},
					}
				},
				Err(_) => {
					println!("Failed to get current directory");
				},
			}
			println!("Failed to remove build directory, sleeping for 60 seconds and trying again...");
			std::thread::sleep(std::time::Duration::from_millis(60000));
			std::fs::remove_dir_all(&build_dir).map_err(|_| "Failed to remove completed build directory")?;
		}
	};

	Ok(())
}

fn print_usage(options: &Options) {
	let brief = "Usage: builder [options]";
	println!("{}", options.usage(brief));
}

fn main() {
	println!("Windows Dependency Helper v0.1");

	let options = {
		let args: Vec<String> = std::env::args().collect();
		let mut options = Options::new();
		options.optopt("t", "toolset", "visual studio toolset version (v140, v141, v142)", "VERSION");
		options.optflag("h", "help", "display this help");

		let matches = match options.parse(&args[1..]) {
			Ok(matches) => matches,
			Err(_) => {
				println!("Invalid arguments specified");
				print_usage(&options);
				return;
			}
		};

		if matches.opt_present("h") {
			print_usage(&options);
			return;
		}

		match matches.opt_str("t") {
			Some(string) => {
				match string.as_ref() {
					"2015" | "140" | "v140" => {
						BuildOptions {
							toolset_version: VisualStudioToolsetVersion::V140,
						}
					},
				   "2017" | "141" | "v141" => {
					BuildOptions {
							toolset_version: VisualStudioToolsetVersion::V141,
						}
					},
					"2019" | "142" | "v142" => {
					 BuildOptions {
							 toolset_version: VisualStudioToolsetVersion::V142,
						 }
					 },
					_ => {
						println!("Error: invalid toolset specified");
						return;
					}
				}
			}
			None => {
				println!("Error: toolset not specified");
				print_usage(&options);
				return;
			}
		}
	};

	{
		let toolset_version_string = match &options.toolset_version {
			VisualStudioToolsetVersion::V140 => "2015 (14.0)",
			VisualStudioToolsetVersion::V141 => "2017 (14.1)",
			VisualStudioToolsetVersion::V142 => "2019 (14.2)",
		};
		println!("Building with Visual C++ {}", toolset_version_string);
	}
	
	let top_dir = match std::env::current_dir() {
		Ok(dir) => dir,
		Err(e) => {
			println!("Failed to get current directory, aborting: {}", e);
			return;
		}
	};

	let artifact_dir = match options.toolset_version {
		VisualStudioToolsetVersion::V140 => top_dir.join("v140r"),
		VisualStudioToolsetVersion::V141 => top_dir.join("v141r"),
		VisualStudioToolsetVersion::V142 => top_dir.join("v142r"),
	};

	let libraries = get_build_list(&artifact_dir, &options.toolset_version);
	
	for this_library in libraries {
		let build_begin_time = Instant::now();
		match build_and_install(&this_library, &top_dir, &artifact_dir, &options) {
			Ok(_) => {
				println!("Build complete in {} seconds", build_begin_time.elapsed().as_secs());
			},
			Err(e) => {
				println!("Build failed: {}", e.message);
				break;
			}
		}
	}
}
