pub enum VisualStudioToolsetVersion {
	V140,
	V141,
	V142,
}

pub struct LibraryDetails {
	pub name: &'static str,
	pub file_name: &'static str,
	pub build_type: BuildType,
}

pub struct CMakeDetails {
	pub cmake_local_path: Option<Vec<&'static str>>,
	pub extra_configure_args: Vec<String>,
}

pub struct CustomBuildStep {
	pub name: &'static str,
	pub command: &'static str,
	pub args: Vec<String>,
	pub in_current_dir: bool,
}

pub struct CustomBuildDetails {
	pub steps: Vec<CustomBuildStep>,
}

pub enum BuildType {
	CMake(CMakeDetails),
	Custom(CustomBuildDetails),
}
