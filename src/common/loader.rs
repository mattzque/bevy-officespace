use anyhow::Result;
use bevy::asset::{RecursiveDependencyLoadState, UntypedAssetId};
use bevy::log::info;
use bevy::prelude::*;
use bevy::utils::HashSet;

#[derive(Resource)]
pub struct AssetLoader {
    pending: HashSet<UntypedAssetId>,
}

impl AssetLoader {
    pub fn new() -> Self {
        Self {
            pending: HashSet::default(),
        }
    }

    pub fn update_loading_state(&mut self, server: &AssetServer) -> Result<()> {
        let mut errors = Vec::new();

        self.pending.retain(|pending| {
            let path = server.get_path(*pending);
            let states = server.get_load_states(*pending);
            states.map_or(true, |(_, _, state)| {
                if state == RecursiveDependencyLoadState::Loaded {
                    info!("Successfully loaded asset: {:?}", path);
                    false
                } else if state == RecursiveDependencyLoadState::Failed {
                    errors.push(format!("Failed loading asset: {:?}", path));
                    false
                } else {
                    true
                }
            })
        });

        if errors.is_empty() {
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "Error loading assets:\n{}",
                errors.join("\n")
            ))
        }
    }

    pub fn add_pending(&mut self, id: UntypedAssetId) {
        self.pending.insert(id);
    }

    pub fn is_finished(&self) -> bool {
        self.pending.is_empty()
    }
}
