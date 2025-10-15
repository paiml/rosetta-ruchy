# Bash Selection Sort Makefile

SHELLCHECK = shellcheck
SCRIPT = selection_sort.sh

.PHONY: all test lint clean help

all: lint test

lint:
	@echo "Running shellcheck..."
	$(SHELLCHECK) $(SCRIPT)
	@echo "✓ Shellcheck passed"

test: lint
	@echo "Running Bash test suite..."
	./$(SCRIPT) test

clean:
	@echo "No artifacts to clean for Bash implementation"

help:
	@echo "Bash Selection Sort Makefile"
	@echo "============================"
	@echo ""
	@echo "Targets:"
	@echo "  all   - Run lint and test (default)"
	@echo "  lint  - Run shellcheck"
	@echo "  test  - Run test suite"
	@echo "  clean - Clean artifacts"
	@echo "  help  - Show this help"
