pub struct Buffer {
    pub data: Vec<u8>,
}

impl Buffer {
    pub fn add_data(&mut self, data: u8) {
        self.data.push(data);
    }
    pub fn clear(self) {
        // self if dropped automatically
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_q8() {
        let mut buf = Buffer {
            data: vec![1, 2, 3],
        };
        buf.add_data(4);
        assert_eq!(buf.data.len(), 4);
        buf.clear();
        // buf.add_data(5); // This should fail to compile if uncommented
    }
}
