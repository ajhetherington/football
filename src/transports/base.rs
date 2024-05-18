use crate::{agent::AgentAction, gamestate::GameState};

pub trait Transport {
    async fn get_action(&self, state: &GameState) -> Vec<AgentAction> {
        todo!()
    }
}