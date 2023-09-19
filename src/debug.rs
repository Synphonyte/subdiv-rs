macro_rules! debug_weak {
    ($($weak:tt)*) => {
        $($weak)*
            .upgrade()
            .map(|a| format!("{}", a.borrow().id))
            .unwrap_or("None".to_string())
    };
}

pub(crate) use debug_weak;