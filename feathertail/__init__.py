"""
Feathertail - A tiny, fast, Rust-backed transformation core for Python table data
"""

try:
    from .feathertail import *
    print("Import successful from __init__.py")
except ImportError as e:
    print(f"Import failed: {e}")
    # Fallback to direct import
    import feathertail.feathertail
    from feathertail.feathertail import *

__version__ = "0.4.1"
__author__ = "Odos Matthews"
__email__ = "odosmatthews@gmail.com"