use bevy::prelude::*;
use bevy::reflect::GetTypeRegistration;
use bevy_mod_picking::prelude::*;

use crate::animation::offset::Offset;
use crate::core::UpdateSet;
use crate::ui::prelude::*;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(DefaultPickingPlugins);

    app.configure::<(
        IsDisabled,
        InteractionTable<ThemeColorFor<BackgroundColor>>,
        InteractionTable<Offset>,
    )>();
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct IsDisabled(pub bool);

impl Configure for IsDisabled {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

// TODO: Text labels are usually child entities, so this is annoying to implement for text colors.
/// Different values of a component to set for each [`Interaction`] state.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct InteractionTable<C: Component> {
    pub normal: C,
    pub hovered: C,
    pub pressed: C,
    pub disabled: C,
}

impl<C: Component + Clone + Reflect + FromReflect + TypePath + GetTypeRegistration> Configure
    for InteractionTable<C>
{
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            apply_interaction_table::<C>.in_set(UpdateSet::SyncLate),
        );
    }
}

fn apply_interaction_table<C: Component + Clone>(
    mut interaction_query: Query<
        (
            Option<&IsDisabled>,
            &Interaction,
            &InteractionTable<C>,
            &mut C,
        ),
        Or<(Changed<Interaction>, Changed<IsDisabled>)>,
    >,
) {
    for (is_disabled, interaction, table, mut target) in &mut interaction_query {
        // Clone the component from the current `Interaction` state.
        *target = if matches!(is_disabled, Some(IsDisabled(true))) {
            &table.disabled
        } else {
            match interaction {
                Interaction::None => &table.normal,
                Interaction::Hovered => &table.hovered,
                Interaction::Pressed => &table.pressed,
            }
        }
        .clone();
    }
}
