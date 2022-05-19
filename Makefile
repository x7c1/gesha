
help: ## docs : Display tasks
	@cat Makefile |\
	egrep '^[A-Z0-9a-z-]+:' |\
	sed -e 's/:[ ]*##[ ]*/:/' |\
	column -t -s :

swagger-ui: ## docs :
	./scripts/swagger-ui.sh

cargo-clippy: ## lint :
	cargo clippy -- --no-deps -D warnings

cargo-fmt: ## format :
	cargo fmt

deno-fmt: ## format :
	cd e2e-tests && deno fmt

format: ## format
	make cargo-fmt
	make deno-fmt

e2e-test: ## test : Run e2e tests
	./scripts/e2e-test.sh

gesha-sample: ## debug : Sample gesha command
	cargo run --bin gesha -- generate --schema schemas/v3.0/petstore.yaml

gesha-test: ## test : Test gesha command
	cargo run --bin gesha -- generate-sample --schema tests/pet.yaml
