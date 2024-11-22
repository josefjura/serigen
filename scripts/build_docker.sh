#!/bin/bash

# Extract the version from Cargo.toml
VERSION=$(grep '^version' Cargo.toml | sed -E 's/version = "(.*)"/\1/')

echo "Building Docker image with tag: serigen:${VERSION}"

# Build the Docker image with the version as the tag
docker build -t serigen:${VERSION} .