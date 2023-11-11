use crate::v3bindings::uniswap_v3_factory::UniswapV3Factory as Factory;
use super::*;

#[derive(Clone)]
pub struct FactoryDeployer {
    pub client: Arc<RevmMiddleware>,
    pub factory: Factory<RevmMiddleware>,
}

impl FactoryDeployer {
    pub async fn new(environment: &Environment) -> Result<Self> {
        let client = RevmMiddleware::new(environment, "factory_deployer".into())?;
        let factory = Factory::deploy(client.clone(), ())?.send().await?;
        Ok(Self { client, factory })
    }
}

#[async_trait::async_trait]
impl Agent for FactoryDeployer {
    async fn step(&mut self) -> Result<()> {
        Ok(())
    }
}