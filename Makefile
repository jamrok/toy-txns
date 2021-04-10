SHELL            := /bin/bash
APP_NAME         := toy-txns
RELEASE_DIR      := target/release

.DEFAULT_GOAL    := help

.PHONY: help # - Show a list of all the available make targets
help:
	@echo "Run 'make' with one or more of the following targets:"
	@grep '^.PHONY:.*' Makefile | cut -d: -f2 | \
		(sort -V 2>/dev/null || sort) | (column -t -s '#' || cat -)

.PHONY: test # - Run the test suite.
test:
	cargo test -- --nocapture

.PHONY: run # - Run the dev version of the application. Set the $FILE variable to override the default file.
run:
	cargo run -- $${FILE:-tests/transactions.csv}

.PHONY: release # - Build the release version of the application.
release:
	cargo build --release
	cp -vf $(RELEASE_DIR)/$(APP_NAME) .

.PHONY: run_release # - Run the release version of the application. Set the $FILE variable to override the default file.
run_release:
	@./$(RELEASE_DIR)/$(APP_NAME) $${FILE:-tests/transactions.csv}

.PHONY: run_all_targets # - Run all the make targets to see if they are valid (Use with caution).
run_all_targets:
	$(eval TARGET_EXCLUDES := 'run_all_targets')
	@echo "Excluding targets with: $(TARGET_EXCLUDES)"
	@sleep 4
	@make | awk '{print $$1}' | grep -vE "^make\[[0-9]" | tail -n+2 \
    | grep -Ev $(TARGET_EXCLUDES) \
    | while read i; do \
        echo "Checking Target [$$i]"; \
        make $$i || { \
          echo "TARGET [$$i] FAILED" && break; \
        }; echo -e "===\n"; done

