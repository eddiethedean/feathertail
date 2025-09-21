# Import from the compiled module
import importlib.util
import os

# Get the path to the compiled module
current_dir = os.path.dirname(os.path.abspath(__file__))
so_file = None
for file in os.listdir(current_dir):
    if file.endswith('.so'):
        so_file = os.path.join(current_dir, file)
        break

if so_file:
    spec = importlib.util.spec_from_file_location("feathertail", so_file)
    feathertail_module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(feathertail_module)
    
    TinyFrame = feathertail_module.TinyFrame
    TinyGroupBy = feathertail_module.TinyGroupBy
else:
    raise ImportError("Could not find compiled feathertail module")

__all__ = ["TinyFrame", "TinyGroupBy"]
__version__ = '0.3.1'