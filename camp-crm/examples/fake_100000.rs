use anyhow::Result;
use camp_crm::model::UserStat;

#[tokio::main]
async fn main() -> Result<()> {
    #[cfg(feature = "local_utils")]
    {
        let pool = sqlx::PgPool::connect("postgres://postgres:postgres@localhost/crm").await?;
        UserStat::gen_and_insert(100000, pool).await?
    }

    Ok(())
}
