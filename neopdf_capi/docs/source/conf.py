import os
import sys
sys.path.insert(0, os.path.abspath('.'))

project = 'NeoPDF'
copyright = '2024, The NeoPDF team'
author = 'The NeoPDF team'

extensions = [
    'sphinx.ext.autodoc',
    'sphinx.ext.doctest',
    'sphinx.ext.intersphinx',
    'sphinx.ext.todo',
    'sphinx.ext.coverage',
    'sphinx.ext.mathjax',
    'sphinx.ext.ifconfig',
    'sphinx.ext.viewcode',
    'breathe'
]

templates_path = ['_templates']
exclude_patterns = []

html_theme = 'sphinx_rtd_theme'
html_show_sphinx = False

breathe_projects = { 'neopdf': '../xml' }
breathe_default_project = 'neopdf'
