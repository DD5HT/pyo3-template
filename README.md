# pyo3-template

A minimal template to create [PyO3][1] projects to accelerate your Python code with Rust

## Help

First install [cargo generate][0]

```cargo install cargo-generate```

then to use this template:

```cargo generate --git https://github.com/DD5HT/pyo3-template.git --name myproject```

We use a Makefile to auto copy the .so file to the right python directory
to run the template just type in ```make run```

[0]: https://github.com/ashleygwilliams/cargo-generate
[1]: https://github.com/PyO3/pyo3

