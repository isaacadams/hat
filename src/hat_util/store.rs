use std::{collections::HashMap, slice::Iter};

#[allow(dead_code)]
pub enum StoreUnion {
    MapStringToJsonValue(StoreMap),
    MapStringToBodyContent(HashMap<String, crate::query::BodyContent>),
}

impl Store for StoreUnion {
    fn fetch_value(&self, key: &str) -> Option<String> {
        let mut is_literal = true;
        let key = if key.starts_with(":") {
            is_literal = false;
            // remove the ':'
            &key[1..]
        } else {
            key
        };

        let value = match self {
            // key = r.headers.content-type
            StoreUnion::MapStringToJsonValue(s) => s.fetch_value(key).map(|v| v.to_string()),
            // key = headers | content-type
            StoreUnion::MapStringToBodyContent(s) => {
                let mut iter = key.split(r#"|"#);
                let key = iter.next()?.trim();
                let filter = iter.next().or(Some(""))?.trim();
                s.get(key)?.query(filter)
            }
        };

        if is_literal {
            value.map(|mut v| {
                v.insert(0, '"');
                v.push('"');
                v
            })
        } else {
            value
        }
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
    fn fetch_value(&self, key: &str) -> Option<String> {
        self.store_1
            .fetch_value(key)
            .or(self.store_2.fetch_value(key))
    }
}

pub type StoreMap = HashMap<String, serde_json::Value>;

use regex::{Captures, Regex};

use crate::query::Queryable;
const PATTERN: &str = r#"\{\{(.*?)}\}"#;
lazy_static::lazy_static! {
    static ref REGEX: Regex = Regex::new(PATTERN).expect("pattern is invalid");
}

pub trait Store {
    fn fetch_value(&self, key: &str) -> Option<String>;

    fn match_and_replace(&self, hydrate: &str) -> String {
        let result = REGEX.replace_all(hydrate, |cap: &Captures| {
            let key = &cap[1];
            if let Some(x) = self.fetch_value(key) {
                x
            } else {
                log::debug!("could not find {}", key);
                format!("{{{{{}}}}}", key)
            }
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

impl Store for StoreMap {
    fn fetch_value(&self, key: &str) -> Option<String> {
        self.get(key).map(|s| s.to_string())
    }
}

impl<T: Store> Store for Iter<'_, &T> {
    fn fetch_value(&self, key: &str) -> Option<String> {
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

    pub fn parse(value: &str) -> serde_json::Value {
        match serde_json::from_str(value) {
            Ok(v) => v,
            Err(_) => serde_json::to_value(value).unwrap_or(serde_json::Value::Null),
        }
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
        let mut map = StoreMap::new();
        map.insert("key".to_string(), self::parse("value"));
        map.insert("number".to_string(), self::parse("123"));
        map.insert("bool".to_string(), self::parse("false"));

        let store = StoreUnion::MapStringToJsonValue(map);

        let hydrated = store.match_and_replace("{{key}}");
        assert_eq!(hydrated, "\"value\"");

        let hydrated = store.match_and_replace("{{bool}} == {{number}}");
        assert_eq!(hydrated, "false == 123");
    }

    #[test]
    fn json_variable_replacement_works() {
        let mut map = StoreMap::new();
        map.insert("header.status".to_string(), self::parse("200"));
        map.insert(
            "r.headers.content-type".to_string(),
            self::parse("application/json"),
        );

        let store = StoreUnion::MapStringToJsonValue(map);

        let hydrated = store.match_and_replace("{{r.headers.content-type}}");
        assert_eq!(hydrated, "\"application/json\"");

        let hydrated = store.match_and_replace("{{header.status}} == 200");
        assert_eq!(hydrated, "200 == 200");
    }

    #[test]
    fn json_array_variable_replacement_works() {
        let mut map = StoreMap::new();

        map.insert("r.body.[0].title".to_string(), self::parse("hello world"));

        let store = StoreUnion::MapStringToJsonValue(map);

        let hydrated = store.match_and_replace("{{r.body.[0].title}} == \"hello world\"");
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
}
