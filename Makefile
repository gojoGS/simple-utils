install:
	cargo install --path .

docs:
	mdbook build docs

docs-serve:
	mdbook serve docs
