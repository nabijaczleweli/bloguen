use std::env;


#[inline]
pub fn default_language_impl() -> Option<String> {
    for v in &["LANG", "LANGUAGE", "LC_NAME"] {
        let out = env::var(v).ok().and_then(|l| {
            let main = if let Some(i) = l.find('.') {
                &l[..i]
            } else {
                &l
            };

            if !["", "C", "POSIX"].contains(&main) {
                Some(main.replace('_', "-"))
            } else {
                None
            }
        });
        if out.is_some() {
            return out;
        }
    }

    None
}
