use tokio_modbus::server::Service;

struct EgccriModbusServer;

impl Service for EgccriModbusServer {
    type Request = ();
    type Response = ();
    type Error = ();
    type Future = ();

    fn call(&self, req: Self::Request) -> Self::Future {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
