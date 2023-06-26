use crate::hat_util::{Assert, RequestBuilder, RequestExecutor, Store, StoreUnion};

pub type HatTestOutput = (Box<dyn Assert>, Option<StoreUnion>);

pub trait HatTestBuilder {
    fn build<T: Store + RequestExecutor>(self, global: &T) -> anyhow::Result<HatTestOutput>;
}

pub struct HatRunner {
    global: Vec<StoreUnion>,
    client: reqwest::blocking::Client,
}

impl RequestExecutor for HatRunner {
    fn execute(
        &self,
        request: RequestBuilder,
    ) -> Result<crate::hat_util::HttpResponse, crate::hat_util::HttpError> {
        request.build(&self.client).send()
    }
}

impl Store for HatRunner {
    fn fetch_value(&self, key: &str) -> Option<&serde_json::Value> {
        self.global.iter().find_map(|s| s.fetch_value(key))
    }
}

impl HatRunner {
    pub fn new(global: StoreUnion, client: reqwest::blocking::Client) -> Self {
        Self {
            global: vec![global],
            client,
        }
    }

    pub fn test<R: HatTestBuilder, I: Iterator<Item = R>>(&mut self, tests: &mut I) -> bool {
        let mut result = String::new();
        let mut all_tests_pass = true;

        for r in tests {
            match r.build(self) {
                Ok((test, outputs)) => {
                    all_tests_pass &= test.assert(&mut result);

                    if let Some(o) = outputs {
                        self.global.push(o);
                    }
                }
                Err(e) => {
                    result.push_str(crate::assertion::pretty_bool(false));
                    result.push_str(e.to_string().as_str());
                    result.push_str("\n\n");
                }
            };
        }

        println!("{}", result);

        all_tests_pass
    }
}
