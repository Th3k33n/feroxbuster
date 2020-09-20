use ansi_term::Color::{Blue, Cyan, Green, Red, Yellow};
use reqwest::Url;
use std::convert::TryInto;

/// Helper function that determines the current depth of a given url
///
/// Essentially looks at the Url path and determines how many directories are present in the
/// given Url
///
/// http://localhost -> 1
/// http://localhost/ -> 1
/// http://localhost/stuff -> 2
/// ...
///
/// returns 0 on error and relative urls
pub fn get_current_depth(target: &str) -> usize {
    log::trace!("enter: get_current_depth({})", target);

    let target = if !target.ends_with('/') {
        // target url doesn't end with a /, for the purposes of determining depth, we'll normalize
        // all urls to end in a / and then calculate accordingly
        format!("{}/", target)
    } else {
        String::from(target)
    };

    match Url::parse(&target) {
        Ok(url) => {
            if let Some(parts) = url.path_segments() {
                // at least an empty string returned by the Split, meaning top-level urls
                let mut depth = 0;

                for _ in parts {
                    depth += 1;
                }

                let return_val = depth;

                log::trace!("exit: get_current_depth -> {}", return_val);
                return return_val;
            };

            log::debug!(
                "get_current_depth called on a Url that cannot be a base: {}",
                url
            );
            log::trace!("exit: get_current_depth -> 0");

            0
        }
        Err(e) => {
            log::error!("could not parse to url: {}", e);
            log::trace!("exit: get_current_depth -> 0");
            0
        }
    }
}

/// todo: docs
pub fn status_colorizer(status: &str) -> String {
    match status.chars().next() {
        Some('1') => Blue.paint(status).to_string(), // informational
        Some('2') => Green.bold().paint(status).to_string(), // success
        Some('3') => Yellow.paint(status).to_string(), // redirects
        Some('4') => Red.paint(status).to_string(),  // client error
        Some('5') => Red.paint(status).to_string(),  // server error
        Some('W') => Cyan.paint(status).to_string(), // wildcard
        _ => status.to_string(),                     // ¯\_(ツ)_/¯
    }
}

/// todo docs
pub fn get_url_path_length(url: &Url) -> u64 {
    log::trace!("enter: get_url_path_length({})", url);

    let path = url.path();

    let segments = if path.starts_with('/') {
        path[1..].split_terminator('/')
    } else {
        log::trace!("exit: get_url_path_length -> 0");
        return 0;
    };

    if let Some(last) = segments.last() {
        // failure on conversion should be very unlikely. While a usize can absolutely overflow a
        // u64, the generally accepted maximum for the length of a url is ~2000.  so the value we're
        // putting into the u64 should never realistically be anywhere close to producing an
        // overflow.
        // usize max: 18,446,744,073,709,551,615
        // u64 max:   9,223,372,036,854,775,807
        let url_len: u64 = last
            .len()
            .try_into()
            .expect("Failed usize -> u64 conversion");

        log::trace!("exit: get_url_path_length -> {}", url_len);
        return url_len;
    }

    log::trace!("exit: get_url_path_length -> 0");
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_url_returns_1() {
        let depth = get_current_depth("http://localhost");
        assert_eq!(depth, 1);
    }

    #[test]
    fn base_url_with_slash_returns_1() {
        let depth = get_current_depth("http://localhost/");
        assert_eq!(depth, 1);
    }

    #[test]
    fn one_dir_returns_2() {
        let depth = get_current_depth("http://localhost/src");
        assert_eq!(depth, 2);
    }

    #[test]
    fn one_dir_with_slash_returns_2() {
        let depth = get_current_depth("http://localhost/src/");
        assert_eq!(depth, 2);
    }
}
