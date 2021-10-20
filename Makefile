
help: ## docs : display tasks
	@cat Makefile |\
	egrep '^[A-Za-z-]+:' |\
	sed -e 's/:[ ]*##[ ]*/:/' |\
	column -t -s :

deno-test: ## tests : run e2e tests
	./scripts/e2e-test.sh

deno-fmt: ## formatter :
	cd e2e-tests && deno fmt