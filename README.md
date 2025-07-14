
# 🪶 feathertail

High-performance Python DataFrame library powered by Rust, designed for flexibility, speed, and clear schema handling.

---

## ✨ Features

- ✅ Build `TinyFrame` from Python dict records (`from_dicts`)
- ✅ Automatic type inference and mixed type handling
- ✅ Support for optional and mixed columns, with fallback to PyObject if needed
- ✅ Fill missing values (`fillna`)
- ✅ Cast columns to different types (`cast_column`)
- ✅ Edit columns flexibly with type-adapting edits (`edit_column`)
- ✅ Drop or rename columns
- ✅ Simple group-by aggregations with `TinyGroupBy`
- ✅ Export to Python dicts (`to_dicts`)
- ✅ Lightweight, fast, pure Rust core

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

## 🧑‍💻 Usage

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

```python
frame.fillna({"age": 20, "score": 0.0})
print(frame.to_dicts())
```

**Output:**
```
[{'name': 'Alice', 'age': 30, 'city': 'New York', 'score': 95.5}, {'name': 'Bob', 'age': 20, 'city': 'Paris', 'score': 85.0}, {'name': 'Charlie', 'age': 25, 'city': 'New York', 'score': 0.0}]
```

```python
frame.cast_column("score", float)
frame.edit_column("city", lambda x: x.upper() if x else x)
print(frame.to_dicts())
```

**Output:**
```
[{'name': 'Alice', 'age': 30, 'city': 'NEW YORK', 'score': 95.5}, ...]
```

```python
frame.drop_columns(["score"])
print(frame.to_dicts())
```

**Output:**
```
[{'name': 'Alice', 'age': 30, 'city': 'NEW YORK'}, ...]
```

```python
frame.rename_column("name", "full_name")
print(frame.to_dicts())
```

**Output:**
```
[{'full_name': 'Alice', 'age': 30, 'city': 'NEW YORK'}, ...]
```

```python
groupby = ft.TinyGroupBy(frame, keys=["city"])
print(groupby.keys)
print(groupby.groups)
```

```python
count_frame = groupby.count(frame)
print(count_frame.to_dicts())
```

**Output:**
```
[{'city': 'NEW YORK', 'count': 2}, {'city': 'PARIS', 'count': 1}]
```

---

## ⚙️ Supported Types

| Type      | Column variant  |
|------------|---------------|
| int        | `Int`, `OptInt` |
| float      | `Float`, `OptFloat` |
| bool       | `Bool`, `OptBool` |
| str        | `Str`, `OptStr` |
| mixed      | `Mixed`, `OptMixed` |

---

## 🐉 Why "feathertail"?

In *Fourth Wing*, "feathertail" refers to a juvenile stage of dragons — smaller, golden, and known for their feathery tails rather than weaponized ones. Feathertail dragons, like Andarna, are characterized by gentleness, non-violence, and an elegant simplicity.

This library embodies those same principles: lightweight, non-destructive, and focused on providing clean, powerful tools for data transformation without heavy dependencies or unnecessary complexity.

---

## ❤️ Contributing

Contributions and ideas are always welcome! Open an issue or a pull request.

---

## 📄 License

MIT
