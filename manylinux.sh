#!/bin/bash

sudo docker run --rm -v $(pwd):/io ghcr.io/pyo3/maturin:main publish --release
