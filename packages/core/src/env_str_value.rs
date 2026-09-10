use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    ops::Deref,
    sync::LazyLock,
};

use regex::Regex;

use crate::error::{Error, Result};

// FromEnvStrValue

pub trait FromEnvStrValue: Sized {
    fn from_env_str(s: EnvStrValue) -> Result<Self>;

    fn conversion_error(s: EnvStrValue) -> Error {
        Error::ValConversionError(format!(
            "Failed to convert into {}: {:?}",
            std::any::type_name::<Self>(),
            s
        ))
    }
}

impl<T> FromEnvStrValue for Vec<T>
where
    T: FromEnvStrValue,
{
    fn from_env_str(s: EnvStrValue) -> Result<Self> {
        if let Some(c) = RE_ARRAY.captures(&s)
            && let (_, [c]) = c.extract()
        {
            Ok(RE_ELEM
                .captures_iter(c)
                .flat_map(|elem| {
                    if let Some(elem) = elem.get(1) {
                        Some(T::from_env_str(EnvStrValue::new(&s.name, elem.as_str())))
                    } else {
                        None
                    }
                })
                .collect::<Result<Vec<T>>>()?)
        } else {
            Err(Self::conversion_error(s))
        }
    }
}

impl<T> FromEnvStrValue for HashSet<T>
where
    T: FromEnvStrValue + Eq + std::hash::Hash,
{
    fn from_env_str(s: EnvStrValue) -> Result<Self> {
        let v: Vec<_> = Vec::<T>::from_env_str(s)?;
        Ok(HashSet::from_iter(v.into_iter()))
    }
}

impl<K, V> FromEnvStrValue for HashMap<K, V>
where
    K: FromEnvStrValue + Eq + std::hash::Hash,
    V: FromEnvStrValue,
{
    fn from_env_str(s: EnvStrValue) -> Result<Self> {
        if let Some(c) = RE_HASHMAP.captures(&s)
            && let (_, [c]) = c.extract()
        {
            let v = RE_ENTRY
                .captures_iter(c)
                .map(|c| -> Result<Option<(K, V)>> {
                    if let Some(k) = c.get(1)
                        && let Some(v) = c.get(2)
                    {
                        Ok(Some((
                            K::from_env_str(EnvStrValue::new(&s.name, &k.as_str()))?,
                            V::from_env_str(EnvStrValue::new(&s.name, &v.as_str()))?,
                        )))
                    } else {
                        Ok(None)
                    }
                })
                .collect::<Result<Vec<Option<(K, V)>>>>()?;
            Ok(v.into_iter().flatten().collect())
        } else {
            Err(Self::conversion_error(s))
        }
    }
}

macro_rules! def_from_env_str_value_for_tuple {
    ($($T:ident),*) => {
        impl<$($T),*> FromEnvStrValue for ($($T,)+)
        where
            $($T: FromEnvStrValue,)+
        {
            fn from_env_str(s: EnvStrValue) -> Result<Self> {
                if let Some(c) = RE_TUPLE.captures(&s)
                    && let (_, [c]) = c.extract()
                {
                    let mut elems = RE_ELEM
                        .captures_iter(c)
                        .flat_map(|elem| {
                            if let Some(elem) = elem.get(1) {
                                Some(elem.as_str())
                            } else {
                                None
                            }
                        });

                    Ok((
                        $($T::from_env_str(
                            EnvStrValue::new(
                                &s.name,
                                if let Some(item) = elems.next() {
                                    item
                                } else {
                                    return Err(Self::conversion_error(s));
                                }
                            )
                        )?,)+
                    ))
                } else {
                    Err(Self::conversion_error(s))
                }
            }
        }
    };
}

def_from_env_str_value_for_tuple!(T1, T2);
def_from_env_str_value_for_tuple!(T1, T2, T3);
def_from_env_str_value_for_tuple!(T1, T2, T3, T4);
def_from_env_str_value_for_tuple!(T1, T2, T3, T4, T5);
def_from_env_str_value_for_tuple!(T1, T2, T3, T4, T5, T6);
def_from_env_str_value_for_tuple!(T1, T2, T3, T4, T5, T6, T7);
def_from_env_str_value_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8);
def_from_env_str_value_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9);

// EnvStrValue

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnvStrValue {
    name: &'static str,
    value: String,
}

impl Display for EnvStrValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Deref for EnvStrValue {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl EnvStrValue {
    pub fn new<S: ToString>(name: &'static str, s: S) -> Self {
        fn normalize(s: String) -> String {
            if let Some(c) = RE_STR.captures(&s)
                && let (_, [s]) = c.extract()
            {
                // "..." で囲まれた文字列はそのまま返す
                s.to_string()
            } else {
                s.trim().to_string()
            }
        }

        Self {
            name,
            value: normalize(s.to_string()),
        }
    }

    pub fn convert<T: FromEnvStrValue>(self) -> Result<T> {
        T::from_env_str(self)
    }
}

impl FromEnvStrValue for String {
    fn from_env_str(s: EnvStrValue) -> Result<Self> {
        Ok(s.value)
    }
}

macro_rules! def_primitive_converter {
    ($S: ty) => {
        impl FromEnvStrValue for $S {
            fn from_env_str(s: EnvStrValue) -> Result<Self> {
                s.parse::<$S>().map_err(|_| Self::conversion_error(s))
            }
        }
    };
}

def_primitive_converter!(bool);
def_primitive_converter!(u128);
def_primitive_converter!(u64);
def_primitive_converter!(u32);
def_primitive_converter!(u16);
def_primitive_converter!(u8);
def_primitive_converter!(usize);
def_primitive_converter!(i128);
def_primitive_converter!(i64);
def_primitive_converter!(i32);
def_primitive_converter!(i16);
def_primitive_converter!(i8);
def_primitive_converter!(isize);
def_primitive_converter!(f64);
def_primitive_converter!(f32);

static RE_ARRAY: LazyLock<Regex> = LazyLock::new(|| {
    regex::Regex::new(r"(?s)^\s*\[(.*)\]\s*$").expect("Failed to create regex for Array parsing")
});
static RE_ELEM: LazyLock<Regex> = LazyLock::new(|| {
    regex::Regex::new(r#"(?s)\s*("[^"]*"|[^\,\s]+)\s*,?"#)
        .expect("Failed to create regex for Element parsing")
});
static RE_STR: LazyLock<Regex> = LazyLock::new(|| {
    regex::Regex::new(r#"(?s)^\s*"(.*)"\s*"#)
        .expect("Failed to create regex for string literal parsing")
});
static RE_HASHMAP: LazyLock<Regex> = LazyLock::new(|| {
    regex::Regex::new(r"(?s)^\s*\{(.*)\}\s*$").expect("Failed to create regex for HashMap parsing")
});
static RE_ENTRY: LazyLock<Regex> = LazyLock::new(|| {
    regex::Regex::new(r#"(?s)\s*("[^"]+"|[^\:]+)\s*\:\s*("[^"]*"|[^\,\s]+),?"#)
        .expect("Failed to create regex for Entry parsing")
});
static RE_TUPLE: LazyLock<Regex> = LazyLock::new(|| {
    regex::Regex::new(r"(?s)^\s*\((.*)\)\s*$").expect("Failed to create regex for Tuple parsing")
});

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_env_str_value() {
        assert_eq!(EnvStrValue::new("test", "test").to_string(), "test");
        assert_eq!(EnvStrValue::new("test", " test ").to_string(), "test");
        assert_eq!(
            EnvStrValue::new("test", r#" " test " "#).to_string(),
            " test "
        );
    }

    #[test]
    fn test_convert() {
        let r: String = EnvStrValue::new("test", "test").convert().unwrap();
        assert_eq!(r, "test".to_string());

        let r: usize = EnvStrValue::new("test", "1").convert().unwrap();
        assert_eq!(r, 1);

        let r: bool = EnvStrValue::new("test", "false").convert().unwrap();
        assert_eq!(r, false);
    }

    #[test]
    fn test_convert_error() {
        let r: Result<u32> = EnvStrValue::new("test", "test").convert();
        assert!(matches!(r.unwrap_err(), Error::ValConversionError(_)));
    }

    #[test]
    fn test_convert_str_to_array_like() {
        let r: Vec<String> = EnvStrValue::new("test", r#"["a","b","c"]"#)
            .convert()
            .unwrap();
        assert_eq!(r, vec!["a".to_string(), "b".to_string(), "c".to_string()]);

        let r: Vec<u32> = EnvStrValue::new("test", r#"[1, 2, 3]"#).convert().unwrap();
        assert_eq!(r, vec![1, 2, 3]);

        let r: HashSet<u32> = EnvStrValue::new("test", r#"[1, 2, 3]"#).convert().unwrap();
        assert_eq!(r, HashSet::from([1, 2, 3]));

        let r: HashSet<String> = EnvStrValue::new("test", r#"[ a , b , c ]"#)
            .convert()
            .unwrap();
        assert_eq!(
            r,
            HashSet::from(["a".to_string(), "b".to_string(), "c".to_string()])
        );

        let r: Vec<String> = EnvStrValue::new("test", r#"["a,x","b,y", c, z]"#)
            .convert()
            .unwrap();
        assert_eq!(
            r,
            vec![
                "a,x".to_string(),
                "b,y".to_string(),
                "c".to_string(),
                "z".to_string()
            ]
        );
    }

    #[test]
    fn test_convert_str_to_hashmap() {
        let r: HashMap<String, String> = EnvStrValue::new(
            "",
            r#"
            { 
                "a": "1,A", 
                "b": "2,B", 
                "c": 3,
            }
        "#,
        )
        .convert()
        .unwrap();

        assert_eq!(
            r,
            HashMap::from([
                ("a".to_string(), "1,A".to_string()),
                ("b".to_string(), "2,B".to_string()),
                ("c".to_string(), "3".to_string()),
            ])
        );

        let r: HashMap<String, u32> = EnvStrValue::new(
            "",
            r#"
            { 
                a: 1, 
                b: 2, 
                c: 3,
            }
        "#,
        )
        .convert()
        .unwrap();

        assert_eq!(
            r,
            HashMap::from([
                ("a".to_string(), 1),
                ("b".to_string(), 2),
                ("c".to_string(), 3),
            ])
        );
    }

    #[test]
    fn test_convert_str_to_tuple() {
        let r: (String, u32) = EnvStrValue::new("test", r#"("a", 1)"#).convert().unwrap();
        assert_eq!(r, ("a".to_string(), 1));

        let r: (String, u32, bool) = EnvStrValue::new("test", r#"("a,x", 1, true)"#)
            .convert()
            .unwrap();
        assert_eq!(r, ("a,x".to_string(), 1, true));
    }
}
