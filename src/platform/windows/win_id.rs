use winapi;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WindowId(winapi::HWND);

impl From<winapi::HWND> for WindowId {
    #[inline]
    fn from(id: winapi::HWND) -> WindowId {
        WindowId(id)
    }
}

impl Into<winapi::HWND> for WindowId {
    #[inline]
    fn into(self) -> winapi::HWND {
        self.0
    }
}
