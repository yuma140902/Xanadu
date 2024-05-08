use super::Component;

pub struct ComponentArray<T> {
    data: Vec<Option<T>>,
}

impl<T: Component> ComponentArray<T> {
    pub const fn new() -> Self {
        Self { data: Vec::new() }
    }

    /// 指定されたインデックスにコンポーネントを追加する
    ///
    /// ## Returns
    ///
    /// 以前のコンポーネントがあればそれを返す。なければNoneを返す
    pub fn replace(&mut self, index: usize, component: T) -> Option<T> {
        if index >= self.data.len() {
            self.data.resize_with(index + 1, || None);
        }
        let old = self.data[index];
        self.data[index] = Some(component);
        old
    }
}

impl<T: Component> Default for ComponentArray<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn replace() {
        let mut array = ComponentArray::new();
        array.replace(0, 42);
        assert_eq!(array.data.len(), 1);
        assert_eq!(array.data[0], Some(42));
    }

    #[test]
    fn replace_offset() {
        let mut array = ComponentArray::new();
        array.replace(0, 42);
        array.replace(2, 43);
        assert_eq!(array.data.len(), 3);
        assert_eq!(array.data[0], Some(42));
        assert_eq!(array.data[1], None);
        assert_eq!(array.data[2], Some(43));
    }

    #[test]
    fn replace_return_value() {
        let mut array = ComponentArray::new();
        assert_eq!(array.replace(0, 42), None);
        assert_eq!(array.replace(0, 43), Some(42));
    }
}