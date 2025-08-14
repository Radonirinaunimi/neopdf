# Minimal makefile for Sphinx documentation

SPHINXOPTS    ?= -j auto
SPHINXBUILD   ?= sphinx-build
SOURCEDIR     = source
BUILDDIR      = build
DOXYFILE      = Doxyfile.in

help:
	@$(SPHINXBUILD) -M help "$(SOURCEDIR)" "$(BUILDDIR)" $(SPHINXOPTS) $(O)

.PHONY: help Makefile all clean xml html

all: html

xml:
	@echo "Running Doxygen..."
	@mkdir -p xml
	@doxygen $(DOXYFILE)

html: xml
	@echo "Building HTML documentation..."
	@$(SPHINXBUILD) -b html "$(SOURCEDIR)" "$(BUILDDIR)/html" $(SPHINXOPTS)

clean:
	@echo "Cleaning build directory and Doxygen output..."
	@rm -rf $(BUILDDIR) xml
