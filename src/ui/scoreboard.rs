use bevy::prelude::*;
use num_bigint::BigUint;

use crate::{
    game::spawn::level::Ring,
    screen::{playing::Currency, Screen},
};

use super::widgets::{CurrencyText, CyclesCountText, PendingCurrencyText};

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
        cycle_count_text.sections[0].value = format!("{}", total_cycles)
    }
}

fn update_currency(
    mut currency: ResMut<Currency>,
    q_rings: Query<&Ring>,
    mut q_currency_text: Query<&mut Text, With<CurrencyText>>,
    mut q_pending_currency_text: Query<&mut Text, (With<PendingCurrencyText>, Without<CurrencyText>)>,
) {
    let total_pending_amount = q_rings
        .iter()
        .fold(BigUint::ZERO, |acc, ring| acc + &ring.pending_amount);
    let mut currency_text = q_currency_text.single_mut();
    let mut pending_currency_text = q_pending_currency_text.single_mut();

    currency.pending_amount = total_pending_amount.clone();
    
    currency_text.sections[0].value = format!(
        "${} ",
        currency.amount,
    );

    pending_currency_text.sections[0].value = format!(
        " ${}",
        total_pending_amount
    );
}
