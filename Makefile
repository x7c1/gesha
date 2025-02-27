
help: ## docs : Display tasks
	@cat Makefile |\
	egrep '^[A-Z0-9a-z-]+:' |\
	sed -e 's/:[ ]*##[ ]*/:/' |\
	column -t -s :

swagger-ui: ## docs :
	./scripts/swagger-ui.sh

deno-fmt: ## format :
	cd e2e-tests && deno fmt

format: ## format
	make cargo-fmt
	make deno-fmt

cargo-fmt: ## format :
	cargo fmt

cargo-fmt-check: ## format :
	cargo fmt -- --check

cargo-clippy: ## lint :
	cargo clippy -- \
	    --no-deps \
	    --deny warnings

cargo-clippy-fix: ## lint :
	cargo clippy --fix -- \
	    --no-deps \
	    --deny warnings

cargo-test: ## test :
	cargo test -- --nocapture

cargo-test-no-run: ## test : Compile tests
	cargo test --no-run

e2e-test: ## test : Run e2e tests
	./scripts/e2e-test.sh

gesha-verify: ## test : Test gesha command
	./scripts/setup-test-dirs.sh
	cargo run --bin gesha-verify
	./scripts/test-examples.sh

gesha-verify-overwrite: ## test : Overwrite examples by generated files
	./scripts/setup-test-dirs.sh
	cargo run --bin gesha-verify -- --overwrite

gesha-sample: ## debug : Sample command
	cargo run --bin gesha -- \
	    --schema schemas/v3.0/petstore.yaml \
	    --output output/v3.0/example/petstore.rs
