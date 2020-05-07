/// 定义系统错误。
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SysError(i32);

impl std::fmt::Display for SysError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"Code={}, Reason="{{}}")"#, self.0)
    }
}

impl std::error::Error for SysError {}

impl From<i32> for SysError {
    fn from(val: i32) -> Self {
        Self { 0: val }
    }
}

impl Into<i32> for SysError {
    fn into(self) -> i32 {
        self.0
    }
}

impl SysError {
    /// 从系统当前 errno 创建一个 SysError 对象。
    #[cfg(unix)]
    pub fn last() -> Self {
        unsafe {
            Self {
                0: *(libc::__errno_location()),
            }
        }
    }

    /// 检测是否为出错状态。
    pub fn is_err(self) -> bool {
        self.0 != 0
    }

    /// 检测是否为无错状态。
    pub fn is_ok(self) -> bool {
        !self.is_err()
    }

    /// 当无错时将返回值映射为其他值。
    pub fn map_or<T>(self, other: T) -> Result<T, SysError> {
        if self.is_err() {
            Err(self)
        } else {
            Ok(other)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sys_error() {
        let err = SysError::last();
        assert_eq!(err.is_err(), false);
    }
}
