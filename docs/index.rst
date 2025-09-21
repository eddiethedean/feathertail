Feathertail Documentation
========================

Feathertail is a high-performance Python DataFrame library powered by Rust, designed to provide pandas-like functionality with superior performance and memory efficiency.

.. toctree::
   :maxdepth: 2
   :caption: Contents:

   getting_started
   advanced_usage
   api/index
   tutorials/index

Features
--------

* **High Performance**: Powered by Rust for blazing-fast operations
* **Memory Efficient**: Optimized memory usage with SIMD and parallel processing
* **Pandas Compatible**: Familiar API that works like pandas
* **Type Safe**: Strong typing with automatic type inference
* **Comprehensive**: Full suite of DataFrame operations including joins, analytics, and time series

Quick Start
-----------

.. code-block:: python

   import feathertail as ft
   
   # Create a DataFrame
   data = [
       {"name": "Alice", "age": 25, "city": "New York"},
       {"name": "Bob", "age": 30, "city": "San Francisco"},
       {"name": "Charlie", "age": 35, "city": "Chicago"}
   ]
   df = ft.TinyFrame.from_dicts(data)
   
   # Basic operations
   print(df.head())
   print(df.describe())
   
   # Filtering and sorting
   filtered = df.filter("age", ">", 25)
   sorted_df = df.sort_values("age", ascending=False)
   
   # GroupBy operations
   grouped = df.groupby("city").agg({"age": "mean"})

API Reference
-------------

.. toctree::
   :maxdepth: 2
   :caption: API Reference:

   api/tinyframe
   api/groupby
   api/joins
   api/analytics
   api/timeseries
   api/string
   api/validation

Indices and tables
==================

* :ref:`genindex`
* :ref:`modindex`
* :ref:`search`
