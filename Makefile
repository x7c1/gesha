
help: ## docs : display tasks
	@cat Makefile |\
	egrep '^[A-Za-z-]+:' |\
	sed -e 's/:[ ]*##[ ]*/:/' |\
	column -t -s :

cargo-clippy: ## linter :
	cargo clippy -- --no-deps -D warnings

cargo-fmt: ## formatter
	cargo fmt -- --check

deno-fmt: ## formatter :
	cd e2e-tests && deno fmt

e2e-test: ## tests : run e2e tests
	./scripts/e2e-test.sh
