error_chain! {
    foreign_links {
        Http(::reqwest::Error);
        Env(::std::env::VarError);
        Url(::url::ParseError);
    }
}
