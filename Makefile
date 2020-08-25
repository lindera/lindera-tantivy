LINDERA_TANTIVY_VERSION ?= $(shell cargo metadata --no-deps --format-version=1 | jq -r '.packages[] | select(.name=="lindera-tantivy") | .version')

.DEFAULT_GOAL := build

clean:
	cargo clean

format:
	cargo fmt

build:
	cargo build --release

test:
	cargo test

tag:
	git tag v$(LINDERA_TANTIVY_VERSION)
	git push origin v$(LINDERA_TANTIVY_VERSION)

publish:
ifeq ($(shell curl -s -XGET https://crates.io/api/v1/crates/lindera-tantivy | jq -r '.versions[].num' | grep $(LINDERA_TANTIVY_VERSION)),)
	cargo package && cargo publish
endif
