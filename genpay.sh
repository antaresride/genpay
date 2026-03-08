#!/bin/bash

# Clean the branch name
BRANCH=$(echo "${1:-main}" | tr -d '\n' | tr '[:upper:]' '[:lower:]')

echo "--- Building Genpay (Branch: $BRANCH) ---"

# Build the image using the Dockerfile in the same folder
docker build \
  --build-arg BRANCH="$BRANCH" \
  -t "genpay:$BRANCH" .

# Remove the branch name from arguments so only compiler flags remain
shift

# Run the container
docker run --rm -it \
  -v "$(pwd):/src" \
  -u "$(id -u):$(id -g)" \
  "genpay:$BRANCH" "$@"
