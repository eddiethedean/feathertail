# Import from the compiled module
try:
    from . import feathertail
    TinyFrame = feathertail.TinyFrame
    TinyGroupBy = feathertail.TinyGroupBy
except ImportError:
    # Fallback: try to find the compiled module manually
    import importlib.util
    import os
    
    current_dir = os.path.dirname(os.path.abspath(__file__))
    so_file = None
    
    # Look for compiled module with various extensions
    for file in os.listdir(current_dir):
        if (file.endswith('.so') or 
            file.endswith('.pyd') or 
            (file.startswith('feathertail') and '.' in file)):
            so_file = os.path.join(current_dir, file)
            break
    
    if so_file:
        spec = importlib.util.spec_from_file_location("feathertail", so_file)
        feathertail_module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(feathertail_module)
        
        TinyFrame = feathertail_module.TinyFrame
        TinyGroupBy = feathertail_module.TinyGroupBy
    else:
        raise ImportError(f"Could not find compiled feathertail module in {current_dir}. Available files: {os.listdir(current_dir)}")

__all__ = ["TinyFrame", "TinyGroupBy"]
__version__ = '0.4.0'