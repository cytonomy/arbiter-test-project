use arbiter_core::{
    data_collection::EventLogger,
    environment::builder::{BlockSettings, EnvironmentBuilder},
};
use crate::agents::{block_admin::BlockAdmin, factory_deployer::FactoryDeployer};

use super::*;


pub async fn setup(config: SimulationConfig<Fixed>) -> Result<Simulation> {
    let environment = EnvironmentBuilder::new()
        .block_settings(BlockSettings::UserControlled)
        .build();

    let block_admin = BlockAdmin::new(&environment, &config).await?;
    let factory_deployer = FactoryDeployer::new(&environment).await?;

    EventLogger::builder()
        .directory(config.output_directory)
        .file_name(config.output_file_name.unwrap())
        .add(factory_deployer.factory.events(), "factory")
        .run()?;

    Ok(Simulation {
        agents: Agents::new().add(block_admin).add(factory_deployer),
        steps: config.trajectory.num_steps,
        environment,
    })
}