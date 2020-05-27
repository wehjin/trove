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
fn data_folder() -> PathBuf { dirs::cache_dir().unwrap() }
