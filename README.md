# fake_user_agent

A simple library for getting random user agents for rust.

## Usage

Get a random user agent from Chrome, Opera, Firefox, Safari, Edge, or Internet Explorer:

```rust
use random_user_agent::get_rua;
let rua = get_rua(); // String
```

If you want a _specific type of browser_ user agent, you can use the following:

Get a random Chrome user agent:

```rust
use random_user_agent::get_chrome_rua;
let rua = get_chrome_rua(); // String
```

Get a random Opera user agent:

```rust
use random_user_agent::get_opera_rua;
let rua = get_opera_rua(); // String
```

Get a random Firefox user agent:

```rust
use random_user_agent::get_firefox_rua;
let rua = get_firefox_rua(); // String
```

Get a random Safari user agent:

```rust
use random_user_agent::get_safari_rua;
let rua = get_safari_rua(); // String
```

Get a random Edge user agent:

```rust
use random_user_agent::get_edge_rua;
let rua = get_edge_rua(); // String
```

Get a random Internet Explorer user agent:

```rust
use random_user_agent::get_ie_rua;
let rua = get_ie_rua(); // String
```
