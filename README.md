# 🪶 feathertail

A high-performance Python DataFrame library powered by Rust — designed for flexibility, blazing speed, and intelligent type handling. Built for production with comprehensive features, advanced analytics, and enterprise-grade performance.

---

## ✨ Key Features

### 🚀 **Core DataFrame Operations**
- ✅ Build `TinyFrame` from Python dict records (`from_dicts`)
- ✅ Automatic type inference, including mixed-type and optional columns
- ✅ Intelligent fallback to Python objects when Rust-native types aren't possible
- ✅ Flexible `fillna` to handle missing data
- ✅ Powerful `cast_column` to convert columns between types
- ✅ Smart `edit_column`: edits that automatically adjust column type if needed
- ✅ Drop or rename columns easily
- ✅ Export back to Python dicts (`to_dicts`)

### 🔗 **Advanced Data Operations**
- ✅ **Join Operations**: Inner, left, right, outer, and cross joins
- ✅ **Filtering & Sorting**: Advanced filtering with multiple conditions and multi-column sorting
- ✅ **GroupBy Aggregations**: Comprehensive statistical operations (sum, mean, min, max, std, var, median, first, last, count, size)
- ✅ **Window Functions**: Rolling and expanding window operations
- ✅ **Ranking Functions**: Rank calculation with multiple methods and percentage change

### 📊 **Advanced Analytics**
- ✅ **Descriptive Statistics**: `describe()`, `skew()`, `kurtosis()`, `quantile()`, `mode()`, `nunique()`
- ✅ **Correlation & Covariance**: Full correlation/covariance matrices and pairwise calculations
- ✅ **Time Series Operations**: DateTime parsing, component extraction, time differences, and shifting
- ✅ **String Operations**: Case conversion, whitespace removal, replacement, splitting, pattern matching, length, and concatenation
- ✅ **Data Validation**: Not null, range, pattern, uniqueness validation with comprehensive reporting

### ⚡ **Performance & Optimization**
- ✅ **SIMD Operations**: x86_64 optimized numerical operations for blazing speed
- ✅ **Parallel Processing**: Multi-core operations using Rayon for GroupBy, filtering, and sorting
- ✅ **Memory Optimization**: String interning, lazy evaluation, and copy-on-write optimizations
- ✅ **Chunked Processing**: Handle large datasets efficiently with streaming operations
- ✅ **Rust-backed Core**: Lightweight, fast, and dependency-light

### 🛠️ **Developer Experience**
- ✅ **Comprehensive Documentation**: Sphinx-generated API docs with tutorials and guides
- ✅ **Logging & Debugging**: Built-in logging system with performance monitoring
- ✅ **Profiling Tools**: Performance profiling and optimization insights
- ✅ **Development Tools**: Pre-commit hooks, automated testing, and development scripts
- ✅ **239 Comprehensive Tests**: Full test coverage running in 0.17 seconds

---

## 📦 Installation

```bash
pip install feathertail
```

> **⚠️ Note**: Currently, the PyPI package contains a build compiled on macOS. We're working on setting up cross-platform builds for Linux and Windows. For now, you may need to build from source on non-macOS systems.

### Building from Source (Recommended for non-macOS)

```bash
# Clone the repository
git clone https://github.com/your-username/feathertail.git
cd feathertail

# Install dependencies and build
pip install maturin
maturin develop --release

# Or install in development mode
pip install -e .
```

---

## 🧑‍💻 Quickstart

### Basic DataFrame Operations

```python
import feathertail as ft

records = [
    {"name": "Alice", "age": 30, "city": "New York", "score": 95.5},
    {"name": "Bob", "age": None, "city": "Paris", "score": 85.0},
    {"name": "Charlie", "age": 25, "city": "New York", "score": None},
]

frame = ft.TinyFrame.from_dicts(records)
print(frame)
```

**Output:**
```
TinyFrame(rows=3, columns=4, cols={ 'name': 'Str', 'age': 'OptInt', 'city': 'Str', 'score': 'OptFloat' })
```

### Advanced Filtering and Sorting

```python
# Filter and sort data
filtered = frame.filter("age", ">", 25)
sorted_frame = frame.sort_values(["city", "age"], ascending=[True, False])
print(sorted_frame.to_dicts())
```

### GroupBy Aggregations

```python
# Comprehensive statistical aggregations
groupby = frame.groupby("city")
stats = groupby.agg([("age", "mean"), ("score", "max"), ("name", "count")])
print(stats.to_dicts())
```

### Join Operations

```python
# Inner join with another DataFrame
other_data = [
    {"city": "New York", "population": 8_000_000},
    {"city": "Paris", "population": 2_000_000},
]
other_frame = ft.TinyFrame.from_dicts(other_data)

joined = frame.join(other_frame, "city", "city", "inner")
print(joined.to_dicts())
```

### Advanced Analytics

```python
# Descriptive statistics
description = frame.describe("score")
print(description.to_dicts())

# Correlation analysis
correlation = frame.corr("age", "score")
print(f"Age-Score correlation: {correlation}")

# Time series operations
time_data = [
    {"timestamp": "2023-01-01 10:00:00", "value": 100},
    {"timestamp": "2023-01-01 11:00:00", "value": 120},
]
time_frame = ft.TinyFrame.from_dicts(time_data)
time_frame = time_frame.to_timestamps("timestamp")
time_frame = time_frame.dt_year("timestamp_ts")
print(time_frame.to_dicts())
```

### Window Functions

```python
# Rolling window operations
data = [{"value": i} for i in range(1, 11)]
window_frame = ft.TinyFrame.from_dicts(data)
rolling_mean = window_frame.rolling_mean("value", 3)
print(rolling_mean.to_dicts())
```

### String Operations

```python
# String manipulation
text_data = [{"text": "  hello world  "}, {"text": "foo bar"}]
text_frame = ft.TinyFrame.from_dicts(text_data)
processed = text_frame.str_upper("text").str_strip("text")
print(processed.to_dicts())
```

### Data Validation

```python
# Data quality checks
validation = frame.validate_not_null("age")
validation_summary = frame.validation_summary("age")
print(f"Validation summary: {validation_summary}")
```

---

## 🚀 Performance Features

### SIMD-Accelerated Operations
```python
# Automatic SIMD optimization for numerical operations
large_data = [{"value": i * 1.5} for i in range(100000)]
large_frame = ft.TinyFrame.from_dicts(large_data)

# These operations use SIMD for maximum performance
sum_result = large_frame.groupby("value").agg([("value", "sum")])
```

### Parallel Processing
```python
# Multi-core operations for large datasets
# Automatically uses all available CPU cores
filtered = large_frame.filter("value", ">", 50000)
sorted_data = large_frame.sort_values("value")
```

### Memory Optimization
```python
# String interning and lazy evaluation
# Memory usage is automatically optimized
frame = ft.TinyFrame.from_dicts(records)
# Operations are optimized for memory efficiency
```

---

## 🛠️ Developer Tools

### Logging and Debugging
```python
# Enable comprehensive logging
ft.init_logging_with_config("info", log_memory=True, log_performance=True, log_operations=True)

# Enable debug mode
ft.enable_debug()

# Enable profiling
ft.enable_profiling()

# Your operations will be logged and profiled
frame = ft.TinyFrame.from_dicts(data)
result = frame.filter("age", ">", 25)

# View profiling report
ft.print_profiling_report()
```

### Performance Monitoring
```python
# Get operation statistics
stats = ft.get_operation_stats("filter")
print(f"Filter operations: {stats}")

# Get overall performance metrics
overall_stats = ft.get_overall_stats()
print(f"Total operations: {overall_stats['total_operations']}")
```

---

## ⚙️ Supported Types

| Type      | Column variants    | Description |
|-----------|-------------------|-------------|
| int       | `Int`, `OptInt`    | 64-bit integers with optional null support |
| float     | `Float`, `OptFloat` | 64-bit floats with optional null support |
| bool      | `Bool`, `OptBool`  | Boolean values with optional null support |
| str       | `Str`, `OptStr`    | UTF-8 strings with optional null support |
| mixed     | `Mixed`, `OptMixed` | Mixed types with automatic Python object fallback |

---

## 📚 Documentation

- **[Getting Started Guide](docs/getting_started.md)** - Learn the basics
- **[Advanced Usage](docs/advanced_usage.md)** - Complex operations and patterns
- **[API Reference](docs/api/index.rst)** - Complete API documentation
- **[Tutorials](docs/tutorials/index.md)** - Step-by-step learning guides
- **[Contributing](CONTRIBUTING.md)** - How to contribute to the project

---

## 🧪 Testing

```bash
# Run all tests (239 tests in ~0.17 seconds)
make test

# Run specific test categories
python -m pytest tests/python/unit/test_tinyframe.py
python -m pytest tests/python/unit/test_joins.py
python -m pytest tests/python/unit/test_analytics.py
```

---

## 🏗️ Building from Source

```bash
# Clone the repository
git clone https://github.com/your-username/feathertail.git
cd feathertail

# Set up development environment
make dev

# Build the package
make build

# Run tests
make test

# Build documentation
make docs
```

---

## 🐉 Why "feathertail"?

In *Fourth Wing*, a "feathertail" is a juvenile dragon — small, golden, and nonviolent, known for grace rather than brute force.  

This library follows the same spirit: gentle on dependencies, elegant in design, and capable of handling complex data types with ease — but with the power and performance of a full-grown dragon when you need it.

---

## 📊 Performance Benchmarks

- **239 comprehensive tests** run in just **0.17 seconds**
- **SIMD-accelerated** numerical operations
- **Parallel processing** for multi-core performance
- **Memory-optimized** with string interning and lazy evaluation
- **Production-ready** with comprehensive error handling and logging

---

## ❤️ Contributing

Contributions, ideas, and feedback are always welcome! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

---

## 📄 License

MIT

---

## 🎯 Roadmap

- [ ] **Cross-platform PyPI builds** - Set up automated builds for Linux and Windows
- [ ] Additional time series functions
- [ ] More statistical distributions
- [ ] Enhanced plotting integration
- [ ] Database connectors
- [ ] Arrow/Parquet integration

---

*Built with ❤️ using Rust and Python*