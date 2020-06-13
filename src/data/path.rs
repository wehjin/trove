use std::io;
use std::path::PathBuf;

pub fn echo(folder_name: &str) -> io::Result<PathBuf> {
	let mut path = data_folder();
	path.push(folder_name);
	std::fs::create_dir_all(&path)?;
	Ok(path)
}

#[cfg(not(test))]
fn data_folder() -> PathBuf { dirs::home_dir().unwrap() }

#[cfg(test)]
fn data_folder() -> PathBuf {
	let mut testdir = std::env::temp_dir();
	testdir.push(&format!("chad-test-{}", rand::random::<u32>()));
	testdir
}
