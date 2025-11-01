use std::sync::OnceLock;
use lib_utils::envs::get_env;

pub fn oss_config() -> &'static OssConfig {
    static INSTANCE: OnceLock<OssConfig> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        OssConfig::load_from_env().unwrap_or_else(|ex| {
            panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}")
        })
    })
}

#[allow(non_snake_case)]
#[derive(Clone)]
pub struct OssConfig {
    pub OSS_ACCESS_KEY_ID: String,
    pub OSS_ACCESS_KEY_SECRET: String,
    pub OSS_BUCKET_NAME: String,
    pub OSS_ENDPOINT: String,
    pub OSS_REGION: String,
    pub OSS_PUBLIC_BASE: String,
}

impl OssConfig {
    fn load_from_env() -> lib_utils::envs::Result<OssConfig> {
        Ok(OssConfig {
            OSS_ACCESS_KEY_ID: get_env("OSS_ACCESS_KEY_ID")?,
            OSS_ACCESS_KEY_SECRET: get_env("OSS_ACCESS_KEY_SECRET")?,
            OSS_BUCKET_NAME: get_env("OSS_BUCKET_NAME")?,
            OSS_REGION: get_env("OSS_REGION")?,
            OSS_ENDPOINT: get_env("OSS_ENDPOINT")?,
            OSS_PUBLIC_BASE: get_env("OSS_PUBLIC_BASE")?,
        })
    }
}