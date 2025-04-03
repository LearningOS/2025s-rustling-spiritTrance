/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/

#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct myStack<T> {  
    q1: Queue<T>,  
    q2: Queue<T>  
}  

impl<T> myStack<T> {  
    pub fn new() -> Self {  
        Self {  
            q1: Queue::<T>::new(),  
            q2: Queue::<T>::new()  
        }  
    }  
    
    pub fn push(&mut self, elem: T) {  
        // 直接将元素添加到主队列中  
        self.q1.enqueue(elem);  
    }  
    
    pub fn pop(&mut self) -> Result<T, &'static str> {  
        // 检查栈是否为空  
        if self.is_empty() {  
            return Err("Stack is empty");  
        }  
        
        // 将q1中除最后一个元素外的所有元素移动到q2  
        let q1_size = self.q1.size();  
        for _ in 0..q1_size - 1 {  
            // 这里可以使用unwrap因为我们已经检查了非空  
            let elem = self.q1.dequeue().unwrap();   
            self.q2.enqueue(elem);  
        }  
        
        // 弹出q1中最后一个元素并直接unwrap  
        // 这里可以使用unwrap因为我们知道队列中至少有一个元素  
        let result = self.q1.dequeue().unwrap_or_else(|_| panic!("Unexpected empty queue"));  
        
        // 交换q1和q2  
        std::mem::swap(&mut self.q1, &mut self.q2);  
        
        Ok(result)  
    }  
    
    pub fn is_empty(&self) -> bool {  
        self.q1.is_empty() && self.q2.is_empty()  
    }  
}  

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_queue(){
		let mut s = myStack::<i32>::new();
		assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
	}
}