# collections

标准库集合：Vec、HashMap（entry API）、HashSet、VecDeque、collect 转换。

## Java 对照

| Rust | Java |
|---|---|
| `Vec<T>` | `ArrayList<T>` |
| `vec![1, 2, 3]` | `List.of(1, 2, 3)` |
| `v.get(i)` 返回 `Option` | `list.get(i)` 越界抛异常 |
| `HashMap<K, V>` | `HashMap<K, V>` |
| `m.entry(k).or_insert(v)` | `putIfAbsent` |
| `*m.entry(k).or_insert(0) += 1` | `m.merge(k, 1, Integer::sum)` |
| `HashSet<T>` + `intersection` | `HashSet<T>` + 手写集合运算 |
| `VecDeque<T>` | `ArrayDeque<T>` |
| `.collect()` | `stream().collect(Collectors.toList())` |

## 运行

```powershell
cargo run -p collections
cargo test -p collections
```
