# pyo3-template

A minimal template to create [PyO3][1] projects to accelerate your Python3 code with Rust.

## Help

First install [cargo generate][0]

```cargo install cargo-generate```

then to use this template run:

```cargo generate --git https://github.com/DD5HT/pyo3-template.git --name myproject```

Now override your toolchain with the nightly version of rust, just run:

```rustup override set nightly```

Type in ```make run``` and you have a working rust lib in Python!

[0]: https://github.com/ashleygwilliams/cargo-generate
[1]: https://github.com/PyO3/pyo3