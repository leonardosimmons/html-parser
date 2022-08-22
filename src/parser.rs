pub struct Parser<T> {
    data: T,
}

impl<T> Parser<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }

    pub fn data(&self) -> &T {
        &self.data
    }

    pub fn get_data(self) -> T {
        self.data
    }
}
