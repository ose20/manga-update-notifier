#[macro_export]
macro_rules! impl_try_init {
    () => {
        pub async fn try_init(app_registry: &AppRegistry) -> Result<Self> {
            let short_title = SHORT_TITLE.to_string();
            let (title, _, url) = extract_manga_info(app_registry, &short_title).await?;
            Ok(Self {
                title,
                short_title,
                url,
            })
        }
    };
}

// MnagaInfo trait の実装マクロ
#[macro_export]
macro_rules! impl_manga_info {
    ($struct_name:ident) => {
        impl MangaInfo for $struct_name {
            fn short_title(&self) -> &str {
                &self.short_title
            }

            fn title(&self) -> &str {
                &self.title
            }

            fn url(&self) -> &str {
                &self.url
            }
        }
    };
}
