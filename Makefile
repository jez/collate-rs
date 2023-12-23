.PHONY: target/release/collate
target/release/collate:
	cargo build --release

.PHONY: target/release/uncollate
target/release/uncollate:
	cargo build --release

.PHONY: target/debug/collate
target/debug/collate:
	cargo build

.PHONY: target/debug/uncollate
target/debug/uncollate:
	cargo build

.PHONY: install
install:
	cargo install --path .

test/%.txt.collate.exp: test/%.txt target/debug/collate
	target/debug/collate $< > $@ && git add --intent-to-add $@

test/%.txt.uncollate.exp: test/%.txt target/debug/uncollate
	target/debug/uncollate $< > $@ && git add --intent-to-add $@

collate_tests := $(patsubst %.txt,%.txt.collate.exp,$(wildcard test/*.txt))
uncollate_tests := $(patsubst %.txt,%.txt.uncollate.exp,$(wildcard test/*.txt))

.PHONY: test
test: $(collate_tests) $(uncollate_tests)
	git diff --exit-code $^
