# ❄️ tinytim_rs

A **tiny, fast, Rust-backed transformation core** for Python table data.  
Designed to replace or accelerate Python-based data logic, seamlessly integrates with your `tinytable` style APIs.

---

## 💥 Features

- 🚀 Super fast Rust backend
- ✅ Type-safe columns (`Int`, `Float`, `Str`, `Bool`, placeholders)
- 🔗 Join support
- 🔬 GroupBy with advanced aggregations: `count`, `sum`, `mean`, `min`, `max`, `median`, `std`
- 🧪 Fill missing values
- ✂️ Drop & rename columns
- 🛠️ Edit columns with Python functions
- 🔄 Convert to/from Python list-of-dicts

---

## 📦 Installation

```bash
pip install tinytim_rs
```

Or, if developing locally:

```bash
maturin develop
```

---

## 🧑‍💻 Basic usage

```python
from tinytim_rs import TinyFrame

data = {
    "id": [1, 2, 3],
    "name": ["alice", "bob", "carol"],
    "score": [100.0, None, 85.5],
}

frame = TinyFrame(data)

# Fill missing
frame.fillna({"score": 0})

# Edit column
frame.edit_column("name", lambda x: x.upper())

# Drop columns
frame.drop_columns(["id"])

# Rename column
frame.rename_column("score", "final_score")

print(frame.columns())
print(frame.to_dicts())
```

---

## 🔗 Joins

```python
left = TinyFrame({
    "id": [1, 2, 3],
    "val_l": [10, 20, 30],
})

right = TinyFrame({
    "id": [2, 3, 4],
    "val_r": [200, 300, 400],
})

joined = left.join(right, ["id"])
print(joined.to_dicts())
```

---

## 🔬 GroupBy

```python
data = {
    "group": ["A", "A", "B", "B", "A"],
    "value": [10, 20, 30, 40, 5],
}

frame = TinyFrame(data)
gb = frame.groupby(["group"])

print("Count:", gb.count().to_dicts())
print("Sum:", gb.sum().to_dicts())
print("Mean:", gb.mean().to_dicts())
print("Min:", gb.min().to_dicts())
print("Max:", gb.max().to_dicts())
print("Median:", gb.median().to_dicts())
print("Std:", gb.std().to_dicts())
```

---

## 🔄 From and to dicts

```python
records = [
    {"id": 1, "value": 10},
    {"id": 2, "value": None},
]

frame = TinyFrame.from_dicts(records)
print(frame.to_dicts())
```

---

## 💡 Placeholder columns

If you have columns with arbitrary Python objects, you can convert them to integer placeholder IDs for Rust operations, then rehydrate after.  
Ask for an example snippet if you'd like help!

---

## 🗺️ Roadmap

- More join types (`left`, `outer`)
- Optional Arrow-based speedups
- Parallel groupby

---

## ❤️ Contributing

Open issues or PRs welcome!  
Designed to stay **tiny**, so proposals should focus on core speed and simplicity.

---

## ⚖️ License

MIT