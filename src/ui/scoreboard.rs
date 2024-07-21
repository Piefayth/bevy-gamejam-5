use bevy::prelude::*;
use num_bigint::BigUint;

use crate::{
    game::spawn::level::Ring,
    screen::{playing::Currency, Screen},
};

use super::widgets::{CurrencyText, CyclesCountText};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (update_cycles, update_currency).run_if(in_state(Screen::Playing)),
    );
}

fn update_cycles(
    q_rings: Query<&Ring>,
    mut q_text: Query<&mut Text, With<CyclesCountText>>,
    old_count: Local<BigUint>,
) {
    if q_text.is_empty() {
        return;
    }

    let mut cycle_count_text = q_text.single_mut();

    let total_cycles = q_rings
        .iter()
        .fold(BigUint::ZERO, |acc, ring| acc + &ring.cycle_count);

    if total_cycles != *old_count {
        cycle_count_text.sections[0].value = format!("Cycle {}", total_cycles)
    }
}

fn update_currency(currency: Res<Currency>, mut q_text: Query<&mut Text, With<CurrencyText>>) {
    if currency.is_changed() {
        let mut currency_text = q_text.single_mut();

        currency_text.sections[0].value = format!(
            "${} (Pending ${})",
            currency.amount, currency.pending_amount
        );
    }
}
