pub async fn get_database_url_for_environment() -> anyhow::Result<String> {
    if let Ok(mode) = std::env::var("LAMBDA_FUNCTION_MODE") {
        match mode.as_str() {
            "watch" => {
                use dotenv::dotenv;
                dotenv().ok();
                let url = std::env::var("DATABASE_URL")?;
                return Ok(url);
            }
            _ => {
                let region = "ap-northeast-1";
                let secret_name = "SecretsManager";
                let input = Some((region, secret_name));
                use establish_aws_mysql_sqlx::construct_url_for_aws;
                let url = construct_url_for_aws(input).await.or_else(|_| {
                    anyhow::bail!(
                        "The appropriate IAM role may not have been set for the Lambda function."
                    )
                })?;
                return Ok(url);
            }
        }
    }
    unreachable!()
}
