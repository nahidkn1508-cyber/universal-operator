use super::{AIProvider, PROVIDERS};

pub fn match_provider(url: &str) -> Option<&'static AIProvider> {
    for provider in PROVIDERS {
        for domain in provider.domains {
            if url.contains(domain) {
                return Some(provider);
            }
        }
    }

    None
}
