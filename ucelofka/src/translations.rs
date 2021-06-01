use anyhow::{anyhow, Result};
use fluent_bundle::{bundle::FluentBundle, FluentArgs, FluentResource};
use fluent_langneg::{negotiate_languages, NegotiationStrategy};
use include_dir::{include_dir, Dir, DirEntry};
use intl_memoizer::concurrent::IntlLangMemoizer;
use lazy_static::lazy_static;
use std::{
    env,
    str::{from_utf8, FromStr},
};
use unic_langid::{langid, LanguageIdentifier};

const DEFAULT_LANG_STR: &str = "en-US";
const DEFAULT_LANG: LanguageIdentifier = langid!("en-US");

static RESOURCES: Dir = include_dir!("resources/");

fn get_available_locales() -> Result<Vec<LanguageIdentifier>> {
    RESOURCES
        .find("*")
        .map_err(|err| anyhow!("{}", err))?
        .filter_map(|e| {
            if let DirEntry::Dir(dir) = e {
                Some(
                    LanguageIdentifier::from_str(dir.path().file_name()?.to_str()?)
                        .map_err(|err| anyhow!("{}", err)),
                )
            } else {
                None
            }
        })
        .collect()
}

fn read_language_data(id: &LanguageIdentifier) -> Result<String> {
    let file = RESOURCES
        .get_file(format!("{}/ucelofka.ftl", id.to_string()))
        .ok_or_else(|| anyhow!("{} translation not found",))?;
    Ok(from_utf8(file.contents)?.to_string())
}

fn detect_language() -> Result<LanguageIdentifier> {
    let lang_str = env::var("LANG")
        .unwrap_or_else(|_| DEFAULT_LANG_STR.to_string())
        .replace(".utf8", "")
        .replace(".UTF8", "")
        .replace(".utf-8", "")
        .replace(".UTF-8", "");
    let lang = LanguageIdentifier::from_str(&lang_str)?;
    Ok(lang)
}

fn get_bundle() -> Result<FluentBundle<FluentResource, IntlLangMemoizer>> {
    let available = get_available_locales()?;
    let mut requested = vec![DEFAULT_LANG];

    if let Ok(langid) = detect_language() {
        requested.push(langid);
    }

    let resolved = negotiate_languages(
        &requested,
        &available,
        Some(&DEFAULT_LANG),
        NegotiationStrategy::Filtering,
    )
    .drain(..)
    .map(Clone::clone)
    .collect::<Vec<LanguageIdentifier>>();

    let mut bundle = FluentBundle::new_concurrent(resolved.clone());
    for lang in resolved {
        let data = read_language_data(&lang)?;
        let resource = FluentResource::try_new(data)
            .map_err(|_| anyhow!("Failed to parse flt file for language {}", lang))?;
        bundle.add_resource_overriding(resource);
    }

    Ok(bundle)
}

pub fn get_message(msgid: &str, args: Option<FluentArgs>) -> String {
    let mut errors = vec![];
    let msg = BUNDLE
        .get_message(msgid)
        .unwrap_or_else(|| panic!("Message `{}` was not found.", msgid));
    let pattern = msg
        .value()
        .unwrap_or_else(|| panic!("Message `{}` has no value.", msgid));
    BUNDLE
        .format_pattern(pattern, args.as_ref(), &mut errors)
        .into()
}

lazy_static! {
    static ref BUNDLE: FluentBundle<FluentResource, IntlLangMemoizer> = {
        match get_bundle() {
            Err(err) => panic!("failed to load translations: {}", err),
            Ok(bundle) => bundle,
        }
    };
}

pub mod texts {
    use super::get_message;
    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref DATA_DIRECTORY_PATH: String = get_message("data-directory-path", None);
    }
}
