pub use suppaftp::{
  list::File,
  types::{FileType, FormatControl},
  FtpError, FtpResult, FtpStream, ImplFtpStream,
};

use crate::ClientLoginInfo;

/// FTP
pub struct Ftp {
  pub login_info: ClientLoginInfo,
  pub stream: FtpStream,
}
impl Ftp {
  /// Try to connect to the remote server
  pub fn connect(login_info: ClientLoginInfo) -> FtpResult<Self> {
    let stream = FtpStream::connect((login_info.host.clone(), login_info.port))?;
    Ok(Self { login_info, stream })
  }
}
impl Ftp {
  /// Log in to the FTP server.
  pub fn login(&mut self) -> FtpResult<()> {
    self
      .stream
      .login(&self.login_info.uname, &self.login_info.passwd)
  }
}
