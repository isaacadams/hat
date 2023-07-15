use crate::{
    error::HatError,
    hat_util::{Assert, RequestBuilder, Store, StoreUnion},
    query::Variable,
};

pub type HatTestOutput = (Box<dyn Assert>, Option<StoreUnion>);

pub trait HatTestBuilder {
    fn build<T: Store + RequestExecutor>(
        self,
        global: &T,
        buffer: &mut String,
    ) -> anyhow::Result<HatTestOutput>;
}

pub trait RequestExecutor {
    fn execute(&self, request: RequestBuilder) -> Result<ureq::Response, HatError>;
}

pub struct HatRunner {
    global: Vec<StoreUnion>,
    client: ureq::Agent,
}

impl RequestExecutor for HatRunner {
    fn execute(&self, request: RequestBuilder) -> Result<ureq::Response, HatError> {
        let (builder, endpoint, body) = request.split();
        let ureq_request = RequestBuilder::build(builder, endpoint, &self.client)
            .ok_or(HatError::RequestBuilder)?;

        if let Some(body) = body {
            ureq_request.send_string(&body)
        } else {
            ureq_request.call()
        }
        .map_err(|e| HatError::HttpResponse(e.to_string()))
    }
}

impl Store for HatRunner {
    fn fetch_value<'a>(&'a self, key: &'a str) -> Option<Variable<'a>> {
        self.global.iter().find_map(|s| s.fetch_value(key))
    }
}

impl HatRunner {
    pub fn new(global: StoreUnion, client: ureq::Agent) -> Self {
        Self {
            global: vec![global],
            client,
        }
    }

    pub fn test<R: HatTestBuilder, I: Iterator<Item = R>>(&mut self, tests: &mut I) -> bool {
        let mut result = String::new();
        let mut all_tests_pass = true;

        for r in tests {
            match r.build(self, &mut result) {
                Ok((test, outputs)) => {
                    all_tests_pass &= test.assert(&mut result);

                    if let Some(o) = outputs {
                        self.global.push(o);
                    }
                }
                Err(e) => {
                    result.push_str(crate::assertion::pretty_bool(false));
                    result.push_str(e.to_string().as_str());
                }
            };

            result.push_str("\n\n");
        }

        println!("{}", result.trim_end_matches('\n'));

        all_tests_pass
    }
}
