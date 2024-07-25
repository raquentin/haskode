#[derive(clap::Parser)]
pub struct Config {
    #[clap(long, env = "GITHUB_CLIENT_ID")]
    pub github_client_id: String,

    #[clap(long, env = "GITHUB_CLIENT_SECRET")]
    pub github_client_secret: String,

    #[clap(long, env = "GITHUB_CALLBACK_URL")]
    pub github_callback_url: String,

    #[clap(long, env = "POSTGRES_URL")]
    pub postgres_url: String,

    #[clap(long, env = "POSTGRES_USER")]
    pub postgres_user: String,

    #[clap(long, env = "POSTGRES_PASSWORD")]
    pub postgres_password: String,
}
