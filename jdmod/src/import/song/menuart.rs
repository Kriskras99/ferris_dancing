//! # Menuart
//! Imports all the textures and phone images used in the menus for this song
use std::{fs::File, io::Write};

use anyhow::{anyhow, Error};
use hipstr::HipStr;
use tracing::{debug, warn};
use ubiart_toolkit::{cooked, cooked::tpl::types::PhoneImages};

use super::SongImportState;
use crate::{
    types::song::{MenuArt, MenuArtTexture, PhoneImage},
    utils::{cook_path, decode_texture},
};

/// Imports all the textures and phone images used in the menus for this song
/* TODO: Replace import/export logic for menuart
   The game only uses coach_{n}, cover_albumcoach, cover_generic, cover_kids, map_bkg, _online and _phone
   cover_kids is only used in the kids menu.
   _online can be a symlink to _generic, and _phone can be created from the decoded texture
   if map_bkg is missing, it should try importing banner_bkg
   if kids is missing, symlink it to generic
*/
pub fn import(
    sis: &SongImportState<'_>,
    menuart_scene: &cooked::isc::Scene<'_>,
    phone_images: &PhoneImages<'_>,
) -> Result<(), Error> {
    let mut menuart = Vec::new();

    for actor in &menuart_scene.actors {
        let actor = actor.actor()?;

        // Get a suitable name from userfriendly
        let name = actor
            .userfriendly
            .split_once('_')
            .ok_or_else(|| anyhow!("Could not split texture name!"))?
            .1;

        // JD2018 8bitretake contains the same cover twice, lets prevent this
        if !menuart.iter().any(|m| {
            if let MenuArt::Texture(m) = m {
                m.name == name
            } else {
                false
            }
        }) {
            let mgc = actor
                .components
                .first()
                .ok_or_else(|| anyhow!("No components in actor"))?
                .material_graphic_component()?;

            let cooked_path = cook_path(
                &mgc.material
                    .gfx_material_serializable
                    .texture_set
                    .gfx_material_texture_path_set
                    .diffuse,
                sis.ugi,
            )?;

            let from = match (sis.vfs.open(cooked_path.as_ref()), sis.lax) {
                (Ok(from), _) => from,
                (Err(err), true) => {
                    if cooked_path.ends_with("_cover_online_kids.tga.ckd")
                        || cooked_path.ends_with("_cover_albumbkg.tga.ckd")
                        || cooked_path.ends_with("_banner_bkg.tga.ckd")
                    {
                        debug!("Warning! {err}");
                    } else {
                        warn!("{err}");
                    }
                    continue;
                }
                (Err(err), false) => return Err(err.into()),
            };

            let decooked_picto = decode_texture(&from, sis.ugi)?;
            let to_filename = format!("{name}.png");
            let path = sis.dirs.menuart().join(&to_filename);
            decooked_picto.save(path)?;

            menuart.push(MenuArt::Texture(MenuArtTexture {
                scale: actor.scale,
                pos2d: actor.pos2d,
                name: name.clone(),
                filename: HipStr::from(to_filename),
                disable_shadow: mgc.disable_shadow,
                anchor: mgc
                    .enums
                    .first()
                    .ok_or_else(|| anyhow!("No enums!"))?
                    .selection,
            }));
        }
    }

    for (name, filename) in phone_images {
        let from = match (sis.vfs.open(filename.as_str().as_ref()), sis.lax) {
            (Ok(from), _) => from,
            (Err(err), true) => {
                debug!("Warning! {err}");
                continue;
            }
            (Err(err), false) => return Err(err.into()),
        };
        let mut new_filename = name.to_lowercase();
        new_filename.push_str("_phone.");
        new_filename.push_str(
            filename
                .rsplit_once('.')
                .ok_or_else(|| anyhow!("Malformed filename"))?
                .1
                .as_str(),
        );
        let mut to = File::create(sis.dirs.menuart().join(&new_filename))?;
        to.write_all(&from)?;

        menuart.push(MenuArt::Phone(PhoneImage {
            filename: HipStr::from(new_filename),
            name: name.clone(),
        }));
    }

    let menuart_path = sis.dirs.menuart().join("menuart.json");

    let menuart_file = File::create(menuart_path)?;
    serde_json::to_writer_pretty(menuart_file, &menuart)?;

    Ok(())
}
