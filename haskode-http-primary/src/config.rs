#[derive(clap::Parser)]
pub struct Config {
    #[clap(long, env = "GITHUB_CLIENT_ID")]
    pub github_client_id: String,

    #[clap(long, env = "GITHUB_CLIENT_SECRET")]
    pub github_client_secret: String,

    #[clap(long, env = "GITHUB_CALLBACK_URL")]
    pub github_callback_url: String,

    #[clap(long, env = "DATABASE_URL")]
    pub database_url: String,
}
