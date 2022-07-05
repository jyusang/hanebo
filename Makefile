CARGO := env -S $$(cat .env) cargo

.PHONY: run-debug
run-debug:
	$(CARGO) run

.PHONY: run
run:
	$(CARGO) run --release
