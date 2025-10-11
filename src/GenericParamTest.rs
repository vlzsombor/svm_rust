struct RepositoryExample<T>{
    _marker: std::marker::PhantomData<T>
}

impl<T> RepositoryExample<T> {
    fn new() -> Self {
        RepositoryExample {
            _marker: std::marker::PhantomData,
        }
    }
    fn get_model(&self, id: i32) -> T {
        todo!()
    }

    fn update_model(&self, model: T) {
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repository() {
        let repo :RepositoryExample<String> = RepositoryExample::new();
        // let res= repo.get_model(33);
        repo.update_model("fassdfa".parse().unwrap());
        // get_model kann nicht getestet werden wegen todo!()
        // let model = repo.get_model(1);
        // update_model kann nicht getestet werden wegen todo!()
        // repo.update_model("test".to_string());
    }
}
