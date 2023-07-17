use crate::query::{Content, Variable};
use std::{collections::HashMap, slice::Iter};

pub type ContentMap = HashMap<String, Content>;

#[allow(dead_code)]
pub enum StoreUnion {
    MapStringToContent(ContentMap),
    Env,
}

impl Store for StoreUnion {
    fn fetch_value<'a>(&'a self, key: &'a str) -> Option<Variable<'_>> {
        let value = match self {
            // key = headers | content-type
            StoreUnion::MapStringToContent(s) => {
                let mut iter = key.split('|');
                let key = iter.next()?.trim();

                let content = s.get(key)?;

                if let Some(filter) = iter.next() {
                    content.query(filter.trim())
                } else {
                    content.value()
                }
            }
            StoreUnion::Env => dotenvy::var(key).ok().map(|v| Variable::Text(v)),
        };

        value
    }
}

pub struct StoreComposed<'a, 'b, A: Store, B: Store> {
    store_1: &'a A,
    store_2: &'b B,
}

impl<'a, 'b, A: Store, B: Store> StoreComposed<'a, 'b, A, B> {
    pub fn new(store_1: &'a A, store_2: &'b B) -> Self {
        Self { store_1, store_2 }
    }
}

impl<'a, 'b, A: Store, B: Store> Store for StoreComposed<'a, 'b, A, B> {
    fn fetch_value<'c>(&'c self, key: &'c str) -> Option<Variable<'c>> {
        self.store_1
            .fetch_value(key)
            .or(self.store_2.fetch_value(key))
    }
}

use regex::{Captures, Regex};

const PATTERN: &str = r"\{\{(.*?)}\}";
lazy_static::lazy_static! {
    static ref REGEX: Regex = Regex::new(PATTERN).expect("pattern is invalid");
}

pub trait Store {
    fn fetch_value<'a>(&'a self, key: &'a str) -> Option<Variable<'a>>;

    fn match_and_replace<F: Fn(Variable) -> String>(&self, hydrate: &str, render: F) -> String {
        let result = REGEX.replace_all(hydrate, |cap: &Captures| {
            let key = &cap[1];
            if let Some(x) = self.fetch_value(key) {
                return render(x);
            }

            log::debug!("could not find {}, captures: {:#?}", key, &cap);
            format!("{{{{{}}}}}", key)
        });

        result.into_owned()
    }

    fn compose<'a, 'b, B: Store>(&'a self, store: &'b B) -> StoreComposed<'a, 'b, Self, B>
    where
        Self: Sized,
    {
        StoreComposed::new(self, store)
    }
}

impl<T: Store> Store for Iter<'_, &T> {
    fn fetch_value<'a>(&'a self, key: &'a str) -> Option<Variable<'a>> {
        for s in self.as_ref() {
            let value = s.fetch_value(key);
            if value.is_some() {
                return value;
            }
        }

        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    pub fn parse(value: &str) -> Content {
        Content::new(value.to_string())
    }

    /* #[test]
    fn variable_store_works() {
        let request = Endpoint::new("https://jsonplaceholder.typicode.com", "get", None)
            .unwrap()
            .build();
        let response = reqwest::blocking::Client::new().execute(request).unwrap();
        let config = Config::read().unwrap();
        let store_env: VariableStore = config.environment.into();
        let store_response: VariableStore = VariableStore::from_response(&response).into();

        let store = vec![store_env, store_response];
        //store.print();

        assert!(VariableStore::equal(&store, "r.status", "200"));
        assert!(VariableStore::equal(
            &store,
            "r.headers.content-type",
            "\"text/html; charset=UTF-8\""
        ));
        assert!(VariableStore::equal(
            &store,
            "r.headers.access-control-allow-credentials",
            "true"
        ));
    } */

    #[test]
    fn variable_replacement_works() {
        let mut map = ContentMap::new();
        map.insert("key".to_string(), self::parse("value"));
        map.insert("number".to_string(), self::parse("123"));
        map.insert("bool".to_string(), self::parse("false"));

        let store = StoreUnion::MapStringToContent(map);

        let hydrated = store.match_and_replace("{{key}}", |v| v.as_literal().to_string());
        assert_eq!(hydrated, "\"value\"");

        let hydrated =
            store.match_and_replace("{{bool}} == {{number}}", |v| v.as_value().to_string());
        assert_eq!(hydrated, "false == 123");
    }

    #[test]
    fn json_variable_replacement_works() {
        let mut map = ContentMap::new();
        map.insert("header.status".to_string(), self::parse("200"));
        map.insert(
            "r.headers.content-type".to_string(),
            self::parse("application/json"),
        );

        let store = StoreUnion::MapStringToContent(map);

        let hydrated =
            store.match_and_replace("{{r.headers.content-type}}", |v| v.as_literal().to_string());
        assert_eq!(hydrated, "\"application/json\"");

        let hydrated =
            store.match_and_replace("{{header.status}} == 200", |v| v.as_value().to_string());
        assert_eq!(hydrated, "200 == 200");
    }

    #[test]
    fn json_array_variable_replacement_works() {
        let mut map = ContentMap::new();

        map.insert("r.body.[0].title".to_string(), self::parse("hello world"));

        let store = StoreUnion::MapStringToContent(map);

        let hydrated = store.match_and_replace("{{r.body.[0].title}} == \"hello world\"", |v| {
            v.as_literal().to_string()
        });
        assert_eq!(hydrated, "\"hello world\" == \"hello world\"");
    }

    #[test]
    fn key_split() {
        let key = "headers | content-type";
        let mut iter = key.split(r#"|"#);
        let key = iter.next().unwrap().trim();
        let filter = iter.next().unwrap().trim();

        assert_eq!(key, "headers");
        assert_eq!(filter, "content-type");
    }

    #[test]
    fn check_multiline() {
        let multiline = r#"I am
        multiline"#;
        assert_eq!(multiline.lines().count(), 2);
    }
}
