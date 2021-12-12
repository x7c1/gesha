
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

e2e-test: ## tests : Run e2e tests
	./scripts/e2e-test.sh
