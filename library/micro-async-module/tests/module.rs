#[cfg(test)]
mod test_module {
    use micro_async_module::{run_async_block_on, App, AsyncRuntime, Module};

    const MODULE_NAME_A: &str = "test_module_a";

    #[derive(Debug)]
    struct TestModuleA;

    impl Module for TestModuleA {
        fn name(&self) -> &str {
            MODULE_NAME_A
        }

        fn start(&self, runtime: AsyncRuntime) {
            run_async_block_on(
                async move {
                    println!("{}", MODULE_NAME_A);
                },
                runtime,
            );
        }
    }

    const MODULE_NAME_B: &str = "test_module_b";

    #[derive(Debug, Clone)]
    struct TestModuleB;

    impl Module for TestModuleB {
        fn name(&self) -> &str {
            MODULE_NAME_B
        }

        fn start(&self, runtime: AsyncRuntime) {
            run_async_block_on(
                async move {
                    println!("{}", MODULE_NAME_B);
                },
                runtime,
            );
        }
    }

    #[test]
    fn it_works() {
        let app = App::new("test".to_string(), 4, 100);
        app.add_module(TestModuleA).add_module(TestModuleB).run();
    }
}
