
# 🪶 feathertail

A high-performance Python DataFrame library powered by Rust — designed for flexibility, blazing speed, and intelligent type handling.

---

## ✨ Features

- ✅ Build `TinyFrame` from Python dict records (`from_dicts`)
- ✅ Automatic type inference, including mixed-type and optional columns
- ✅ Intelligent fallback to Python objects when Rust-native types aren’t possible
- ✅ Flexible `fillna` to handle missing data
- ✅ Powerful `cast_column` to convert columns between types
- ✅ Smart `edit_column`: edits that automatically adjust column type if needed
- ✅ Drop or rename columns easily
- ✅ Group-by aggregations via `TinyGroupBy`
- ✅ Export back to Python dicts (`to_dicts`)
- ✅ Rust-backed core: lightweight, fast, and dependency-light

---

## 📦 Installation

```bash
pip install feathertail
```

Or, from local source:

```bash
pip install -e .
```

---

## 🧑‍💻 Quickstart

```python
import feathertail as ft

records = [
    {"name": "Alice", "age": 30, "city": "New York", "score": 95.5},
    {"name": "Bob", "age": None, "city": "Paris", "score": 85.0},
    {"name": "Charlie", "age": 25, "city": "New York", "score": None},
]

frame = ft.TinyFrame.from_dicts(records)
print(frame)
print(frame.to_dicts())
```

**Output:**
```
TinyFrame(rows=3, columns=4, cols={ 'name': 'Str', 'age': 'OptInt', 'city': 'Str', 'score': 'OptFloat' })
[{'name': 'Alice', 'age': 30, 'city': 'New York', 'score': 95.5}, ...]
```

---

### Fill missing values

```python
frame.fillna({"age": 20, "score": 0.0})
print(frame.to_dicts())
```

```
[{'name': 'Alice', 'age': 30, 'city': 'New York', 'score': 95.5},
 {'name': 'Bob', 'age': 20, 'city': 'Paris', 'score': 85.0},
 {'name': 'Charlie', 'age': 25, 'city': 'New York', 'score': 0.0}]
```

---

### Cast and edit columns

```python
frame.cast_column("score", float)
frame.edit_column("city", lambda x: x.upper() if x else x)
print(frame.to_dicts())
```

```
[{'name': 'Alice', 'age': 30, 'city': 'NEW YORK', 'score': 95.5},
 {'name': 'Bob', 'age': 20, 'city': 'PARIS', 'score': 85.0},
 {'name': 'Charlie', 'age': 25, 'city': 'NEW YORK', 'score': 0.0}]
```

---

### Drop and rename columns

```python
frame.drop_columns(["score"])
frame.rename_column("name", "full_name")
print(frame.to_dicts())
```

```
[{'full_name': 'Alice', 'age': 30, 'city': 'NEW YORK'},
 {'full_name': 'Bob', 'age': 20, 'city': 'PARIS'},
 {'full_name': 'Charlie', 'age': 25, 'city': 'NEW YORK'}]
```

---

### Group by columns

```python
groupby = ft.TinyGroupBy(frame, keys=["city"])
count_frame = groupby.count(frame)
print(count_frame.to_dicts())
```

```
[{'city': 'NEW YORK', 'count': 2},
 {'city': 'PARIS', 'count': 1}]
```

---

## ⚙️ Supported Types

| Type      | Column variants    |
|------------|---------------------|
| int        | `Int`, `OptInt`    |
| float      | `Float`, `OptFloat` |
| bool       | `Bool`, `OptBool`  |
| str        | `Str`, `OptStr`    |
| mixed & fallback | `Mixed`, `OptMixed` (includes Python fallback objects automatically) |

---

## 🐉 Why "feathertail"?

In *Fourth Wing*, a "feathertail" is a juvenile dragon — small, golden, and nonviolent, known for grace rather than brute force.  

This library follows the same spirit: gentle on dependencies, elegant in design, and capable of handling complex data types with ease.

---

## ❤️ Contributing

Contributions, ideas, and feedback are always welcome! Please open an issue or pull request.

---

## 📄 License

MIT
