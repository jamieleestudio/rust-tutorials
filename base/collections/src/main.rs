// 集合
// 对标 Java：ArrayList / HashMap / HashSet / ArrayDeque
// Rust 标准库三大件：Vec、HashMap、VecDeque（还有 HashSet/BTreeMap 等）

use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    // ============ Vec：对标 ArrayList<T> ============
    let mut v: Vec<i32> = Vec::new(); // 空数组，需要类型标注
    v.push(1);
    v.push(2);
    v.push(3);
    // vec! 宏：对标 List.of(1, 2, 3)，但可变
    let v2 = vec![1, 2, 3];
    let zeros = vec![0; 3]; // 3 个 0，对标 new ArrayList<>(Collections.nCopies(3, 0))

    // 索引访问 vs 安全访问
    let third = v2[2]; // 越界直接 panic，对标 ArrayList.get 越界抛 IndexOutOfBoundsException
    let maybe = v2.get(10); // 越界返回 None，对标 Optional 包装的安全访问
    println!("v={v:?} v2={v2:?} zeros={zeros:?} third={third} get(10)={maybe:?}");

    // 弹出与遍历
    let popped = v.pop(); // Option<i32>，对标 Deque/栈操作
    println!("pop = {popped:?}，剩余 {v:?}");
    for x in &v {
        // &v：借用遍历（不拿走所有权），对标 for (int x : list)
        print!("{x} ");
    }
    println!();

    // ============ HashMap：对标 HashMap<K, V> ============
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("alice"), 90);
    scores.insert(String::from("bob"), 85);

    // get 返回 Option（对标 map.get 可能返回 null）
    let alice = scores.get("alice");
    println!("alice = {alice:?}");

    // entry API：对标 putIfAbsent / merge / computeIfAbsent
    let count = scores.entry("carol".to_string()).or_insert(0); // 不存在则插入默认值
    *count += 50; // entry 返回 &mut V，原地更新
    println!("carol = {:?}", scores.get("carol"));

    // 计数器惯用法：对标 map.merge(word, 1, Integer::sum)
    let text = "a b a c a b";
    let mut freq: HashMap<&str, i32> = HashMap::new();
    for word in text.split_whitespace() {
        *freq.entry(word).or_insert(0) += 1;
    }
    println!("词频 = {freq:?}");

    // 遍历（对标 entrySet）
    for (k, v) in &scores {
        println!("{k} -> {v}");
    }

    // ============ HashSet：对标 HashSet<T> ============
    let mut seen: HashSet<&str> = HashSet::new();
    seen.insert("a");
    seen.insert("a"); // 重复插入无事发生
    println!("seen = {seen:?}，contains a = {}", seen.contains("a"));

    // 集合运算（对标 retainAll/containsAll 手写逻辑）
    let set_a: HashSet<i32> = [1, 2, 3].into_iter().collect();
    let set_b: HashSet<i32> = [2, 3, 4].into_iter().collect();
    let intersection: HashSet<i32> = set_a.intersection(&set_b).copied().collect();
    println!("交集 = {intersection:?}");

    // ============ VecDeque：对标 ArrayDeque<T> ============
    let mut queue: VecDeque<&str> = VecDeque::new();
    queue.push_back("任务1"); // 尾进
    queue.push_back("任务2");
    let first = queue.pop_front(); // 头出
    println!("出队 = {first:?}，剩余 {queue:?}");

    // ============ 从集合转换 ============
    // collect()：对标 stream().collect(Collectors.toList())
    let squares: Vec<i32> = (1..=5).map(|x| x * x).collect();
    println!("平方 = {squares:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_like_arraylist() {
        let mut v = vec![1, 2, 3];
        v.push(4);
        assert_eq!(v.len(), 4);
        assert_eq!(v[0], 1);
        assert_eq!(v.get(100), None); // 安全访问
    }

    #[test]
    fn hashmap_entry_api() {
        let mut m: HashMap<String, i32> = HashMap::new();
        let c = m.entry("k".to_string()).or_insert(0);
        *c += 1;
        let c = m.entry("k".to_string()).or_insert(0);
        *c += 1;
        assert_eq!(m["k"], 2); // Index 语法也能访问 HashMap
    }

    #[test]
    fn word_frequency() {
        let mut freq: HashMap<&str, i32> = HashMap::new();
        for word in "a b a".split_whitespace() {
            *freq.entry(word).or_insert(0) += 1;
        }
        assert_eq!(freq["a"], 2);
        assert_eq!(freq["b"], 1);
    }

    #[test]
    fn set_operations() {
        let a: HashSet<i32> = [1, 2, 3].into_iter().collect();
        let b: HashSet<i32> = [2, 3, 4].into_iter().collect();
        let inter: HashSet<i32> = a.intersection(&b).copied().collect();
        let expected: HashSet<i32> = [2, 3].into_iter().collect();
        assert_eq!(inter, expected);
    }

    #[test]
    fn deque_fifo() {
        let mut q: VecDeque<i32> = VecDeque::new();
        q.push_back(1);
        q.push_back(2);
        assert_eq!(q.pop_front(), Some(1));
    }
}
