#![warn(missing_docs)]
//! A simple library for getting random user agents from a list of popular browsers
//! such as Chrome, Firefox, Safari, Opera, Edge, and Internet Explorer.
//!
//! The user agents are hard-coded into the binary. They can be found [here](https://github.com/fake-useragent/fake-useragent/blob/master/src/fake_useragent/data/browsers.json).
//!
//! # Examples
//!
//! Get a random user agent from Chrome, Opera, Firefox, Safari, Edge, or Internet Explorer:
//! ```
//! use random_user_agent::get_rua;
//! let rua = get_rua(); // String
//! ```
//!
//! If you want a *specific type of browser* user agent, you can use the following:
//!
//! Get a random Chrome user agent:
//! ```
//! use random_user_agent::get_chrome_rua;
//! let rua = get_chrome_rua(); // String
//! ```
//!
//! Get a random Opera user agent:
//! ```
//! use random_user_agent::get_opera_rua;
//! let rua = get_opera_rua(); // String
//! ```
//!
//! Get a random Firefox user agent:
//! ```
//! use random_user_agent::get_firefox_rua;
//! let rua = get_firefox_rua(); // String
//! ```
//!
//! Get a random Safari user agent:
//! ```
//! use random_user_agent::get_safari_rua;
//! let rua = get_safari_rua(); // String
//! ```
//!
//! Get a random Edge user agent:   
//! ```
//! use random_user_agent::get_edge_rua;
//! let rua = get_edge_rua(); // String
//! ```
//!
//! Get a random Internet Explorer user agent:
//! ```
//! use random_user_agent::get_ie_rua;
//! let rua = get_ie_rua(); // String
//! ```

/// User agent module
mod user_agents;

/// Using fastrand for random number generation
extern crate fastrand;

/// Importing user agents
use user_agents::{
    CHROME_USER_AGENTS, EDGE_USER_AGENTS, FIREFOX_USER_AGENTS, IE_USER_AGENTS, OPERA_USER_AGENTS,
    SAFARI_USER_AGENTS,
};

/// Gets a random chrome user agent
pub fn get_chrome_rua() -> String {
    let ri = fastrand::usize(..CHROME_USER_AGENTS.len());
    CHROME_USER_AGENTS[ri].to_string()
}

/// Gets a random opera user agent
pub fn get_opera_rua() -> String {
    let ri = fastrand::usize(..OPERA_USER_AGENTS.len());
    OPERA_USER_AGENTS[ri].to_string()
}

/// Gets a random firefox user agent
pub fn get_firefox_rua() -> String {
    let ri = fastrand::usize(..FIREFOX_USER_AGENTS.len());
    FIREFOX_USER_AGENTS[ri].to_string()
}

/// Gets a random safari user agent
pub fn get_safari_rua() -> String {
    let ri = fastrand::usize(..SAFARI_USER_AGENTS.len());
    SAFARI_USER_AGENTS[ri].to_string()
}

/// Gets a random edge user agent
pub fn get_edge_rua() -> String {
    let ri = fastrand::usize(..EDGE_USER_AGENTS.len());
    EDGE_USER_AGENTS[ri].to_string()
}

/// Gets a random internet explorer user agent
pub fn get_ie_rua() -> String {
    let ri = fastrand::usize(..IE_USER_AGENTS.len());
    IE_USER_AGENTS[ri].to_string()
}

/// Gets a random user agent (Chrome, Opera, Firefox, Safari, Edge, or IE).
pub fn get_rua() -> String {
    let browser = BrowserType::get_random();
    match browser {
        BrowserType::CRHOME => get_chrome_rua(),
        BrowserType::OPERA => get_opera_rua(),
        BrowserType::FIREFOX => get_firefox_rua(),
        BrowserType::SAFARI => get_safari_rua(),
        BrowserType::EDGE => get_edge_rua(),
        BrowserType::IE => get_ie_rua(),
    }
}

enum BrowserType {
    CRHOME,
    OPERA,
    FIREFOX,
    SAFARI,
    EDGE,
    IE,
}

impl BrowserType {
    pub fn get_random() -> Self {
        let random_number = fastrand::usize(..5);
        match random_number {
            0 => BrowserType::CRHOME,
            1 => BrowserType::OPERA,
            2 => BrowserType::FIREFOX,
            3 => BrowserType::SAFARI,
            4 => BrowserType::EDGE,
            5 => BrowserType::IE,
            _ => {
                println!("Defaulting to Chrome");
                BrowserType::CRHOME
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_chrome_rua() {
        let rua = get_chrome_rua();
        println!("{}", rua);
        assert!(CHROME_USER_AGENTS.contains(&rua.as_str()));
    }

    #[test]
    fn test_get_opera_rua() {
        let rua = get_opera_rua();
        assert!(OPERA_USER_AGENTS.contains(&rua.as_str()));
    }

    #[test]
    fn test_get_firefox_rua() {
        let rua = get_firefox_rua();
        assert!(FIREFOX_USER_AGENTS.contains(&rua.as_str()));
    }

    #[test]
    fn test_get_safari_rua() {
        let rua = get_safari_rua();
        assert!(SAFARI_USER_AGENTS.contains(&rua.as_str()));
    }

    #[test]
    fn test_get_edge_rua() {
        let rua = get_edge_rua();
        assert!(EDGE_USER_AGENTS.contains(&rua.as_str()));
    }

    #[test]
    fn test_get_ie_rua() {
        let rua = get_ie_rua();
        assert!(IE_USER_AGENTS.contains(&rua.as_str()));
    }

    #[test]
    fn test_get_rua() {
        let rua = get_rua();
        assert!(
            CHROME_USER_AGENTS.contains(&rua.as_str())
                || OPERA_USER_AGENTS.contains(&rua.as_str())
                || FIREFOX_USER_AGENTS.contains(&rua.as_str())
                || SAFARI_USER_AGENTS.contains(&rua.as_str())
                || EDGE_USER_AGENTS.contains(&rua.as_str())
                || IE_USER_AGENTS.contains(&rua.as_str())
        );
    }
}
