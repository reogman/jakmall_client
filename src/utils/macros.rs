#[macro_export]
macro_rules! err_parse {
    ($msg:literal $(,)?) => {
        return Err(anyhow::anyhow!(format!("Parse Error -> {}", $msg)))
    };
    ($err:expr $(,)?) => {
        return Err(anyhow::anyhow!(format!("Parse Error -> {}", $err)))
    };
    ($fmt:expr, $($arg:tt)*) => {
        return Err(anyhow::anyhow!("Parse Error -> {}", format!($fmt, $($arg)*)))
    };
}

#[macro_export]
macro_rules! some_or_err {
    ($some:expr, $errmsg: literal $(,)?) => {
        match $some {
            Some(tar) => tar,
            None => return Err(anyhow::anyhow!(format!("Parse Error -> {}", $errmsg))),
        }
    };
}

#[macro_export]
macro_rules! err_parse_msg {
    ($msg:literal $(,)?) => {
        format!("Fail to retrieve response {}", $msg)
    };
}

#[macro_export]
macro_rules! err_http_msg {
    ($msg:literal $(,)?) => {
        format!("Fail to retrieve response {}", $msg)
    };
}
