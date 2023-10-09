# Destination directory for benchmark files
DEST_DIR=benches/files

# Sizes in MB for test files
SIZES=1 10 100 1024 2048

# Script for generating the test files
GENERATE_SCRIPT=./generate_test_file.sh

# Default target
all: bootstrap

# Bootstrap test files of varying sizes using the script
bootstrap: $(DEST_DIR)
	@for size in $(SIZES); do \
		$(GENERATE_SCRIPT) $(DEST_DIR)/benchmark_$$size"M"_random $$size; \
	done

# Create destination directory if it doesn't exist
$(DEST_DIR):
	mkdir -p $(DEST_DIR)

# Clean up the generated test files
clean:
	rm -f $(DEST_DIR)/benchmark_*_random
