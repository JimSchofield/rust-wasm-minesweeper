use bevy::log;
use bevy::prelude::*;

use crate::{
    components::{Bomb, BombNeighbor, Coordinates, Uncover},
    events::TileTriggerEvent,
    resources::Board,
    BoardCompletedEvent,
    BombExplosionEvent,
};

pub fn trigger_event_handler(
    mut commands: Commands,
    board: Res<Board>,
    mut tile_trigger_evr: EventReader<TileTriggerEvent>,
) {
    for trigger_event in tile_trigger_evr.iter() {
        if let Some(entity) = board.tile_to_uncover(&trigger_event.0) {
            commands.entity(*entity).insert(Uncover);
        }
    }
}

pub fn uncover_tiles(
    mut commands: Commands,
    mut board: ResMut<Board>,
    children: Query<(Entity, &Parent), With<Uncover>>,
    parents: Query<(&Coordinates, Option<&Bomb>, Option<&BombNeighbor>)>,
    mut board_completed_event_wr: EventWriter<BoardCompletedEvent>,
    mut bomb_explosion_event_wr: EventWriter<BombExplosionEvent>,
) {
    for (entity, parent) in children.iter() {
        commands.entity(entity).despawn_recursive();

        let (coords, bomb, bomb_counter) = match parents.get(parent.0) {
            Ok(v) => v,
            Err(e) => {
                log::error!("{}", e);
                continue;
            }
        };

        match board.try_uncover_tile(coords) {
            None => log::debug!("Tried to uncover already uncovered tile"),
            Some(e) => log::debug!("Uncovered tile {} (entity: {:?})", coords, e),
        }

        if board.is_completed() {
            log::info!("Board completed!");
            board_completed_event_wr.send(BoardCompletedEvent);
        }

        if bomb.is_some() {
            bomb_explosion_event_wr.send(BombExplosionEvent);
        } else if bomb_counter.is_none() {
            for entity in board.adjacent_covered_tiles(*coords) {
                commands.entity(entity).insert(Uncover);
            }
        }
    }
}
