.PHONY: release
target/release/collate target/release/uncollate:
	cargo build --release

.PHONY: debug
target/debug/collate target/debug/uncollate:
	cargo build

.PHONY: install
install:
	cargo install --path .

test/%.txt.collate.exp: test/%.txt debug
	target/debug/collate $< > $@ && git add --intent-to-add $@

test/%.txt.uncollate.exp: test/%.txt debug
	target/debug/uncollate $< > $@ && git add --intent-to-add $@

collate_tests := $(patsubst %.txt,%.txt.collate.exp,$(wildcard test/*.txt))
uncollate_tests := $(patsubst %.txt,%.txt.uncollate.exp,$(wildcard test/*.txt))

.PHONY: test
test: $(collate_tests) $(uncollate_tests)
	git diff --exit-code $^
