pub mod device_profile;
pub mod device_shadow;

pub trait StorageHandler {
    fn on_add(&self);

    fn on_update(&self);

    fn on_delete(&self);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
