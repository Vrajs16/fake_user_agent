# fake_user_agent

A simple library for getting random user agents from a list of popular browsers such as Chrome, Firefox, Safari, Opera, Edge, and Internet Explorer in rust.

**NOTE**: The user agents are hard-coded into the binary. They can be found [here](https://github.com/fake-useragent/fake-useragent/blob/main/src/fake_useragent/data/browsers.jsonl).

## Usage

Get a random user agent from Chrome, Opera, Firefox, Safari, Edge, or Internet Explorer:

```rust
use fake_user_agent::get_rua;
let rua = get_rua(); // &'static str
```

If you want a *specific type of browser* user agent, you can use the following:

Get a random Chrome user agent:

```rust
use fake_user_agent::get_chrome_rua;
let rua = get_chrome_rua(); // &'static str
```

Get a random Opera user agent:

```rust
use fake_user_agent::get_opera_rua;
let rua = get_opera_rua(); // &'static str
```

Get a random Firefox user agent:

```rust
use fake_user_agent::get_firefox_rua;
let rua = get_firefox_rua(); // &'static str
```

Get a random Safari user agent:

```rust
use fake_user_agent::get_safari_rua;
let rua = get_safari_rua(); // &'static str
```

Get a random Edge user agent:

```rust
use fake_user_agent::get_edge_rua;
let rua = get_edge_rua(); // &'static str
```

Get a random Internet Explorer user agent:

```rust
use fake_user_agent::get_ie_rua;
let rua = get_ie_rua(); // &'static str
```
