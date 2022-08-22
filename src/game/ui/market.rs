use bevy::prelude::*;

use crate::{
    game::{
        components::{
            MarketPriceDirectionIndicator, MarketPriceIndicator, MarketPriceValueIndicator,
        },
        market::Market,
    },
    loader::TextureAssets,
};

pub fn update_market_price_ui(
    market: Res<Market>,
    textures: Res<TextureAssets>,
    indicators: Query<(&MarketPriceIndicator, &Children)>,
    mut texts: Query<&mut Text, With<MarketPriceValueIndicator>>,
    mut icons: Query<&mut UiImage, With<MarketPriceDirectionIndicator>>,
) {
    for (indicator, children) in indicators.iter() {
        let market_val = market
            .market
            .get(&indicator.0)
            .unwrap()
            .get(&indicator.1)
            .unwrap();

        for child in children.iter() {
            match texts.get_mut(*child) {
                Ok(mut t) => {
                    t.sections[0].value = format!("{}", market_val.current_price);
                    continue;
                }
                _ => {}
            }

            match icons.get_mut(*child) {
                Ok(mut i) => {
                    *i = if market_val.price_acceleration > 0.0 {
                        textures.up.clone()
                    } else {
                        textures.down.clone()
                    }
                    .into();

                    continue;
                }
                _ => {}
            }
        }
    }
}
