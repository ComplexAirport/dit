#[macro_export]
macro_rules! success {
    ($($arg:tt)*) => {
        println!(
            "{} {}",
            console::style("[+]").green().bold(),
            format!($($arg)*)
        )
    };
}

#[macro_export]
macro_rules! failure {
    ($($arg:tt)*) => {
        println!(
            "{} {}",
            console::style("[!]").red().bold(),
            format!($($arg)*)
        )
    };
}

#[macro_export]
macro_rules! warning {
    ($($arg:tt)*) => {
        println!(
            "{} {}",
            console::style("[~]").yellow().bold(),
            format!($($arg)*)
        )
    };
}

#[macro_export]
macro_rules! hint {
        ($($arg:tt)*) => {
        println!(
            "{} hint: {}",
            console::style("[*]").dim().bold(),
            format!($($arg)*)
        )
    };
}
