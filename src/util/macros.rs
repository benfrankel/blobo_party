/// Helper trait to be generic over `Option` and `Result`.
pub trait Success<T> {
    fn success(self) -> Option<T>;
}

impl<T> Success<T> for Option<T> {
    fn success(self) -> Option<T> {
        self
    }
}

impl<T, E> Success<T> for Result<T, E> {
    fn success(self) -> Option<T> {
        self.ok()
    }
}

/// Unwrap or warn and return.
#[macro_export]
macro_rules! r {
    ($return:expr, $expr:expr $(,)?) => {
        match $crate::util::macros::Success::success($expr) {
            Some(x) => x,
            None => {
                warn!("Unwrap failed: {}", stringify!($expr));
                return $return;
            },
        }
    };

    ($expr:expr $(,)?) => {
        match $crate::util::macros::Success::success($expr) {
            Some(x) => x,
            None => {
                warn!("Unwrap failed: {}", stringify!($expr));
                return;
            },
        }
    };
}

/// Unwrap or return quietly.
#[macro_export]
macro_rules! rq {
    ($return:expr, $expr:expr $(,)?) => {
        match $crate::util::macros::Success::success($expr) {
            Some(x) => x,
            None => return $return,
        }
    };

    ($expr:expr $(,)?) => {
        match $crate::util::macros::Success::success($expr) {
            Some(x) => x,
            None => return,
        }
    };
}

/// Unwrap or warn and continue.
#[macro_export]
macro_rules! c {
    ($expr:expr) => {
        match $crate::util::macros::Success::success($expr) {
            Some(x) => x,
            None => {
                warn!("Unwrap failed: {}", stringify!($expr));
                continue;
            },
        }
    };
}

/// Unwrap or continue quiety.
#[macro_export]
macro_rules! cq {
    ($expr:expr) => {
        match $crate::util::macros::Success::success($expr) {
            Some(x) => x,
            None => continue,
        }
    };
}
