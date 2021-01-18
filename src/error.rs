#[derive(Debug)]
pub enum CptbError {
    SettingsFileMissing(std::io::Error),
    SettingsFileParserError(serde_json::Error),
    DownloadError(ureq::Error),
}

impl From<std::io::Error> for CptbError {
    fn from(err: std::io::Error) -> CptbError {
        CptbError::SettingsFileMissing(err)
    }
}

impl From<serde_json::Error> for CptbError {
    fn from(err: serde_json::Error) -> CptbError {
        CptbError::SettingsFileParserError(err)
    }
}

impl From<ureq::Error> for CptbError {
    fn from(err: ureq::Error) -> CptbError {
        CptbError::DownloadError(err)
    }
}
