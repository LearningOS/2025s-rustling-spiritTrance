/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {  
        // 增加元素数量  
        self.count += 1;  
        
        // 如果向量长度不够，添加默认值扩展它  
        if self.count >= self.items.len() {  
            self.items.push(T::default());  
        }  
        
        // 将值放在数组末尾  
        self.items[self.count] = value;  
        
        // 向上调整堆（上浮）  
        let mut idx = self.count;  
        
        // 当前节点不是根节点，且与父节点比较满足条件（如小于父节点，对于最小堆）  
        while idx > 1 && (self.comparator)(&self.items[idx], &self.items[self.parent_idx(idx)]) {  
            // 交换当前节点与父节点  
            let parent = self.parent_idx(idx);  
            self.items.swap(idx, parent); 
            // 继续向上调整  
            idx = self.parent_idx(idx);  
        }  
    }  

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {  
        // 如果没有子节点，返回节点自己的索引  
        if !self.children_present(idx) {  
            return idx;  
        }  
        
        let left_idx = self.left_child_idx(idx);  
        let right_idx = self.right_child_idx(idx);  
        
        // 如果只有左子节点，返回左子节点索引  
        if right_idx > self.count {  
            return left_idx;  
        }  
        
        // 否则比较两个子节点，返回满足比较条件的那个的索引  
        if (self.comparator)(&self.items[left_idx], &self.items[right_idx]) {  
            left_idx  
        } else {  
            right_idx  
        }  
    }  
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;
    fn next(&mut self) -> Option<T> {  
        if self.is_empty() {  
            return None;  
        }  
        
        // 保存根节点的值作为返回值  
        let mut result = T::default();  
        std::mem::swap(&mut result, &mut self.items[1]);  
        
        // 将最后一个元素移到根节点位置  
        if self.count > 1 {  
            // 使用swap来避免移动  
            self.items.swap(1, self.count);  
        }  
        
        // 减少堆大小  
        self.count -= 1;  
        
        // 如果堆不为空，进行向下调整（下沉）  
        if !self.is_empty() {  
            let mut idx = 1;  
            
            // 向下调整，直到叶子节点或者满足堆的性质  
            while self.children_present(idx) {  
                let child_idx = self.smallest_child_idx(idx);  
                
                // 如果当前节点不满足比较条件，与子节点交换  
                if !(self.comparator)(&self.items[idx], &self.items[child_idx]) {  
                    self.items.swap(idx, child_idx);  
                    idx = child_idx;  
                } else {  
                    // 否则已经满足堆的性质，停止调整  
                    break;  
                }  
            }  
        }  
        
        // 返回之前保存的根节点值  
        Some(result)  
    }  
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}