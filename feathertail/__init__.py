"""
Feathertail - A tiny, fast, Rust-backed transformation core for Python table data
"""

# Import the compiled module
try:
    from .feathertail import *
except ImportError:
    # Fallback for wheel installation
    import feathertail.feathertail
    from feathertail.feathertail import *

__version__ = "0.4.1"
__author__ = "Odos Matthews"
__email__ = "odosmatthews@gmail.com"