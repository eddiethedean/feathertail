"""
Feathertail - A tiny, fast, Rust-backed transformation core for Python table data
"""

# Import the compiled module
# The PyO3 module should be available as a standard Python module
try:
    # Try to import the compiled module directly
    import feathertail as _feathertail
    from _feathertail import *
except ImportError:
    # If that fails, try to find and load the .so file manually
    import sys
    import os
    import importlib.util
    
    # Get the current directory where this __init__.py is located
    current_dir = os.path.dirname(__file__)
    
    # Find the compiled .so file for the current Python version
    python_version = f"cpython-{sys.version_info.major}{sys.version_info.minor}"
    so_files = [f for f in os.listdir(current_dir) if f.startswith('feathertail') and f.endswith('.so') and python_version in f]
    if not so_files:
        # Fallback to any .so file if version-specific one not found
        so_files = [f for f in os.listdir(current_dir) if f.startswith('feathertail') and f.endswith('.so')]
    if not so_files:
        raise ImportError("Could not find compiled feathertail module (.so file)")
    
    # Load the compiled module
    so_file = os.path.join(current_dir, so_files[0])
    spec = importlib.util.spec_from_file_location("feathertail", so_file)
    feathertail_module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(feathertail_module)
    
    # Import all public symbols from the compiled module
    for name in dir(feathertail_module):
        if not name.startswith('_'):
            globals()[name] = getattr(feathertail_module, name)

__version__ = "0.5.0"
__author__ = "Odos Matthews"
__email__ = "odosmatthews@gmail.com"
