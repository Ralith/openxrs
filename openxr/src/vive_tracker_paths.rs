use crate::Path;

pub struct ViveTrackerPathsHTCX {
    pub persistent: Path,
    pub role: Option<Path>,
}

impl From<&sys::ViveTrackerPathsHTCX> for ViveTrackerPathsHTCX {
    fn from(paths: &sys::ViveTrackerPathsHTCX) -> Self {
        Self {
            persistent: paths.persistent_path,
            role: if paths.role_path.into_raw() == sys::NULL_PATH as u64 {
                None
            } else {
                Some(paths.role_path)
            },
        }
    }
}
