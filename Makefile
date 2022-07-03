CARGO := env -S $$(cat .env) cargo

.PHONY: run
run:
	$(CARGO) run
