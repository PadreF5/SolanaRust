use dotenv::dotenv;
use log::{error, info};
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::{env, str::FromStr};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let rpc_url = env::var("SOLANA_RPC_URL").expect("SOLANA_RPC_URL must be set");
    let raydium_pool = env::var("RAYDIUM_POOL").expect("RAYDIUM_POOL must be set");
    let orca_pool = env::var("ORCA_POOL").expect("ORCA_POOL must be set");
    let threshold: f64 = env::var("ARBITRAGE_THRESHOLD")
        .expect("ARBITRAGE_THRESHOLD must be set")
        .parse()
        .expect("ARBITRAGE_THRESHOLD must be a number");

    let client = RpcClient::new(rpc_url);

    loop {
        match fetch_prices(&client, &raydium_pool, &orca_pool).await {
            Ok((raydium_price, orca_price)) => {
                info!(
                    "Raydium Price: {:.2}, Orca Price: {:.2}",
                    raydium_price, orca_price
                );

                if (raydium_price - orca_price).abs() > threshold {
                    info!("Arbitrage opportunity found!");
                    if let Err(e) = execute_arbitrage().await {
                        error!("Failed to execute arbitrage: {}", e);
                    }
                }
            }
            Err(e) => {
                error!("Error fetching prices: {}", e);
            }
        }

        sleep(Duration::from_secs(10)).await; // Проверка каждые 10 секунд
    }
}

async fn fetch_prices(
    client: &RpcClient,
    raydium_pool: &str,
    orca_pool: &str,
) -> Result<(f64, f64), Box<dyn std::error::Error>> {
    let raydium_price = get_pool_price(client, raydium_pool).await?;
    let orca_price = get_pool_price(client, orca_pool).await?;
    Ok((raydium_price, orca_price))
}

async fn get_pool_price(
    client: &RpcClient,
    pool_address: &str,
) -> Result<f64, Box<dyn std::error::Error>> {
    let pubkey = Pubkey::from_str(pool_address)?;
    let account_data = client.get_account_data(&pubkey)?;

    // Заглушка для парсинга данных из аккаунта пула
    // В реальном проекте нужно парсить данные в зависимости от формата Raydium или Orca
    Ok(1.0) // Возвращаем фиктивную цену 1.0
}

async fn execute_arbitrage() -> Result<(), Box<dyn std::error::Error>> {
    // Здесь должна быть логика отправки транзакции на покупку/продажу
    info!("Executing arbitrage transaction...");
    Ok(())
}
