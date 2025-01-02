pub mod json;
pub mod path;
pub mod plumbing;

// TODO: Remove pub use and replace uses with utils::path::
use std::{cmp::Ordering, ffi::OsStr, fmt::Display};

use clap::ValueEnum;
use dotstar_toolkit_utils::bytes::{
    primitives::{u32be, u32le},
    read::{BinaryDeserialize, ReadAtExt, ReadError},
    write::{BinarySerialize, WriteAt, WriteError},
};
pub use path::{PathId, SplitPath};
use serde::{Deserialize, Serialize};
use tracing::{trace, warn};
use ubiart_toolkit_shared_types::errors::ParserError;
pub use ubiart_toolkit_shared_types::{errors, Color, LocaleId};

pub struct InternedString;
impl BinaryDeserialize<'_> for InternedString {
    type Ctx = ();
    type Output = &'static str;

    fn deserialize_at_with(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        let string_id = reader.read_at::<u32be>(position)?;
        match string_id {
            0x015A_715D => Ok("jd_stand02"),
            0x01A1_A141 => Ok("jd_disappear01"),
            0x01AB_5AD8 => Ok("rayman_f1"),
            0x0213_982B => Ok("jd_release_instrument_musha"),
            0x0288_3A7E => Ok("MusicTrackComponent_Template"),
            0x0305_962E => Ok("uv_left"),
            0x03B3_8F68 => Ok("jd_airguitartostand"),
            0x043A_54AC => Ok("jd_singing_chorus"),
            0x04AA_BE42 => Ok("jd_release_instrument_mushf"),
            0x0579_E81B => Ok("PleoTextureGraphicComponent"),
            0x094A_A595 => Ok("mushiarchi_c"),
            0x09C2_6AAB => Ok("CameraGraphicComponent_Template"),
            0x09D3_B4FD => Ok("jd_standuturn"),
            0x0A12_48A0 => Ok("jd_jumppink_48i"),
            0x0B16_F4D7 => Ok("jd_jumpgreen"),
            0x0C73_6497 => Ok("MasterTape_Template"),
            0x0DAA_F8A4 => Ok("jd_appear_mushc"),
            0x0E03_0DFB => Ok("JD_UIWidgetGroupHUD_Lyrics_Template"),
            0x0E1D_D749 => Ok("wait"),
            0x0E35_5C68 => Ok("FXControllerComponent_Template"),
            0x0F37_2250 => Ok("jd_move_musha"),
            0x0F9E_0BD8 => Ok("jd_dance05"),
            0x1189_9EA2 => Ok("jd_flyuturn"),
            0x119A_3AD6 => Ok("jd_stand_mushc"),
            0x11E5_28DF => Ok("jd_scream"),
            0x1263_DAD9 => Ok("PleoComponent"),
            0x12E6_D0F7 => Ok("jd_standtojump"),
            0x1366_762E => Ok("bg_color_0"),
            0x13DA_8CEB => Ok("gandalf_c1"),
            0x143C_2F8B => Ok("jd_sleep"),
            0x1445_31FF => Ok("navigation_default"),
            0x1576_B015 => Ok("navigation_row"),
            0x1642_9EB7 => Ok("ViewportUIComponent_Template"),
            0x16B5_8A2E => Ok("jd_waittostand"),
            0x1715_908D => Ok("jd_dancestand"),
            0x1720_AB75 => Ok("stars_raceline_wdf"),
            0x1726_0372 => Ok("fade"),
            0x1759_E29D => Ok("JD_AvatarDescComponent"),
            0x1781_E01E => Ok("jd_dance01"),
            0x188D_DD58 => Ok("jd_standcooltostand"),
            0x18AF_6D8D => Ok("avatar"),
            0x1A7E_999A => Ok("Mesh3DComponent"),
            0x1A94_861B => Ok("hide_coach"),
            0x1AE2_847A => Ok("jd_jumpgreen_48i"),
            0x1B49_A3F5 => Ok("trot"),
            0x1B85_7BCE => Ok("Actor_Template"),
            0x1C5A_989C => Ok("jd_stand_mushd"),
            0x1CD5_FA5D => Ok("jd_standtomove_mushf"),
            0x1DAD_09E9 => Ok("jd_dance02"),
            0x1DC3_B8D9 => Ok("jd_pulse"),
            0x1E28_34CD => Ok("jd_dance07"),
            0x2266_3BF0 => Ok("jd_dance08"),
            0x2351_47EE => Ok("color_3"),
            0x231F_27DE => Ok("TapeCase_Component"),
            0x24A3_7BF0 => Ok("TML_Motion"),
            0x24B7_AF0C => Ok("JD_PictoComponent_Template"),
            0x251D_7B4D => Ok("jd_standgreen"),
            0x2520_774E => Ok("stamppageunlock"),
            0x2551_6A68 => Ok("jd_standtodance"),
            0x2644_D1BB => Ok("jd_jumpforwardyellow"),
            0x27C6_D339 => Ok("JD_AvatarDescTemplate"),
            0x2810_2F02 => Ok("navigation_speed"),
            0x28DF_6F7E => Ok("run"),
            0x2949_932E => Ok("PleoTextureGraphicComponent_Template"),
            0x2962_2272 => Ok("jd_stand04"),
            0x2A67_89CD => Ok("jd_dance_mushc"),
            0x2C9C_B4F0 => Ok("JD_FixedCameraComponent_Template"),
            0x2CE8_53CC => Ok("jd_trot"),
            0x2EAC_DF02 => Ok("jd_freefall"),
            0x2EEA_87A2 => Ok("VIDEO_DLC"),
            0x2F5F_FD16 => Ok("jd_move01"),
            0x3124_19E1 => Ok("jd_appear_musha"),
            0x31D3_B347 => Ok("lyrics"),
            0x3236_CF4C => Ok("BezierTreeComponent"),
            0x323B_9202 => Ok("jd_stand_to_dance"),
            0x32FC_7FE9 => Ok("jd_move_mushc"),
            0x3335_224D => Ok("jd_smiling"),
            0x3337_6E90 => Ok("jd_walk_to_stand"),
            0x3384_44B2 => Ok("jd_standtolittlejumps"),
            0x3499_DADA => Ok("takeoff"),
            0x34A7_203E => Ok("init"),
            0x3584_81FD => Ok("gandalf_a1"),
            0x358D_F7D2 => Ok("jd_standtoairguitar"),
            0x35F9_F405 => Ok("standard"),
            0x3603_D6A1 => Ok("jd_appear03"),
            0x3622_1BD2 => Ok("offstand"),
            0x36E2_685B => Ok("jd_sleeptostand"),
            0x36F0_1B59 => Ok("JD_AsyncPlayerDesc_Template"),
            0x38F4_A3D5 => Ok("JD_UIWidgetElement_Template"),
            0x3930_4379 => Ok("JD_CMU_GenericStage_Component"),
            0x39CE_EA49 => Ok("jd_yeah01"),
            0x39EA_ED0F => Ok("jd_dancing02"),
            0x3AB1_2D51 => Ok("jd_stand_mushb"),
            0x3C0F_7F19 => Ok("jd_jumpforwardpink"),
            0x3C14_4B91 => Ok("jd_run"),
            0x3D58_ACFE => Ok("pulse_glow"),
            0x3D5D_EBA2 => Ok("JD_FixedCameraComponent"),
            0x3E5B_FD0C => Ok("jd_standtowalk_flag"),
            0x3ED0_2533 => Ok("JD_BeatPulseComponent_Template"),
            0x3F24_5FD9 => Ok("jd_yeah02tostand"),
            0x3FDD_8551 => Ok("rayman_d1"),
            0x4055_79FB => Ok("JD_SongDatabaseComponent"),
            0x40A1_5156 => Ok("menu_valid"),
            0x4166_F23C => Ok("snap"),
            0x418F_AF9A => Ok("menu_lstick_right"),
            0x42CF_C2B0 => Ok("jd_release_instrument_mushc"),
            0x42F8_5868 => Ok("jd_falling"),
            0x4491_5DC4 => Ok("crowd"),
            0x450D_5301 => Ok("jd_appear_mushe"),
            0x46EE_152E => Ok("jd_walk"),
            0x48D9_D505 => Ok("jd_airbattery"),
            0x49ED_DBB7 => Ok("bg_color_2"),
            0x4A24_BAD3 => Ok("JD_CMU_GenericStage_Component_Template"),
            0x4B04_9017 => Ok("ui_rollover_button"),
            0x4C55_6308 => Ok("navigation"),
            0x4C84_AA3B => Ok("balls_right_to_left"),
            0x4D4C_66BD => Ok("color_2"),
            0x4DA9_5BE9 => Ok("jd_run_to_fall"),
            0x4E8C_DF75 => Ok("SingleInstanceMesh3DComponent_Template"),
            0x4ECB_49D8 => Ok("jd_appear04"),
            0x4FA4_0F09 => Ok("SubSceneActor"),
            0x51EA_2CD0 => Ok("JD_AutodanceComponent_Template"),
            0x547E_605D => Ok("jd_sing"),
            0x54DB_784A => Ok("jd_dance03"),
            0x5557_347E => Ok("jd_jumppink"),
            0x55BC_2C7E => Ok("rayman_a1"),
            0x561D_11DE => Ok("jd_walk02"),
            0x562C_0D7D => Ok("fly"),
            0x5632_1EA5 => Ok("JD_GoldMoveComponent"),
            0x5739_C533 => Ok("jd_standtomove_mushc"),
            0x5786_F663 => Ok("jd_stand_on_cloud"),
            0x57A8_8768 => Ok("rabbid_a1"),
            0x5881_ED4A => Ok("mushiarchi_a"),
            0x5A20_D4B1 => Ok("video_autodance"),
            0x5B64_8E44 => Ok("JD_BlockFlowTemplate"),
            0x5BF0_F580 => Ok("jd_move_mushf"),
            0x5CB3_3DD6 => Ok("jd_dance"),
            0x5FD9_515A => Ok("howl_a1"),
            0x6025_345F => Ok("standback"),
            0x608D_5073 => Ok("rabbid_c1"),
            0x6143_3FBF => Ok("dance"),
            0x6158_A88A => Ok("idle"),
            0x6310_644A => Ok("dancee"),
            0x64A5_FD03 => Ok("jd_sing_mushe"),
            0x64EC_D957 => Ok("JD_SongDatabaseTemplate"),
            0x64ED_9E36 => Ok("TexturePatcherComponent_Template"),
            0x654D_E0D6 => Ok("jd_stand"),
            0x6696_D39A => Ok("video"),
            0x677B_269B => Ok("MasterTape"),
            0x67B8_BB77 => Ok("JD_AutodanceComponent"),
            0x6817_578B => Ok("balls_left_to_right"),
            0x687A_8D4D => Ok("mushiarchi_f"),
            0x68B1_C3DB => Ok("jd_walkinrythm"),
            0x68ED_319A => Ok("Mesh3DComponent_Template"),
            0x693E_C811 => Ok("JD_UIWidgetGroupHUD_AutodanceRecorder_Template"),
            0x6B2A_14A3 => Ok("jd_standtosing"),
            0x6BAA_6E48 => Ok("fern_a1"),
            0x6BE1_C047 => Ok("jd_walk_flag"),
            0x6D21_306E => Ok("murphy"),
            0x6DC2_DBB2 => Ok("menu_phone_right"),
            0x6E8C_7373 => Ok("Orange"),
            0x6F32_8BC1 => Ok("TexturePatcherComponent"),
            0x6F40_37D0 => Ok("coverflow"),
            0x6F67_D81B => Ok("jd_standtotrumpet"),
            0x6F89_7E64 => Ok("jd_standtowalk"),
            0x7016_06C0 => Ok("bokeh_right_to_left"),
            0x701C_8C35 => Ok("jd_jumptostand"),
            0x70E2_CCB9 => Ok("jd_dance04"),
            0x711E_C932 => Ok("jd_jump_forward_to_stand"),
            0x716A_F260 => Ok("jd_appear02"),
            0x71D0_7D84 => Ok("jd_standtodance04"),
            0x7233_490C => Ok("menu_dpad_right"),
            0x72B6_1FC5 => Ok("MaterialGraphicComponent"),
            0x7411_331E => Ok("navigation_age"),
            0x757A_D36C => Ok("bumpball"),
            0x75BA_CA2E => Ok("appear"),
            0x76BD_C51C => Ok("jd_disappear"),
            0x77B2_F41E => Ok("jd_dancetostand"),
            0x77F7_5307 => Ok("jd_movetostand_mushd"),
            0x7850_5079 => Ok("airguitar"),
            0x785A_2F6B => Ok("c_love01"),
            0x78FF_9B6E => Ok("jd_waddletostand"),
            0x794B_EA49 => Ok("jd_waddlewalk"),
            0x7966_A7DF => Ok("jd_move_mushd"),
            0x7978_DAE6 => Ok("jd_walk_trumpet_to_walk_flag"),
            0x7A7C_235B => Ok("MusicTrackComponent"),
            0x7AC8_2C4C => Ok("jd_standback"),
            0x7B01_9CD6 => Ok("jd_jumpyellow"),
            0x7C0C_114C => Ok("BezierTreeComponent_Template"),
            0x7C4C_7444 => Ok("JumpForward"),
            0x7DA3_98D7 => Ok("jd_jumpforwardgreen"),
            0x7DD8_643C => Ok("SoundComponent"),
            0x7EB5_5950 => Ok("jd_fly"),
            0x7F7A_3028 => Ok("UITextBox_Template"),
            0x815A_1F37 => Ok("jd_appear"),
            0x81A6_96B4 => Ok("rayman_b1"),
            0x8229_ABC3 => Ok("TapeCase_Template"),
            0x824E_4DD0 => Ok("jd_dance_musha"),
            0x830C_0B89 => Ok("jd_dance06"),
            0x83B2_58E1 => Ok("gotodefault"),
            0x84B1_0A9F => Ok("ConvertedTmlTape_Template"),
            0x8530_548F => Ok("jd_appear_mushf"),
            0x855C_ED08 => Ok("jd_walkbasic_soft"),
            0x85F9_F3D2 => Ok("jd_flytostand"),
            0x8603_0624 => Ok("jd_dance04tostand"),
            0x8620_5C56 => Ok("jd_sing01"),
            0x869A_BC4F => Ok("jd_sing_mushc"),
            0x86A9_AFB0 => Ok("rabbid_d1"),
            0x87E1_83E7 => Ok("jd_dance_mushf"),
            0x892D_18CD => Ok("sleep"),
            0x8936_CC52 => Ok("jd_stand_mushe"),
            0x8A94_1A86 => Ok("jd_standtomove_mushb"),
            0x8AC2_B5C6 => Ok("JD_SongDescTemplate"),
            0x8D4F_FFB6 => Ok("FxControllerComponent"),
            0x8D84_424B => Ok("jump"),
            0x8DA9_E375 => Ok("JD_BlockFlowComponent"),
            0x8E09_B64A => Ok("navigation_kids"),
            0x8F54_5995 => Ok("JD_RegistrationComponent_Template"),
            0x9070_0035 => Ok("rayman_c1"),
            0x91C3_85E9 => Ok("jd_littlejumps"),
            0x9313_0918 => Ok("jd_stand01"),
            0x9396_1EB6 => Ok("moved"),
            0x9426_8A9B => Ok("jd_stand_to_walk"),
            0x95E3_1B9D => Ok("jd_dance_mushb"),
            0x9738_0045 => Ok("jd_standtojumpstand"),
            0x97AD_6452 => Ok("move"),
            0x97B9_4CDF => Ok("mushiarchi_b"),
            0x97CA_628B => Ok("Actor"),
            0x97D7_B4CF => Ok("rayman_e1"),
            0x98D3_E1B6 => Ok("jd_trumpettostand"),
            0x9A0A_4843 => Ok("PleoComponent_Template"),
            0x9A2C_5760 => Ok("bg_color_3"),
            0x9AAF_0CC6 => Ok("heart_a"),
            0x9C09_36AB => Ok("stand"),
            0x9CAE_4325 => Ok("TextureGraphicComponent_Template"),
            0x9CCE_B199 => Ok("checkbox"),
            0x9CD9_0BCB => Ok("theme"),
            0x9D17_9797 => Ok("fishswarm_a1"),
            0x9DA8_C9ED => Ok("jd_standyellow"),
            0x9DB6_950C => Ok("jd_airguitar"),
            0x9DF8_8CAE => Ok("drc"),
            0x9E6F_4101 => Ok("jd_standcool"),
            0x9F87_350C => Ok("JD_UIWidgetGroupHUD_AutodanceRecorder"),
            0xA071_AE72 => Ok("jd_movetostand_mushb"),
            0xA2A6_29C6 => Ok("singing"),
            0xA388_7F2D => Ok("jd_laughing"),
            0xA521_AB39 => Ok("jd_stand03"),
            0xA58D_BCC2 => Ok("JD_TransitionSceneConfig"),
            0xA5A7_99CA => Ok("pulse_fg"),
            0xA5BE_76EE => Ok("dancea"),
            0xA5F8_835B => Ok("jd_standtoairbattery"),
            0xA74B_1D8A => Ok("jd_standtostandcool"),
            0xA948_2D80 => Ok("scream"),
            0xA9B9_1515 => Ok("JD_ChannelZappingComponent_Template"),
            0xAA48_3873 => Ok("hit"),
            0xAA55_B6BD => Ok("asyncplayervideo"),
            0xAA5B_5DAD => Ok("ClearColorComponent_Template"),
            0xAB6E_1718 => Ok("wdf_crowdloop"),
            0xAA8F_6E39 => Ok("bg_color_1"),
            0xABF3_773E => Ok("master"),
            0xAC95_218D => Ok("jd_jump_forward"),
            0xAD1A_4447 => Ok("partymaster_coach"),
            0xADA3_A4B8 => Ok("jd_move_mushe"),
            0xAE3A_649E => Ok("jd_wait"),
            0xAEBB_218B => Ok("ClearColorComponent"),
            0xAEC9_B9AE => Ok("JD_UIWidgetGroupHUD_Template"),
            0xAEEC_36C0 => Ok("jd_move01tostand01"),
            0xAF54_ED04 => Ok("walk"),
            0xAF76_E14E => Ok("jd_sing_mushb"),
            0xAF8D_5904 => Ok("jd_appear_mushd"),
            0xB04E_87AE => Ok("jd_standtoyeah02"),
            0xB11F_C1B6 => Ok("prelobby"),
            0xB1BB_5C98 => Ok("d_taunt01"),
            0xB20E_35D5 => Ok("navigation_big_items"),
            0xB244_0614 => Ok("falling"),
            0xB2EA_C9D8 => Ok("jd_walk_mushf"),
            0xB312_14C3 => Ok("mushiarchi_d"),
            0xB3FD_416D => Ok("jd_walk_trumpet"),
            0xB46D_B10C => Ok("jump_forward"),
            0xB5FF_66D7 => Ok("jd_move_mushb"),
            0xB6C2_C936 => Ok("jd_walk_mushe"),
            0xB74B_7B4B => Ok("bokeh_left_to_right"),
            0xB75F_C150 => Ok("jd_dance02tostand"),
            0xB7A8_AA24 => Ok("jd_standtodance02"),
            0xBA69_7320 => Ok("JD_GoldMoveComponent_Template"),
            0xBB2E_8C8C => Ok("jd_walk_mushc"),
            0xBEA8_2EB8 => Ok("JD_CreditsComponent_Template"),
            0xC0B3_9AD2 => Ok("jd_playingthetrumpet"),
            0xC13C_4C60 => Ok("jd_sing_mushd"),
            0xC224_1528 => Ok("squirrel_a1"),
            0xC2A7_097A => Ok("jd_yeah02"),
            0xC2CD_6D18 => Ok("jd_standtodance01"),
            0xC305_83CB => Ok("jd_walk_flag_to_walk_trumpet"),
            0xC316_BF34 => Ok("JD_PictoComponent"),
            0xC33B_4C02 => Ok("menu_phone_left"),
            0xC3E9_91D9 => Ok("jd_walktostand"),
            0xC530_C4B5 => Ok("jd_standtomove_musha"),
            0xC549_311C => Ok("pulse"),
            0xC658_7B65 => Ok("jd_sing_mushf"),
            0xC738_9490 => Ok("ui"),
            0xC74C_A426 => Ok("jd_walk01"),
            0xC78C_6935 => Ok("jd_jumpyellow_48i"),
            0xC7EA_1948 => Ok("fx"),
            0xC917_3E02 => Ok("jd_appear01tomove01"),
            0xC981_76EB => Ok("jd_standtofly"),
            0xC992_571B => Ok("jd_jump01"),
            0xCA41_998C => Ok("b_sad_angry01"),
            0xCC4B_3C67 => Ok("VIDEO_DWS"),
            0xCD07_BB76 => Ok("ConvertedTmlTape_Component"),
            0xCD37_0C70 => Ok("color_0"),
            0xCD94_7561 => Ok("bg_color_4"),
            0xCE01_8EDB => Ok("JD_MapSceneConfig"),
            0xCE44_8441 => Ok("hud"),
            0xD010_A414 => Ok("jd_jump02"),
            0xD10C_BEED => Ok("UITextBox"),
            0xD28C_B99B => Ok("jd_stand_mushf"),
            0xD364_964F => Ok("notepiafplatform_b1"),
            0xD52B_3D82 => Ok("jd_jumpstand"),
            0xD567_9795 => Ok("jd_yeah01tostand"),
            0xD5B5_4597 => Ok("TML_Sequence"),
            0xD617_EC47 => Ok("jd_dance03tostand"),
            0xD64E_0E2A => Ok("menu_dpad_left"),
            0xD72F_EEFF => Ok("rabbid_b1"),
            0xD742_D7F9 => Ok("singe"),
            0xD752_7D31 => Ok("danced"),
            0xD87E_9293 => Ok("jd_appear01tostand01"),
            0xD94D_6C53 => Ok("SoundComponent_Template"),
            0xD9B1_E95C => Ok("menu_lstick_left"),
            0xDA4E_FA04 => Ok("jd_standtomove_mushd"),
            0xDA70_FD02 => Ok("uv_right"),
            0xDAEA_2FC4 => Ok("jd_dance_mushd"),
            0xDCB5_AB6D => Ok("jd_movetostand_mushc"),
            0xDCC3_1B4A => Ok("jd_standtodance03"),
            0xDD28_30A7 => Ok("sing"),
            0xDD51_94F2 => Ok("jd_jumpstandtostand"),
            0xDF1F_C047 => Ok("jd_stand01tomove01"),
            0xDF5D_EFE1 => Ok("jd_walk_flag_to_stand"),
            0xDFEF_DBFB => Ok("decel"),
            0xE07F_CC3F => Ok("JD_SongDescComponent"),
            0xE083_9849 => Ok("jd_sing_musha"),
            0xE0A2_4B6D => Ok("JD_RegistrationComponent"),
            0xE0A7_3096 => Ok("mushiarchi_e"),
            0xE1BB_B7F9 => Ok("cypress_a1"),
            0xE335_352C => Ok("jd_standpink"),
            0xE342_3550 => Ok("a_happy01"),
            0xE3F4_71A9 => Ok("jd_standtodance06"),
            0xE41C_9FC6 => Ok("UIItemTextField_Template"),
            0xE5C0_74A1 => Ok("jd_singtostand"),
            0xE628_44B8 => Ok("JD_UIBannerSceneConfig"),
            0xE67E_5894 => Ok("color_1"),
            0xE691_30A9 => Ok("jd_walk_mushd"),
            0xE754_EB09 => Ok("jd_standtomove_mushe"),
            0xE798_2D66 => Ok("jd_stand_musha"),
            0xE848_9177 => Ok("jd_dance_mushe"),
            0xE8BF_CBAB => Ok("jd_dance01tostand"),
            0xE937_BA26 => Ok("jd_walk_mushb"),
            0xE9D0_D72E => Ok("jd_standtoyeah01"),
            0xEA9C_6A7D => Ok("jd_appear_mushb"),
            0xEB35_D2EA => Ok("jd_standtowait"),
            0xEB53_7A60 => Ok("AMB"),
            0xECDC_E65E => Ok("jd_standblink"),
            0xED6A_45B8 => Ok("jd_movetostand_mushe"),
            0xED70_D456 => Ok("jd_movetostand_mushf"),
            0xED94_7296 => Ok("landing"),
            0xEE1B_E23C => Ok("jd_release_instrument_mushb"),
            0xEE8E_8E5A => Ok("jd_walk_musha"),
            0xF13C_2D9B => Ok("jd_jump"),
            0xF22C_9426 => Ok("JD_UIWidgetGroupHUD_Lyrics"),
            0xF234_BE67 => Ok("jd_disappear02"),
            0xF27C_A99F => Ok("laughing"),
            0xF352_6572 => Ok("jd_release_instrument_mushd"),
            0xF390_F834 => Ok("globox_j1"),
            0xF878_DC2D => Ok("JD_SongDatabaseSceneConfig"),
            0xF9BE_082F => Ok("MaterialGraphicComponent_Template"),
            0xFA35_7DA1 => Ok("wiimote"),
            0xFD45_47AC => Ok("TML_Karaoke"),
            0xFE29_CF9B => Ok("jd_movetostand_musha"),
            0xFEC5_1930 => Ok("jd_appear01"),
            0xFEC7_7C31 => Ok("jd_release_instrument_mushe"),
            0xFF97_3637 => Ok("jd_dancing01"),
            0xFFA7_E616 => Ok("jd_standtowaddle"),
            0xFFFF_FFFF => Ok(""),
            0x3BFE_3B8C => {
                trace!("Unknown interned string 0x3BFE_3B8C");
                Ok("Unknown interned string 0x3BFE_3B8C")
            }
            0x65B1_633D => {
                trace!("Unknown interned string 0x65B1_633D");
                Ok("Unknown interned string 0x65B1_633D")
            }
            0x23DA_961A => {
                trace!("Unknown interned string 0x23DA_961A");
                Ok("Unknown interned string 0x23DA_961A")
            }
            0x7FE5_2E1D => {
                trace!("Unknown interned string 0x7FE5_2E1D");
                Ok("Unknown interned string 0x7FE5_2E1D")
            }
            0x3B7C_7648 => {
                trace!("Unknown interned string 0x3B7C_7648");
                Ok("Unknown interned string 0x3B7C_7648")
            }
            0x5DEB_327B => {
                trace!("Unknown interned string 0x5DEB_327B");
                Ok("Unknown interned string 0x5DEB_327B")
            }
            0x64F5_BE2C => {
                trace!("Unknown interned string 0x64F5_BE2C");
                Ok("Unknown interned string 0x64F5_BE2C")
            }
            0x6E9F_D71B => {
                trace!("Unknown interned string 0x6E9F_D71B");
                Ok("Unknown interned string 0x6E9F_D71B")
            }
            0x64B5_BE2C => {
                trace!("Unknown interned string 0x64B5_BE2C");
                Ok("Unknown interned string 0x64B5_BE2C")
            }
            0x7CFA_6BFD => {
                trace!("Unknown interned string 0x7CFA_6BFD");
                Ok("Unknown interned string 0x7CFA_6BFD")
            }
            0x8D98_844F => {
                trace!("Unknown interned string 0x8D98_844F");
                Ok("Unknown interned string 0x8D98_844F")
            }
            0xA8BA_E977 => {
                trace!("Unknown interned string 0xA8BA_E977");
                Ok("Unknown interned string 0xA8BA_E977")
            }
            0xAB82_5F73 => {
                trace!("Unknown interned string 0xAB82_5F73");
                Ok("Unknown interned string 0xAB82_5F73")
            }
            0xBF95_30AF => {
                trace!("Unknown interned string 0xBF95_30AF");
                Ok("Unknown interned string 0xBF95_30AF")
            }
            0xC918_094C => {
                trace!("Unknown interned string 0xC918_094C");
                Ok("Unknown interned string 0xC918_094C")
            }
            0x00F4_BE74 => {
                trace!("Unknown interned string 0x00F4_BE74");
                Ok("Unknown interned string 0x00F4_BE74")
            }
            0x012C_17C9 => {
                trace!("Unknown interned string 0x012C_17C9");
                Ok("Unknown interned string 0x012C_17C9")
            }
            0x034A_931D => {
                trace!("Unknown interned string 0x034A_931D");
                Ok("Unknown interned string 0x034A_931D")
            }
            0x04B5_6A92 => {
                trace!("Unknown interned string 0x04B5_6A92");
                Ok("Unknown interned string 0x04B5_6A92")
            }
            0x05A6_6299 => {
                trace!("Unknown interned string 0x05A6_6299");
                Ok("Unknown interned string 0x05A6_6299")
            }
            0x05F6_3C7D => {
                trace!("Unknown interned string 0x05F6_3C7D");
                Ok("Unknown interned string 0x05F6_3C7D")
            }
            0x06EE_FD1C => {
                trace!("Unknown interned string 0x06EE_FD1C");
                Ok("Unknown interned string 0x06EE_FD1C")
            }
            0x0700_42BC => {
                trace!("Unknown interned string 0x0700_42BC");
                Ok("Unknown interned string 0x0700_42BC")
            }
            0x0709_2316 => {
                trace!("Unknown interned string 0x0709_2316");
                Ok("Unknown interned string 0x0709_2316")
            }
            0x076B_4F4A => {
                trace!("Unknown interned string 0x076B_4F4A");
                Ok("Unknown interned string 0x076B_4F4A")
            }
            0x07EF_C35F => {
                trace!("Unknown interned string 0x07EF_C35F");
                Ok("Unknown interned string 0x07EF_C35F")
            }
            0x0804_5EBE => {
                trace!("Unknown interned string 0x0804_5EBE");
                Ok("Unknown interned string 0x0804_5EBE")
            }
            0x08BF_AE2F => {
                trace!("Unknown interned string 0x08BF_AE2F");
                Ok("Unknown interned string 0x08BF_AE2F")
            }
            0x08C7_1C17 => {
                trace!("Unknown interned string 0x08C7_1C17");
                Ok("Unknown interned string 0x08C7_1C17")
            }
            0x0943_EEDC => {
                trace!("Unknown interned string 0x0943_EEDC");
                Ok("Unknown interned string 0x0943_EEDC")
            }
            0x099C_916E => {
                trace!("Unknown interned string 0x099C_916E");
                Ok("Unknown interned string 0x099C_916E")
            }
            0x09C6_C1BF => {
                trace!("Unknown interned string 0x09C6_C1BF");
                Ok("Unknown interned string 0x09C6_C1BF")
            }
            0x0A78_8CCD => {
                trace!("Unknown interned string 0x0A78_8CCD");
                Ok("Unknown interned string 0x0A78_8CCD")
            }
            0x0B11_F085 => {
                trace!("Unknown interned string 0x0B11_F085");
                Ok("Unknown interned string 0x0B11_F085")
            }
            0x0BA3_7717 => {
                trace!("Unknown interned string 0x0BA3_7717");
                Ok("Unknown interned string 0x0BA3_7717")
            }
            0x0C51_C9FC => {
                trace!("Unknown interned string 0x0C51_C9FC");
                Ok("Unknown interned string 0x0C51_C9FC")
            }
            0x0CA4_4B2A => {
                trace!("Unknown interned string 0x0CA4_4B2A");
                Ok("Unknown interned string 0x0CA4_4B2A")
            }
            0x0D8A_3ABE => {
                trace!("Unknown interned string 0x0D8A_3ABE");
                Ok("Unknown interned string 0x0D8A_3ABE")
            }
            0x0ECF_E2F9 => {
                trace!("Unknown interned string 0x0ECF_E2F9");
                Ok("Unknown interned string 0x0ECF_E2F9")
            }
            0x0F75_B242 => {
                trace!("Unknown interned string 0x0F75_B242");
                Ok("Unknown interned string 0x0F75_B242")
            }
            0x0F92_8FB7 => {
                trace!("Unknown interned string 0x0F92_8FB7");
                Ok("Unknown interned string 0x0F92_8FB7")
            }
            0x1011_D446 => {
                trace!("Unknown interned string 0x1011_D446");
                Ok("Unknown interned string 0x1011_D446")
            }
            0x10E9_0D36 => {
                trace!("Unknown interned string 0x10E9_0D36");
                Ok("Unknown interned string 0x10E9_0D36")
            }
            0x122C_5358 => {
                trace!("Unknown interned string 0x122C_5358");
                Ok("Unknown interned string 0x122C_5358")
            }
            0x13EA_5A32 => {
                trace!("Unknown interned string 0x13EA_5A32");
                Ok("Unknown interned string 0x13EA_5A32")
            }
            0x149F_8261 => {
                trace!("Unknown interned string 0x149F_8261");
                Ok("Unknown interned string 0x149F_8261")
            }
            0x1647_2351 => {
                trace!("Unknown interned string 0x1647_2351");
                Ok("Unknown interned string 0x1647_2351")
            }
            0x164B_1797 => {
                trace!("Unknown interned string 0x164B_1797");
                Ok("Unknown interned string 0x164B_1797")
            }
            0x1997_9135 => {
                trace!("Unknown interned string 0x1997_9135");
                Ok("Unknown interned string 0x1997_9135")
            }
            0x1A0E_6875 => {
                trace!("Unknown interned string 0x1A0E_6875");
                Ok("Unknown interned string 0x1A0E_6875")
            }
            0x1AA3_3E73 => {
                trace!("Unknown interned string 0x1AA3_3E73");
                Ok("Unknown interned string 0x1AA3_3E73")
            }
            0x1B3A_188C => {
                trace!("Unknown interned string 0x1B3A_188C");
                Ok("Unknown interned string 0x1B3A_188C")
            }
            0x1B5E_E3D8 => {
                trace!("Unknown interned string 0x1B5E_E3D8");
                Ok("Unknown interned string 0x1B5E_E3D8")
            }
            0x1CE2_53CE => {
                trace!("Unknown interned string 0x1CE2_53CE");
                Ok("Unknown interned string 0x1CE2_53CE")
            }
            0x1D0B_2295 => {
                trace!("Unknown interned string 0x1D0B_2295");
                Ok("Unknown interned string 0x1D0B_2295")
            }
            0x1D2D_7E04 => {
                trace!("Unknown interned string 0x1D2D_7E04");
                Ok("Unknown interned string 0x1D2D_7E04")
            }
            0x1E02_4C1C => {
                trace!("Unknown interned string 0x1E02_4C1C");
                Ok("Unknown interned string 0x1E02_4C1C")
            }
            0x1E44_4456 => {
                trace!("Unknown interned string 0x1E44_4456");
                Ok("Unknown interned string 0x1E44_4456")
            }
            0x1EE4_154E => {
                trace!("Unknown interned string 0x1EE4_154E");
                Ok("Unknown interned string 0x1EE4_154E")
            }
            0x1FB3_53D9 => {
                trace!("Unknown interned string 0x1FB3_53D9");
                Ok("Unknown interned string 0x1FB3_53D9")
            }
            0x20C2_59BB => {
                trace!("Unknown interned string 0x20C2_59BB");
                Ok("Unknown interned string 0x20C2_59BB")
            }
            0x211A_00BE => {
                trace!("Unknown interned string 0x211A_00BE");
                Ok("Unknown interned string 0x211A_00BE")
            }
            0x2124_0007 => {
                trace!("Unknown interned string 0x2124_0007");
                Ok("Unknown interned string 0x2124_0007")
            }
            0x2156_E2E1 => {
                trace!("Unknown interned string 0x2156_E2E1");
                Ok("Unknown interned string 0x2156_E2E1")
            }
            0x21CE_367A => {
                trace!("Unknown interned string 0x21CE_367A");
                Ok("Unknown interned string 0x21CE_367A")
            }
            0x2364_6E2C => {
                trace!("Unknown interned string 0x2364_6E2C");
                Ok("Unknown interned string 0x2364_6E2C")
            }
            0x23D5_F8DF => {
                trace!("Unknown interned string 0x23D5_F8DF");
                Ok("Unknown interned string 0x23D5_F8DF")
            }
            0x23E4_DA31 => {
                trace!("Unknown interned string 0x23E4_DA31");
                Ok("Unknown interned string 0x23E4_DA31")
            }
            0x24B9_2D4F => {
                trace!("Unknown interned string 0x24B9_2D4F");
                Ok("Unknown interned string 0x24B9_2D4F")
            }
            0x25B3_414D => {
                trace!("Unknown interned string 0x25B3_414D");
                Ok("Unknown interned string 0x25B3_414D")
            }
            0x25E6_4778 => {
                trace!("Unknown interned string 0x25E6_4778");
                Ok("Unknown interned string 0x25E6_4778")
            }
            0x2713_3495 => {
                trace!("Unknown interned string 0x2713_3495");
                Ok("Unknown interned string 0x2713_3495")
            }
            0x29BB_08B5 => {
                trace!("Unknown interned string 0x29BB_08B5");
                Ok("Unknown interned string 0x29BB_08B5")
            }
            0x2A1B_4AA9 => {
                trace!("Unknown interned string 0x2A1B_4AA9");
                Ok("Unknown interned string 0x2A1B_4AA9")
            }
            0x2AD7_1E9D => {
                trace!("Unknown interned string 0x2AD7_1E9D");
                Ok("Unknown interned string 0x2AD7_1E9D")
            }
            0x2AEB_AF7A => {
                trace!("Unknown interned string 0x2AEB_AF7A");
                Ok("Unknown interned string 0x2AEB_AF7A")
            }
            0x2B46_95EA => {
                trace!("Unknown interned string 0x2B46_95EA");
                Ok("Unknown interned string 0x2B46_95EA")
            }
            0x2CB3_C8E8 => {
                trace!("Unknown interned string 0x2CB3_C8E8");
                Ok("Unknown interned string 0x2CB3_C8E8")
            }
            0x2E4A_14E0 => {
                trace!("Unknown interned string 0x2E4A_14E0");
                Ok("Unknown interned string 0x2E4A_14E0")
            }
            0x2F05_5A2B => {
                trace!("Unknown interned string 0x2F05_5A2B");
                Ok("Unknown interned string 0x2F05_5A2B")
            }
            0x2F1A_4B9D => {
                trace!("Unknown interned string 0x2F1A_4B9D");
                Ok("Unknown interned string 0x2F1A_4B9D")
            }
            0x302A_5919 => {
                trace!("Unknown interned string 0x302A_5919");
                Ok("Unknown interned string 0x302A_5919")
            }
            0x34C9_3E4A => {
                trace!("Unknown interned string 0x34C9_3E4A");
                Ok("Unknown interned string 0x34C9_3E4A")
            }
            0x34C9_5267 => {
                trace!("Unknown interned string 0x34C9_5267");
                Ok("Unknown interned string 0x34C9_5267")
            }
            0x34DC_04C5 => {
                trace!("Unknown interned string 0x34DC_04C5");
                Ok("Unknown interned string 0x34DC_04C5")
            }
            0x3591_2E84 => {
                trace!("Unknown interned string 0x3591_2E84");
                Ok("Unknown interned string 0x3591_2E84")
            }
            0x360D_E1CB => {
                trace!("Unknown interned string 0x360D_E1CB");
                Ok("Unknown interned string 0x360D_E1CB")
            }
            0x3728_2FB8 => {
                trace!("Unknown interned string 0x3728_2FB8");
                Ok("Unknown interned string 0x3728_2FB8")
            }
            0x381A_3226 => {
                trace!("Unknown interned string 0x381A_3226");
                Ok("Unknown interned string 0x381A_3226")
            }
            0x384E_95E1 => {
                trace!("Unknown interned string 0x384E_95E1");
                Ok("Unknown interned string 0x384E_95E1")
            }
            0x38E7_F502 => {
                trace!("Unknown interned string 0x38E7_F502");
                Ok("Unknown interned string 0x38E7_F502")
            }
            0x3A0C_6265 => {
                trace!("Unknown interned string 0x3A0C_6265");
                Ok("Unknown interned string 0x3A0C_6265")
            }
            0x3A44_17EC => {
                trace!("Unknown interned string 0x3A44_17EC");
                Ok("Unknown interned string 0x3A44_17EC")
            }
            0x3A5C_28C8 => {
                trace!("Unknown interned string 0x3A5C_28C8");
                Ok("Unknown interned string 0x3A5C_28C8")
            }
            0x3A76_00C8 => {
                trace!("Unknown interned string 0x3A76_00C8");
                Ok("Unknown interned string 0x3A76_00C8")
            }
            0x3B21_DCE3 => {
                trace!("Unknown interned string 0x3B21_DCE3");
                Ok("Unknown interned string 0x3B21_DCE3")
            }
            0x3B7D_06FE => {
                trace!("Unknown interned string 0x3B7D_06FE");
                Ok("Unknown interned string 0x3B7D_06FE")
            }
            0x3BA4_887E => {
                trace!("Unknown interned string 0x3BA4_887E");
                Ok("Unknown interned string 0x3BA4_887E")
            }
            0x3D7C_C9EA => {
                trace!("Unknown interned string 0x3D7C_C9EA");
                Ok("Unknown interned string 0x3D7C_C9EA")
            }
            0x3E14_0F75 => {
                trace!("Unknown interned string 0x3E14_0F75");
                Ok("Unknown interned string 0x3E14_0F75")
            }
            0x3E2B_7D2C => {
                trace!("Unknown interned string 0x3E2B_7D2C");
                Ok("Unknown interned string 0x3E2B_7D2C")
            }
            0x3E40_A24D => {
                trace!("Unknown interned string 0x3E40_A24D");
                Ok("Unknown interned string 0x3E40_A24D")
            }
            0x40AB_6630 => {
                trace!("Unknown interned string 0x40AB_6630");
                Ok("Unknown interned string 0x40AB_6630")
            }
            0x40C3_2C24 => {
                trace!("Unknown interned string 0x40C3_2C24");
                Ok("Unknown interned string 0x40C3_2C24")
            }
            0x41F0_FD39 => {
                trace!("Unknown interned string 0x41F0_FD39");
                Ok("Unknown interned string 0x41F0_FD39")
            }
            0x43CC_B169 => {
                trace!("Unknown interned string 0x43CC_B169");
                Ok("Unknown interned string 0x43CC_B169")
            }
            0x4415_EAAB => {
                trace!("Unknown interned string 0x4415_EAAB");
                Ok("Unknown interned string 0x4415_EAAB")
            }
            0x45D9_37F6 => {
                trace!("Unknown interned string 0x45D9_37F6");
                Ok("Unknown interned string 0x45D9_37F6")
            }
            0x45DB_77E3 => {
                trace!("Unknown interned string 0x45DB_77E3");
                Ok("Unknown interned string 0x45DB_77E3")
            }
            0x45F5_16B6 => {
                trace!("Unknown interned string 0x45F5_16B6");
                Ok("Unknown interned string 0x45F5_16B6")
            }
            0x4640_43DC => {
                trace!("Unknown interned string 0x4640_43DC");
                Ok("Unknown interned string 0x4640_43DC")
            }
            0x476A_BE41 => {
                trace!("Unknown interned string 0x476A_BE41");
                Ok("Unknown interned string 0x476A_BE41")
            }
            0x4776_A88B => {
                trace!("Unknown interned string 0x4776_A88B");
                Ok("Unknown interned string 0x4776_A88B")
            }
            0x484A_28F4 => {
                trace!("Unknown interned string 0x484A_28F4");
                Ok("Unknown interned string 0x484A_28F4")
            }
            0x496F_21D7 => {
                trace!("Unknown interned string 0x496F_21D7");
                Ok("Unknown interned string 0x496F_21D7")
            }
            0x4B4B_6ACA => {
                trace!("Unknown interned string 0x4B4B_6ACA");
                Ok("Unknown interned string 0x4B4B_6ACA")
            }
            0x4BAB_4944 => {
                trace!("Unknown interned string 0x4BAB_4944");
                Ok("Unknown interned string 0x4BAB_4944")
            }
            0x4D66_E42D => {
                trace!("Unknown interned string 0x4D66_E42D");
                Ok("Unknown interned string 0x4D66_E42D")
            }
            0x4E21_C2C3 => {
                trace!("Unknown interned string 0x4E21_C2C3");
                Ok("Unknown interned string 0x4E21_C2C3")
            }
            0x4F88_2E0A => {
                trace!("Unknown interned string 0x4F88_2E0A");
                Ok("Unknown interned string 0x4F88_2E0A")
            }
            0x5093_7023 => {
                trace!("Unknown interned string 0x5093_7023");
                Ok("Unknown interned string 0x5093_7023")
            }
            0x5217_57F6 => {
                trace!("Unknown interned string 0x5217_57F6");
                Ok("Unknown interned string 0x5217_57F6")
            }
            0x5289_FE86 => {
                trace!("Unknown interned string 0x5289_FE86");
                Ok("Unknown interned string 0x5289_FE86")
            }
            0x5297_28B7 => {
                trace!("Unknown interned string 0x5297_28B7");
                Ok("Unknown interned string 0x5297_28B7")
            }
            0x5325_1DE8 => {
                trace!("Unknown interned string 0x5325_1DE8");
                Ok("Unknown interned string 0x5325_1DE8")
            }
            0x544B_54F5 => {
                trace!("Unknown interned string 0x544B_54F5");
                Ok("Unknown interned string 0x544B_54F5")
            }
            0x5553_9604 => {
                trace!("Unknown interned string 0x5553_9604");
                Ok("Unknown interned string 0x5553_9604")
            }
            0x5565_69E1 => {
                trace!("Unknown interned string 0x5565_69E1");
                Ok("Unknown interned string 0x5565_69E1")
            }
            0x564B_AC77 => {
                trace!("Unknown interned string 0x564B_AC77");
                Ok("Unknown interned string 0x564B_AC77")
            }
            0x565D_8B3E => {
                trace!("Unknown interned string 0x565D_8B3E");
                Ok("Unknown interned string 0x565D_8B3E")
            }
            0x569A_C7DF => {
                trace!("Unknown interned string 0x569A_C7DF");
                Ok("Unknown interned string 0x569A_C7DF")
            }
            0x577D_702F => {
                trace!("Unknown interned string 0x577D_702F");
                Ok("Unknown interned string 0x577D_702F")
            }
            0x57C6_C642 => {
                trace!("Unknown interned string 0x57C6_C642");
                Ok("Unknown interned string 0x57C6_C642")
            }
            0x582D_3EF7 => {
                trace!("Unknown interned string 0x582D_3EF7");
                Ok("Unknown interned string 0x582D_3EF7")
            }
            0x5890_539D => {
                trace!("Unknown interned string 0x5890_539D");
                Ok("Unknown interned string 0x5890_539D")
            }
            0x5926_CA9A => {
                trace!("Unknown interned string 0x5926_CA9A");
                Ok("Unknown interned string 0x5926_CA9A")
            }
            0x593D_D635 => {
                trace!("Unknown interned string 0x593D_D635");
                Ok("Unknown interned string 0x593D_D635")
            }
            0x599B_4A34 => {
                trace!("Unknown interned string 0x599B_4A34");
                Ok("Unknown interned string 0x599B_4A34")
            }
            0x5B33_0CCC => {
                trace!("Unknown interned string 0x5B33_0CCC");
                Ok("Unknown interned string 0x5B33_0CCC")
            }
            0x5BA3_1692 => {
                trace!("Unknown interned string 0x5BA3_1692");
                Ok("Unknown interned string 0x5BA3_1692")
            }
            0x5C8E_3A59 => {
                trace!("Unknown interned string 0x5C8E_3A59");
                Ok("Unknown interned string 0x5C8E_3A59")
            }
            0x5CBD_978E => {
                trace!("Unknown interned string 0x5CBD_978E");
                Ok("Unknown interned string 0x5CBD_978E")
            }
            0x5E37_0BAB => {
                trace!("Unknown interned string 0x5E37_0BAB");
                Ok("Unknown interned string 0x5E37_0BAB")
            }
            0x5F72_ACD7 => {
                trace!("Unknown interned string 0x5F72_ACD7");
                Ok("Unknown interned string 0x5F72_ACD7")
            }
            0x6071_54FD => {
                trace!("Unknown interned string 0x6071_54FD");
                Ok("Unknown interned string 0x6071_54FD")
            }
            0x6113_4203 => {
                trace!("Unknown interned string 0x6113_4203");
                Ok("Unknown interned string 0x6113_4203")
            }
            0x6117_8BDD => {
                trace!("Unknown interned string 0x6117_8BDD");
                Ok("Unknown interned string 0x6117_8BDD")
            }
            0x6172_EE79 => {
                trace!("Unknown interned string 0x6172_EE79");
                Ok("Unknown interned string 0x6172_EE79")
            }
            0x6223_0A78 => {
                trace!("Unknown interned string 0x6223_0A78");
                Ok("Unknown interned string 0x6223_0A78")
            }
            0x6235_13B4 => {
                trace!("Unknown interned string 0x6235_13B4");
                Ok("Unknown interned string 0x6235_13B4")
            }
            0x624F_A519 => {
                trace!("Unknown interned string 0x624F_A519");
                Ok("Unknown interned string 0x624F_A519")
            }
            0x625C_D8F4 => {
                trace!("Unknown interned string 0x625C_D8F4");
                Ok("Unknown interned string 0x625C_D8F4")
            }
            0x6375_AED7 => {
                trace!("Unknown interned string 0x6375_AED7");
                Ok("Unknown interned string 0x6375_AED7")
            }
            0x6473_C247 => {
                trace!("Unknown interned string 0x6473_C247");
                Ok("Unknown interned string 0x6473_C247")
            }
            0x64CA_7347 => {
                trace!("Unknown interned string 0x64CA_7347");
                Ok("Unknown interned string 0x64CA_7347")
            }
            0x6561_72E9 => {
                trace!("Unknown interned string 0x6561_72E9");
                Ok("Unknown interned string 0x6561_72E9")
            }
            0x657A_CC79 => {
                trace!("Unknown interned string 0x657A_CC79");
                Ok("Unknown interned string 0x657A_CC79")
            }
            0x67F7_3F07 => {
                trace!("Unknown interned string 0x67F7_3F07");
                Ok("Unknown interned string 0x67F7_3F07")
            }
            0x6923_3D7C => {
                trace!("Unknown interned string 0x6923_3D7C");
                Ok("Unknown interned string 0x6923_3D7C")
            }
            0x69DC_356C => {
                trace!("Unknown interned string 0x69DC_356C");
                Ok("Unknown interned string 0x69DC_356C")
            }
            0x6AA9_A89A => {
                trace!("Unknown interned string 0x6AA9_A89A");
                Ok("Unknown interned string 0x6AA9_A89A")
            }
            0x6B07_B9D7 => {
                trace!("Unknown interned string 0x6B07_B9D7");
                Ok("Unknown interned string 0x6B07_B9D7")
            }
            0x6B83_6FC0 => {
                trace!("Unknown interned string 0x6B83_6FC0");
                Ok("Unknown interned string 0x6B83_6FC0")
            }
            0x6BE0_62E0 => {
                trace!("Unknown interned string 0x6BE0_62E0");
                Ok("Unknown interned string 0x6BE0_62E0")
            }
            0x6C1F_2410 => {
                trace!("Unknown interned string 0x6C1F_2410");
                Ok("Unknown interned string 0x6C1F_2410")
            }
            0x6D5F_2C3A => {
                trace!("Unknown interned string 0x6D5F_2C3A");
                Ok("Unknown interned string 0x6D5F_2C3A")
            }
            0x6D94_5134 => {
                trace!("Unknown interned string 0x6D94_5134");
                Ok("Unknown interned string 0x6D94_5134")
            }
            0x6F39_3793 => {
                trace!("Unknown interned string 0x6F39_3793");
                Ok("Unknown interned string 0x6F39_3793")
            }
            0x7014_DA8D => {
                trace!("Unknown interned string 0x7014_DA8D");
                Ok("Unknown interned string 0x7014_DA8D")
            }
            0x70E8_05A9 => {
                trace!("Unknown interned string 0x70E8_05A9");
                Ok("Unknown interned string 0x70E8_05A9")
            }
            0x71F7_3DB6 => {
                trace!("Unknown interned string 0x71F7_3DB6");
                Ok("Unknown interned string 0x71F7_3DB6")
            }
            0x7344_2688 => {
                trace!("Unknown interned string 0x7344_2688");
                Ok("Unknown interned string 0x7344_2688")
            }
            0x73EC_0074 => {
                trace!("Unknown interned string 0x73EC_0074");
                Ok("Unknown interned string 0x73EC_0074")
            }
            0x772E_B30B => {
                trace!("Unknown interned string 0x772E_B30B");
                Ok("Unknown interned string 0x772E_B30B")
            }
            0x773C_7F72 => {
                trace!("Unknown interned string 0x773C_7F72");
                Ok("Unknown interned string 0x773C_7F72")
            }
            0x7806_19DD => {
                trace!("Unknown interned string 0x7806_19DD");
                Ok("Unknown interned string 0x7806_19DD")
            }
            0x7825_D31E => {
                trace!("Unknown interned string 0x7825_D31E");
                Ok("Unknown interned string 0x7825_D31E")
            }
            0x7BD2_C45F => {
                trace!("Unknown interned string 0x7BD2_C45F");
                Ok("Unknown interned string 0x7BD2_C45F")
            }
            0x7CB6_39D4 => {
                trace!("Unknown interned string 0x7CB6_39D4");
                Ok("Unknown interned string 0x7CB6_39D4")
            }
            0x7CD1_8923 => {
                trace!("Unknown interned string 0x7CD1_8923");
                Ok("Unknown interned string 0x7CD1_8923")
            }
            0x7CE8_6D5D => {
                trace!("Unknown interned string 0x7CE8_6D5D");
                Ok("Unknown interned string 0x7CE8_6D5D")
            }
            0x7E45_970E => {
                trace!("Unknown interned string 0x7E45_970E");
                Ok("Unknown interned string 0x7E45_970E")
            }
            0x7F33_01E4 => {
                trace!("Unknown interned string 0x7F33_01E4");
                Ok("Unknown interned string 0x7F33_01E4")
            }
            0x812E_32CF => {
                trace!("Unknown interned string 0x812E_32CF");
                Ok("Unknown interned string 0x812E_32CF")
            }
            0x8160_5A20 => {
                trace!("Unknown interned string 0x8160_5A20");
                Ok("Unknown interned string 0x8160_5A20")
            }
            0x8258_7375 => {
                trace!("Unknown interned string 0x8258_7375");
                Ok("Unknown interned string 0x8258_7375")
            }
            0x835D_4CB0 => {
                trace!("Unknown interned string 0x835D_4CB0");
                Ok("Unknown interned string 0x835D_4CB0")
            }
            0x8456_BB04 => {
                trace!("Unknown interned string 0x8456_BB04");
                Ok("Unknown interned string 0x8456_BB04")
            }
            0x8491_0CBA => {
                trace!("Unknown interned string 0x8491_0CBA");
                Ok("Unknown interned string 0x8491_0CBA")
            }
            0x854F_E621 => {
                trace!("Unknown interned string 0x854F_E621");
                Ok("Unknown interned string 0x854F_E621")
            }
            0x857B_487A => {
                trace!("Unknown interned string 0x857B_487A");
                Ok("Unknown interned string 0x857B_487A")
            }
            0x857C_FE2C => {
                trace!("Unknown interned string 0x857C_FE2C");
                Ok("Unknown interned string 0x857C_FE2C")
            }
            0x8730_0A11 => {
                trace!("Unknown interned string 0x8730_0A11");
                Ok("Unknown interned string 0x8730_0A11")
            }
            0x87E4_6BD9 => {
                trace!("Unknown interned string 0x87E4_6BD9");
                Ok("Unknown interned string 0x87E4_6BD9")
            }
            0x87EC_2E4A => {
                trace!("Unknown interned string 0x87EC_2E4A");
                Ok("Unknown interned string 0x87EC_2E4A")
            }
            0x898A_6B0B => {
                trace!("Unknown interned string 0x898A_6B0B");
                Ok("Unknown interned string 0x898A_6B0B")
            }
            0x8990_8AFD => {
                trace!("Unknown interned string 0x8990_8AFD");
                Ok("Unknown interned string 0x8990_8AFD")
            }
            0x899A_2E8C => {
                trace!("Unknown interned string 0x899A_2E8C");
                Ok("Unknown interned string 0x899A_2E8C")
            }
            0x89D0_D4ED => {
                trace!("Unknown interned string 0x89D0_D4ED");
                Ok("Unknown interned string 0x89D0_D4ED")
            }
            0x8B38_74D3 => {
                trace!("Unknown interned string 0x8B38_74D3");
                Ok("Unknown interned string 0x8B38_74D3")
            }
            0x8C66_5ED2 => {
                trace!("Unknown interned string 0x8C66_5ED2");
                Ok("Unknown interned string 0x8C66_5ED2")
            }
            0x8E85_CAFE => {
                trace!("Unknown interned string 0x8E85_CAFE");
                Ok("Unknown interned string 0x8E85_CAFE")
            }
            0x8ED5_864E => {
                trace!("Unknown interned string 0x8ED5_864E");
                Ok("Unknown interned string 0x8ED5_864E")
            }
            0x8F40_0195 => {
                trace!("Unknown interned string 0x8F40_0195");
                Ok("Unknown interned string 0x8F40_0195")
            }
            0x9146_8385 => {
                trace!("Unknown interned string 0x9146_8385");
                Ok("Unknown interned string 0x9146_8385")
            }
            0x9169_7C77 => {
                trace!("Unknown interned string 0x9169_7C77");
                Ok("Unknown interned string 0x9169_7C77")
            }
            0x916B_F2C3 => {
                trace!("Unknown interned string 0x916B_F2C3");
                Ok("Unknown interned string 0x916B_F2C3")
            }
            0x91DB_09A4 => {
                trace!("Unknown interned string 0x91DB_09A4");
                Ok("Unknown interned string 0x91DB_09A4")
            }
            0x929F_849C => {
                trace!("Unknown interned string 0x929F_849C");
                Ok("Unknown interned string 0x929F_849C")
            }
            0x95B3_60A6 => {
                trace!("Unknown interned string 0x95B3_60A6");
                Ok("Unknown interned string 0x95B3_60A6")
            }
            0x969E_313D => {
                trace!("Unknown interned string 0x969E_313D");
                Ok("Unknown interned string 0x969E_313D")
            }
            0x976F_6164 => {
                trace!("Unknown interned string 0x976F_6164");
                Ok("Unknown interned string 0x976F_6164")
            }
            0x9B44_1603 => {
                trace!("Unknown interned string 0x9B44_1603");
                Ok("Unknown interned string 0x9B44_1603")
            }
            0x9BCA_7D1B => {
                trace!("Unknown interned string 0x9BCA_7D1B");
                Ok("Unknown interned string 0x9BCA_7D1B")
            }
            0x9BD8_A2F4 => {
                trace!("Unknown interned string 0x9BD8_A2F4");
                Ok("Unknown interned string 0x9BD8_A2F4")
            }
            0x9CC6_16F4 => {
                trace!("Unknown interned string 0x9CC6_16F4");
                Ok("Unknown interned string 0x9CC6_16F4")
            }
            0x9D08_2985 => {
                trace!("Unknown interned string 0x9D08_2985");
                Ok("Unknown interned string 0x9D08_2985")
            }
            0x9D5D_4366 => {
                trace!("Unknown interned string 0x9D5D_4366");
                Ok("Unknown interned string 0x9D5D_4366")
            }
            0x9D74_6616 => {
                trace!("Unknown interned string 0x9D74_6616");
                Ok("Unknown interned string 0x9D74_6616")
            }
            0x9DD1_08C9 => {
                trace!("Unknown interned string 0x9DD1_08C9");
                Ok("Unknown interned string 0x9DD1_08C9")
            }
            0x9F3F_1BC5 => {
                trace!("Unknown interned string 0x9F3F_1BC5");
                Ok("Unknown interned string 0x9F3F_1BC5")
            }
            0x9F4C_DBBE => {
                trace!("Unknown interned string 0x9F4C_DBBE");
                Ok("Unknown interned string 0x9F4C_DBBE")
            }
            0x9F8F_8E52 => {
                trace!("Unknown interned string 0x9F8F_8E52");
                Ok("Unknown interned string 0x9F8F_8E52")
            }
            0xA026_4F16 => {
                trace!("Unknown interned string 0xA026_4F16");
                Ok("Unknown interned string 0xA026_4F16")
            }
            0xA07A_6D10 => {
                trace!("Unknown interned string 0xA07A_6D10");
                Ok("Unknown interned string 0xA07A_6D10")
            }
            0xA0FD_98CC => {
                trace!("Unknown interned string 0xA0FD_98CC");
                Ok("Unknown interned string 0xA0FD_98CC")
            }
            0xA1E4_D3F6 => {
                trace!("Unknown interned string 0xA1E4_D3F6");
                Ok("Unknown interned string 0xA1E4_D3F6")
            }
            0xA2AA_2F92 => {
                trace!("Unknown interned string 0xA2AA_2F92");
                Ok("Unknown interned string 0xA2AA_2F92")
            }
            0xA355_7351 => {
                trace!("Unknown interned string 0xA355_7351");
                Ok("Unknown interned string 0xA355_7351 (*.anm)")
            }
            0xA383_F721 => {
                trace!("Unknown interned string 0xA383_F721");
                Ok("Unknown interned string 0xA383_F721")
            }
            0xA3C7_88B3 => {
                trace!("Unknown interned string 0xA3C7_88B3");
                Ok("Unknown interned string 0xA3C7_88B3")
            }
            0xA3F3_DF79 => {
                trace!("Unknown interned string 0xA3F3_DF79");
                Ok("Unknown interned string 0xA3F3_DF79")
            }
            0xA442_F7E9 => {
                trace!("Unknown interned string 0xA442_F7E9");
                Ok("Unknown interned string 0xA442_F7E9")
            }
            0xA4F8_7AEF => {
                trace!("Unknown interned string 0xA4F8_7AEF");
                Ok("Unknown interned string 0xA4F8_7AEF")
            }
            0xA713_BE0A => {
                trace!("Unknown interned string 0xA713_BE0A");
                Ok("Unknown interned string 0xA713_BE0A")
            }
            0xA750_DE16 => {
                trace!("Unknown interned string 0xA750_DE16");
                Ok("Unknown interned string 0xA750_DE16")
            }
            0xA75B_5FF1 => {
                trace!("Unknown interned string 0xA75B_5FF1");
                Ok("Unknown interned string 0xA75B_5FF1")
            }
            0xA772_6921 => {
                trace!("Unknown interned string 0xA772_6921");
                Ok("Unknown interned string 0xA772_6921")
            }
            0xA7CF_6F9B => {
                trace!("Unknown interned string 0xA7CF_6F9B");
                Ok("Unknown interned string 0xA7CF_6F9B")
            }
            0xA87C_ECF8 => {
                trace!("Unknown interned string 0xA87C_ECF8");
                Ok("Unknown interned string 0xA87C_ECF8")
            }
            0xA8B7_C2DC => {
                trace!("Unknown interned string 0xA8B7_C2DC");
                Ok("Unknown interned string 0xA8B7_C2DC")
            }
            0xA948_C400 => {
                trace!("Unknown interned string 0xA948_C400");
                Ok("Unknown interned string 0xA948_C400")
            }
            0xA95D_355C => {
                trace!("Unknown interned string 0xA95D_355C");
                Ok("Unknown interned string 0xA95D_355C")
            }
            0xA9D7_06D1 => {
                trace!("Unknown interned string 0xA9D7_06D1");
                Ok("Unknown interned string 0xA9D7_06D1")
            }
            0xA9D7_8C4E => {
                trace!("Unknown interned string 0xA9D7_8C4E");
                Ok("Unknown interned string 0xA9D7_8C4E")
            }
            0xAA3D_EC61 => {
                trace!("Unknown interned string 0xAA3D_EC61");
                Ok("Unknown interned string 0xAA3D_EC61")
            }
            0xAD5F_C3AE => {
                trace!("Unknown interned string 0xAD5F_C3AE");
                Ok("Unknown interned string 0xAD5F_C3AE")
            }
            0xADD2_B542 => {
                trace!("Unknown interned string 0xADD2_B542");
                Ok("Unknown interned string 0xADD2_B542")
            }
            0xAE2A_8559 => {
                trace!("Unknown interned string 0xAE2A_8559");
                Ok("Unknown interned string 0xAE2A_8559")
            }
            0xAEDF_261B => {
                trace!("Unknown interned string 0xAEDF_261B");
                Ok("Unknown interned string 0xAEDF_261B")
            }
            0xB034_23B9 => {
                trace!("Unknown interned string 0xB034_23B9");
                Ok("Unknown interned string 0xB034_23B9")
            }
            0xB0F4_CADB => {
                trace!("Unknown interned string 0xB0F4_CADB");
                Ok("Unknown interned string 0xB0F4_CADB")
            }
            0xB2AD_6FE0 => {
                trace!("Unknown interned string 0xB2AD_6FE0");
                Ok("Unknown interned string 0xB2AD_6FE0")
            }
            0xB2E3_6991 => {
                trace!("Unknown interned string 0xB2E3_6991");
                Ok("Unknown interned string 0xB2E3_6991")
            }
            0xB313_8AD9 => {
                trace!("Unknown interned string 0xB313_8AD9");
                Ok("Unknown interned string 0xB313_8AD9")
            }
            0xB326_F58A => {
                trace!("Unknown interned string 0xB326_F58A");
                Ok("Unknown interned string 0xB326_F58A")
            }
            0xB33D_B95E => {
                trace!("Unknown interned string 0xB33D_B95E");
                Ok("Unknown interned string 0xB33D_B95E")
            }
            0xB376_7FC2 => {
                trace!("Unknown interned string 0xB376_7FC2");
                Ok("Unknown interned string 0xB376_7FC2")
            }
            0xB3A0_433C => {
                trace!("Unknown interned string 0xB3A0_433C");
                Ok("Unknown interned string 0xB3A0_433C")
            }
            0xB41F_48EE => {
                trace!("Unknown interned string 0xB41F_48EE");
                Ok("Unknown interned string 0xB41F_48EE")
            }
            0xB524_BBF9 => {
                trace!("Unknown interned string 0xB524_BBF9");
                Ok("Unknown interned string 0xB524_BBF9")
            }
            0xB603_FC52 => {
                trace!("Unknown interned string 0xB603_FC52");
                Ok("Unknown interned string 0xB603_FC52")
            }
            0xB6C4_FECA => {
                trace!("Unknown interned string 0xB6C4_FECA");
                Ok("Unknown interned string 0xB6C4_FECA")
            }
            0xB8F7_0640 => {
                trace!("Unknown interned string 0xB8F7_0640");
                Ok("Unknown interned string 0xB8F7_0640")
            }
            0xB9AE_3C0F => {
                trace!("Unknown interned string 0xB9AE_3C0F");
                Ok("Unknown interned string 0xB9AE_3C0F")
            }
            0xB9D0_586B => {
                trace!("Unknown interned string 0xB9D0_586B");
                Ok("Unknown interned string 0xB9D0_586B")
            }
            0xBAF7_40B2 => {
                trace!("Unknown interned string 0xBAF7_40B2");
                Ok("Unknown interned string 0xBAF7_40B2")
            }
            0xBB9E_7C01 => {
                trace!("Unknown interned string 0xBB9E_7C01");
                Ok("Unknown interned string 0xBB9E_7C01")
            }
            0xBBC8_BEAB => {
                trace!("Unknown interned string 0xBBC8_BEAB");
                Ok("Unknown interned string 0xBBC8_BEAB")
            }
            0xBC5A_FCDF => {
                trace!("Unknown interned string 0xBC5A_FCDF");
                Ok("Unknown interned string 0xBC5A_FCDF")
            }
            0xBCD5_5F87 => {
                trace!("Unknown interned string 0xBCD5_5F87");
                Ok("Unknown interned string 0xBCD5_5F87")
            }
            0xBF1D_6046 => {
                trace!("Unknown interned string 0xBF1D_6046");
                Ok("Unknown interned string 0xBF1D_6046")
            }
            0xBF6F_B816 => {
                trace!("Unknown interned string 0xBF6F_B816");
                Ok("Unknown interned string 0xBF6F_B816")
            }
            0xBF9C_0B8D => {
                trace!("Unknown interned string 0xBF9C_0B8D");
                Ok("Unknown interned string 0xBF9C_0B8D")
            }
            0xC0B5_B54D => {
                trace!("Unknown interned string 0xC0B5_B54D");
                Ok("Unknown interned string 0xC0B5_B54D")
            }
            0xC165_B570 => {
                trace!("Unknown interned string 0xC165_B570");
                Ok("Unknown interned string 0xC165_B570")
            }
            0xC278_4760 => {
                trace!("Unknown interned string 0xC278_4760");
                Ok("Unknown interned string 0xC278_4760")
            }
            0xC27D_230D => {
                trace!("Unknown interned string 0xC27D_230D");
                Ok("Unknown interned string 0xC27D_230D")
            }
            0xC2CC_0B69 => {
                trace!("Unknown interned string 0xC2CC_0B69");
                Ok("Unknown interned string 0xC2CC_0B69")
            }
            0xC366_DA37 => {
                trace!("Unknown interned string 0xC366_DA37");
                Ok("Unknown interned string 0xC366_DA37")
            }
            0xC41B_DAA0 => {
                trace!("Unknown interned string 0xC41B_DAA0");
                Ok("Unknown interned string 0xC41B_DAA0")
            }
            0xC448_CE6F => {
                trace!("Unknown interned string 0xC448_CE6F");
                Ok("Unknown interned string 0xC448_CE6F")
            }
            0xC5D4_9BBB => {
                trace!("Unknown interned string 0xC5D4_9BBB");
                Ok("Unknown interned string 0xC5D4_9BBB")
            }
            0xC5F0_C6C2 => {
                trace!("Unknown interned string 0xC5F0_C6C2");
                Ok("Unknown interned string 0xC5F0_C6C2")
            }
            0xC7F7_F243 => {
                trace!("Unknown interned string 0xC7F7_F243");
                Ok("Unknown interned string 0xC7F7_F243")
            }
            0xC8A7_75E3 => {
                trace!("Unknown interned string 0xC8A7_75E3");
                Ok("Unknown interned string 0xC8A7_75E3")
            }
            0xCB22_8C0C => {
                trace!("Unknown interned string 0xCB22_8C0C");
                Ok("Unknown interned string 0xCB22_8C0C")
            }
            0xCB3E_767F => {
                trace!("Unknown interned string 0xCB3E_767F");
                Ok("Unknown interned string 0xCB3E_767F")
            }
            0xCB4D_EDC4 => {
                trace!("Unknown interned string 0xCB4D_EDC4");
                Ok("Unknown interned string 0xCB4D_EDC4")
            }
            0xCB96_9FD7 => {
                trace!("Unknown interned string 0xCB96_9FD7");
                Ok("Unknown interned string 0xCB96_9FD7")
            }
            0xCBC5_3A01 => {
                trace!("Unknown interned string 0xCBC5_3A01");
                Ok("Unknown interned string 0xCBC5_3A01")
            }
            0xCC67_D496 => {
                trace!("Unknown interned string 0xCC67_D496");
                Ok("Unknown interned string 0xCC67_D496")
            }
            0xCCC1_ECF5 => {
                trace!("Unknown interned string 0xCCC1_ECF5");
                Ok("Unknown interned string 0xCCC1_ECF5")
            }
            0xCCE6_F626 => {
                trace!("Unknown interned string 0xCCE6_F626");
                Ok("Unknown interned string 0xCCE6_F626")
            }
            0xCD09_DCF3 => {
                trace!("Unknown interned string 0xCD09_DCF3");
                Ok("Unknown interned string 0xCD09_DCF3")
            }
            0xCFB9_CC44 => {
                trace!("Unknown interned string 0xCFB9_CC44");
                Ok("Unknown interned string 0xCFB9_CC44")
            }
            0xCFFA_705D => {
                trace!("Unknown interned string 0xCFFA_705D");
                Ok("Unknown interned string 0xCFFA_705D")
            }
            0xD0A8_C084 => {
                trace!("Unknown interned string 0xD0A8_C084");
                Ok("Unknown interned string 0xD0A8_C084")
            }
            0xD101_48FF => {
                trace!("Unknown interned string 0xD101_48FF");
                Ok("Unknown interned string 0xD101_48FF")
            }
            0xD290_5B61 => {
                trace!("Unknown interned string 0xD290_5B61");
                Ok("Unknown interned string 0xD290_5B61")
            }
            0xD2FB_E1BC => {
                trace!("Unknown interned string 0xD2FB_E1BC");
                Ok("Unknown interned string 0xD2FB_E1BC")
            }
            0xD30F_50E5 => {
                trace!("Unknown interned string 0xD30F_50E5");
                Ok("Unknown interned string 0xD30F_50E5")
            }
            0xD584_7906 => {
                trace!("Unknown interned string 0xD584_7906");
                Ok("Unknown interned string 0xD584_7906")
            }
            0xD6E1_1CF3 => {
                trace!("Unknown interned string 0xD6E1_1CF3");
                Ok("Unknown interned string 0xD6E1_1CF3")
            }
            0xD7F6_0D9A => {
                trace!("Unknown interned string 0xD7F6_0D9A");
                Ok("Unknown interned string 0xD7F6_0D9A")
            }
            0xD807_F1C4 => {
                trace!("Unknown interned string 0xD807_F1C4");
                Ok("Unknown interned string 0xD807_F1C4")
            }
            0xD82A_C796 => {
                trace!("Unknown interned string 0xD82A_C796");
                Ok("Unknown interned string 0xD82A_C796")
            }
            0xD8C6_52E6 => {
                trace!("Unknown interned string 0xD8C6_52E6");
                Ok("Unknown interned string 0xD8C6_52E6")
            }
            0xD8EA_5A2C => {
                trace!("Unknown interned string 0xD8EA_5A2C");
                Ok("Unknown interned string 0xD8EA_5A2C")
            }
            0xD92E_DDF0 => {
                trace!("Unknown interned string 0xD92E_DDF0");
                Ok("Unknown interned string 0xD92E_DDF0")
            }
            0xD9AA_B606 => {
                trace!("Unknown interned string 0xD9AA_B606");
                Ok("Unknown interned string 0xD9AA_B606")
            }
            0xDA2B_BE0A => {
                trace!("Unknown interned string 0xDA2B_BE0A");
                Ok("Unknown interned string 0xDA2B_BE0A")
            }
            0xDA42_3EAA => {
                trace!("Unknown interned string 0xDA42_3EAA");
                Ok("Unknown interned string 0xDA42_3EAA")
            }
            0xDA9B_83B0 => {
                trace!("Unknown interned string 0xDA9B_83B0");
                Ok("Unknown interned string 0xDA9B_83B0")
            }
            0xDC14_AD39 => {
                trace!("Unknown interned string 0xDC14_AD39");
                Ok("Unknown interned string 0xDC14_AD39")
            }
            0xDC6E_4E0C => {
                trace!("Unknown interned string 0xDC6E_4E0C");
                Ok("Unknown interned string 0xDC6E_4E0C")
            }
            0xDD41_05C4 => {
                trace!("Unknown interned string 0xDD41_05C4");
                Ok("Unknown interned string 0xDD41_05C4")
            }
            0xDD48_3F29 => {
                trace!("Unknown interned string 0xDD48_3F29");
                Ok("Unknown interned string 0xDD48_3F29")
            }
            0xDE67_C327 => {
                trace!("Unknown interned string 0xDE67_C327");
                Ok("Unknown interned string 0xDE67_C327")
            }
            0xDE6C_B88E => {
                trace!("Unknown interned string 0xDE6C_B88E");
                Ok("Unknown interned string 0xDE6C_B88E")
            }
            0xDEE7_2D83 => {
                trace!("Unknown interned string 0xDEE7_2D83");
                Ok("Unknown interned string 0xDEE7_2D83")
            }
            0xDF3A_57E1 => {
                trace!("Unknown interned string 0xDF3A_57E1");
                Ok("Unknown interned string 0xDF3A_57E1")
            }
            0xDF4B_E326 => {
                trace!("Unknown interned string 0xDF4B_E326");
                Ok("Unknown interned string 0xDF4B_E326")
            }
            0xE007_553F => {
                trace!("Unknown interned string 0xE007_553F");
                Ok("Unknown interned string 0xE007_553F")
            }
            0xE05D_183B => {
                trace!("Unknown interned string 0xE05D_183B");
                Ok("Unknown interned string 0xE05D_183B")
            }
            0xE179_6C67 => {
                trace!("Unknown interned string 0xE179_6C67");
                Ok("Unknown interned string 0xE179_6C67")
            }
            0xE1AE_A7A3 => {
                trace!("Unknown interned string 0xE1AE_A7A3");
                Ok("Unknown interned string 0xE1AE_A7A3")
            }
            0xE2E1_F8A6 => {
                trace!("Unknown interned string 0xE2E1_F8A6");
                Ok("Unknown interned string 0xE2E1_F8A6")
            }
            0xE2E9_2D3B => {
                trace!("Unknown interned string 0xE2E9_2D3B");
                Ok("Unknown interned string 0xE2E9_2D3B")
            }
            0xE339_88C0 => {
                trace!("Unknown interned string 0xE339_88C0");
                Ok("Unknown interned string 0xE339_88C0")
            }
            0xE42C_3B10 => {
                trace!("Unknown interned string 0xE42C_3B10");
                Ok("Unknown interned string 0xE42C_3B10")
            }
            0xE46C_4131 => {
                trace!("Unknown interned string 0xE46C_4131");
                Ok("Unknown interned string 0xE46C_4131")
            }
            0xE617_3783 => {
                trace!("Unknown interned string 0xE617_3783");
                Ok("Unknown interned string 0xE617_3783")
            }
            0xE71A_BB71 => {
                trace!("Unknown interned string 0xE71A_BB71");
                Ok("Unknown interned string 0xE71A_BB71")
            }
            0xE758_9E57 => {
                trace!("Unknown interned string 0xE758_9E57");
                Ok("Unknown interned string 0xE758_9E57")
            }
            0xE881_8A15 => {
                trace!("Unknown interned string 0xE881_8A15");
                Ok("Unknown interned string 0xE881_8A15")
            }
            0xE8C2_7753 => {
                trace!("Unknown interned string 0xE8C2_7753");
                Ok("Unknown interned string 0xE8C2_7753")
            }
            0xEA90_77D6 => {
                trace!("Unknown interned string 0xEA90_77D6");
                Ok("Unknown interned string 0xEA90_77D6")
            }
            0xEC3B_62E6 => {
                trace!("Unknown interned string 0xEC3B_62E6");
                Ok("Unknown interned string 0xEC3B_62E6")
            }
            0xEC6A_3CFE => {
                trace!("Unknown interned string 0xEC6A_3CFE");
                Ok("Unknown interned string 0xEC6A_3CFE")
            }
            0xEF51_B8FD => {
                trace!("Unknown interned string 0xEF51_B8FD");
                Ok("Unknown interned string 0xEF51_B8FD")
            }
            0xEFF1_F044 => {
                trace!("Unknown interned string 0xEFF1_F044");
                Ok("Unknown interned string 0xEFF1_F044")
            }
            0xEFF4_66D6 => {
                trace!("Unknown interned string 0xEFF4_66D6");
                Ok("Unknown interned string 0xEFF4_66D6")
            }
            0xF040_7152 => {
                trace!("Unknown interned string 0xF040_7152");
                Ok("Unknown interned string 0xF040_7152")
            }
            0xF15B_40B1 => {
                trace!("Unknown interned string 0xF15B_40B1");
                Ok("Unknown interned string 0xF15B_40B1")
            }
            0xF2C1_348F => {
                trace!("Unknown interned string 0xF2C1_348F");
                Ok("Unknown interned string 0xF2C1_348F")
            }
            0xF3C1_CC2A => {
                trace!("Unknown interned string 0xF3C1_CC2A");
                Ok("Unknown interned string 0xF3C1_CC2A")
            }
            0xF402_2D50 => {
                trace!("Unknown interned string 0xF402_2D50");
                Ok("Unknown interned string 0xF402_2D50")
            }
            0xF447_E18A => {
                trace!("Unknown interned string 0xF447_E18A");
                Ok("Unknown interned string 0xF447_E18A")
            }
            0xF4DD_E9AB => {
                trace!("Unknown interned string 0xF4DD_E9AB");
                Ok("Unknown interned string 0xF4DD_E9AB")
            }
            0xF64A_F91E => {
                trace!("Unknown interned string 0xF64A_F91E");
                Ok("Unknown interned string 0xF64A_F91E")
            }
            0xF779_C517 => {
                trace!("Unknown interned string 0xF779_C517");
                Ok("Unknown interned string 0xF779_C517")
            }
            0xF955_2091 => {
                trace!("Unknown interned string 0xF955_2091");
                Ok("Unknown interned string 0xF955_2091")
            }
            0xF975_E351 => {
                trace!("Unknown interned string 0xF975_E351");
                Ok("Unknown interned string 0xF975_E351")
            }
            0xF9C4_3771 => {
                trace!("Unknown interned string 0xF9C4_3771");
                Ok("Unknown interned string 0xF9C4_3771")
            }
            0xFA0D_3426 => {
                trace!("Unknown interned string 0xFA0D_3426");
                Ok("Unknown interned string 0xFA0D_3426")
            }
            0xFAA4_E27B => {
                trace!("Unknown interned string 0xFAA4_E27B");
                Ok("Unknown interned string 0xFAA4_E27B")
            }
            0xFAFA_6E2D => {
                trace!("Unknown interned string 0xFAFA_6E2D");
                Ok("Unknown interned string 0xFAFA_6E2D")
            }
            0xFB57_228E => {
                trace!("Unknown interned string 0xFB57_228E");
                Ok("Unknown interned string 0xFB57_228E")
            }
            0xFB62_2B45 => {
                trace!("Unknown interned string 0xFB62_2B45");
                Ok("Unknown interned string 0xFB62_2B45")
            }
            0xFB68_AA9F => {
                trace!("Unknown interned string 0xFB68_AA9F");
                Ok("Unknown interned string 0xFB68_AA9F")
            }
            0xFC75_8052 => {
                trace!("Unknown interned string 0xFC75_8052");
                Ok("Unknown interned string 0xFC75_8052")
            }
            0xFCC7_CD74 => {
                trace!("Unknown interned string 0xFCC7_CD74");
                Ok("Unknown interned string 0xFCC7_CD74")
            }
            0xFCD2_F024 => {
                trace!("Unknown interned string 0xFCD2_F024");
                Ok("Unknown interned string 0xFCD2_F024")
            }
            0xFDF3_9390 => {
                trace!("Unknown interned string 0xFDF3_9390");
                Ok("Unknown interned string 0xFDF3_9390")
            }
            0xFF3B_CE1C => {
                trace!("Unknown interned string 0xFF3B_CE1C");
                Ok("Unknown interned string 0xFF3B_CE1C")
            }
            0xFF45_AAA2 => {
                trace!("Unknown interned string 0xFF45_AAA2");
                Ok("Unknown interned string 0xFF45_AAA2")
            }
            0xFF54_ED3D => {
                trace!("Unknown interned string 0xFF54_ED3D");
                Ok("Unknown interned string 0xFF54_ED3D")
            }
            _ => Err(ReadError::custom(format!(
                "Unknown interned string id: 0x{string_id:08x}"
            ))),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UniqueGameId {
    pub game: Game,
    pub platform: Platform,
    pub id: u32,
}

impl UniqueGameId {
    pub const WIIU2015: Self = Self {
        game: Game::JustDance2015,
        platform: Platform::WiiU,
        id: 0xC563_9F58,
    };
    pub const WIIU2016: Self = Self {
        game: Game::JustDance2016,
        platform: Platform::WiiU,
        id: 0xF9D9_B22B,
    };
    pub const WIIU2017: Self = Self {
        game: Game::JustDance2017,
        platform: Platform::WiiU,
        id: 0x04A2_5379,
    };
    pub const WIN2017: Self = Self {
        game: Game::JustDance2017,
        platform: Platform::Win,
        id: 0x1D3A_4C30,
    };
    pub const NX2017: Self = Self {
        game: Game::JustDance2017,
        platform: Platform::Nx,
        id: 0x32F3_512A,
    };
    pub const NX2018: Self = Self {
        game: Game::JustDance2018,
        platform: Platform::Nx,
        id: 0x032E_71C5,
    };
    pub const NX2019V1: Self = Self {
        game: Game::JustDance2019,
        platform: Platform::Nx,
        id: 0x57A7_053C,
    };
    pub const NX2019V2: Self = Self {
        game: Game::JustDance2019,
        platform: Platform::Nx,
        id: 0xC781_A65B,
    };
    pub const NX2020: Self = Self {
        game: Game::JustDance2020,
        platform: Platform::Nx,
        id: 0x217A_94CE,
    };
    pub const NX_CHINA: Self = Self {
        game: Game::JustDanceChina,
        platform: Platform::Nx,
        id: 0xA155_8F87,
    };
    pub const NX2021: Self = Self {
        game: Game::JustDance2021,
        platform: Platform::Nx,
        id: 0xA4F0_18EE,
    };
    pub const NX2022: Self = Self {
        game: Game::JustDance2022,
        platform: Platform::Nx,
        id: 0x1DDB_2268,
    };
}

impl PartialOrd for UniqueGameId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Order by game first, if games are the same order by ID
        // A newer ID means a newer version of that game
        self.game
            .partial_cmp(&other.game)
            .map(|order| order.then(self.id.cmp(&other.id)))
    }
}

impl Display for UniqueGameId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::write!(f, "{} for {}", self.game, self.platform)
    }
}

impl From<UniqueGameId> for u32 {
    fn from(value: UniqueGameId) -> Self {
        value.id
    }
}

impl TryFrom<u32> for UniqueGameId {
    type Error = ParserError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x1C24_B91A => Ok(Self {
                game: Game::JustDance2014,
                platform: Platform::WiiU,
                id: value,
            }),
            0xC563_9F58 => Ok(Self {
                game: Game::JustDance2015,
                platform: Platform::WiiU,
                id: value,
            }),
            // Base       Update 1      Update 2
            0xDA14_5C61 | 0x8C9D_65E4 | 0xF9D_9B22B => Ok(Self {
                game: Game::JustDance2016,
                platform: Platform::WiiU,
                id: value,
            }),
            0x04A2_5379 => Ok(Self {
                game: Game::JustDance2017,
                platform: Platform::WiiU,
                id: value,
            }),
            0x1D3A_4C30 => Ok(Self {
                game: Game::JustDance2017,
                platform: Platform::Win,
                id: value,
            }),
            0x415E_6D8C | 0x32F3_512A => Ok(Self {
                game: Game::JustDance2017,
                platform: Platform::Nx,
                id: value,
            }),
            0x032E_71C5 => Ok(Self {
                game: Game::JustDance2018,
                platform: Platform::Nx,
                id: value,
            }),
            0x1F5E_E42F | 0xC781_A65B | 0x57A7_053C => Ok(Self {
                game: Game::JustDance2019,
                platform: Platform::Nx,
                id: value,
            }),
            0x72B4_2FF4 | 0xB292_FD08 | 0x217A_94CE => Ok(Self {
                game: Game::JustDance2020,
                platform: Platform::Nx,
                id: value,
            }),
            0xA155_8F87 => Ok(Self {
                game: Game::JustDanceChina,
                platform: Platform::Nx,
                id: value,
            }),
            0x4C8E_C5C5 => Ok(Self {
                game: Game::JustDance2020,
                platform: Platform::Wii,
                id: value,
            }),
            0xEB5D_504C | 0xA4F0_18EE => Ok(Self {
                game: Game::JustDance2021,
                platform: Platform::Nx,
                id: value,
            }),
            0x1DDB_2268 => Ok(Self {
                game: Game::JustDance2022,
                platform: Platform::Nx,
                id: value,
            }),
            _ => Err(ParserError::custom(format!(
                "Unknown game platform: {value:x}"
            ))),
        }
    }
}

impl BinaryDeserialize<'_> for UniqueGameId {
    type Ctx = bool;
    type Output = Self;

    fn deserialize_at_with(
        reader: &(impl ReadAtExt + ?Sized),
        position: &mut u64,
        lax: bool,
    ) -> Result<Self, ReadError> {
        let value = reader.read_at::<u32be>(position)?;
        let result = Self::try_from(value)
            .map_err(|_| ReadError::custom(format!("Unknown game platform: {value:x}")));
        if result.is_err() && lax {
            warn!("Unknown game platform: {value:x}");
            return Ok(Self {
                game: Game::Unknown,
                platform: Platform::Nx,
                id: value,
            });
        }
        result
    }
}

impl BinarySerialize for UniqueGameId {
    type Ctx = ();
    type Input = Self;

    fn serialize_at_with_ctx(
        input: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        _ctx: (),
    ) -> Result<(), WriteError> {
        writer.write_at::<u32be>(position, input.id)?;
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Serialize, Deserialize, ValueEnum)]
pub enum Game {
    JustDance2014 = 20140,
    JustDance2015 = 20150,
    JustDance2016 = 20160,
    JustDance2017 = 20170,
    JustDance2018 = 20180,
    JustDance2019 = 20190,
    JustDance2020 = 20200,
    JustDanceChina = 20201,
    JustDance2021 = 20210,
    JustDance2022 = 20220,
    Unknown,
}

impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if *self == Self::Unknown || *other == Self::Unknown {
            None
        } else {
            #[allow(
                clippy::as_conversions,
                reason = "the enum values are in the range of 20140-20220 so is always safe"
            )]
            (*self as u32).partial_cmp(&(*other as u32))
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::JustDance2014 => std::write!(f, "Just Dance 2014"),
            Self::JustDance2015 => std::write!(f, "Just Dance 2015"),
            Self::JustDance2016 => std::write!(f, "Just Dance 2016"),
            Self::JustDance2017 => std::write!(f, "Just Dance 2017"),
            Self::JustDance2018 => std::write!(f, "Just Dance 2018"),
            Self::JustDance2019 => std::write!(f, "Just Dance 2019"),
            Self::JustDance2020 => std::write!(f, "Just Dance 2020"),
            Self::JustDanceChina => std::write!(f, "Just Dance China"),
            Self::JustDance2021 => std::write!(f, "Just Dance 2021"),
            Self::JustDance2022 => std::write!(f, "Just Dance 2022"),
            Self::Unknown => std::write!(f, "Unknown"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum Platform {
    Win = 0x0,
    X360 = 0x1,
    Ps4 = 0x3,
    Wii = 0x5,
    WiiU = 0x8,
    Nx = 0xB,
}

impl Ord for Platform {
    #[allow(clippy::match_same_arms, reason = "Clearer this way")]
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Nx, Self::Nx) => Ordering::Equal,
            (Self::Nx, _) => Ordering::Greater,
            (_, Self::Nx) => Ordering::Less,
            (Self::WiiU, Self::WiiU) => Ordering::Equal,
            (Self::WiiU, _) => Ordering::Greater,
            (_, Self::WiiU) => Ordering::Less,
            (Self::Win, Self::Win) => Ordering::Equal,
            (Self::Win, _) => Ordering::Greater,
            (_, Self::Win) => Ordering::Less,
            (Self::Ps4, Self::Ps4) => Ordering::Equal,
            (Self::Ps4, _) => Ordering::Greater,
            (_, Self::Ps4) => Ordering::Less,
            (Self::X360, Self::X360) => Ordering::Equal,
            (Self::X360, _) => Ordering::Greater,
            (_, Self::X360) => Ordering::Less,
            (Self::Wii, Self::Wii) => Ordering::Equal,
        }
    }
}

impl PartialOrd for Platform {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Win => write!(f, "Windows"),
            Self::X360 => std::write!(f, "Xbox 360"),
            Self::Ps4 => std::write!(f, "PlayStation 4"),
            Self::Wii => std::write!(f, "Wii"),
            Self::WiiU => std::write!(f, "Wii U"),
            Self::Nx => std::write!(f, "Switch"),
        }
    }
}

impl TryFrom<u32> for Platform {
    type Error = ParserError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x0 => Ok(Self::Win),
            0x1 => Ok(Self::X360),
            0x3 => Ok(Self::Ps4),
            0x5 => Ok(Self::Wii),
            0x8 => Ok(Self::WiiU),
            0xB => Ok(Self::Nx),
            _ => Err(ParserError::custom(format!("Unknown platform id {value}!"))),
        }
    }
}

impl From<Platform> for u32 {
    #[allow(
        clippy::as_conversions,
        reason = "Platform is repr(u32) thus this is always safe"
    )]
    fn from(value: Platform) -> Self {
        value as Self
    }
}

impl BinaryDeserialize<'_> for Platform {
    type Ctx = ();
    type Output = Self;

    fn deserialize_at_with(
        reader: &'_ (impl ReadAtExt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<Self::Output, ReadError> {
        Self::try_from(reader.read_at::<u32be>(position)?)
            .map_err(|e| ReadError::custom(format!("{e:?}")))
    }
}

impl BinarySerialize for Platform {
    type Ctx = ();
    type Input = Self;

    fn serialize_at_with_ctx(
        input: Self::Input,
        writer: &mut (impl WriteAt + ?Sized),
        position: &mut u64,
        _ctx: Self::Ctx,
    ) -> Result<(), WriteError> {
        writer.write_at::<u32be>(position, u32::from(input))?;
        Ok(())
    }
}

// Calculates the Ubisoft string id for a given string.
//
// Implementation based on the Python implementation by github.com/InvoxiPlayGames
#[must_use]
pub fn string_id(string: &str) -> u32 {
    let bytes = string.as_bytes();
    let mut upper = Vec::with_capacity(bytes.len());
    // Convert lowercase chars to uppercase
    for byte in bytes {
        if *byte >= 0x61 && *byte <= 0x7A {
            upper.push(*byte - 0x20);
        } else {
            upper.push(*byte);
        }
    }
    ubi_crc(&upper)
}

/// Calculates the Ubisoft string id for a given os string.
///
/// Implementation based on the Python implementation by github.com/InvoxiPlayGames
#[must_use]
pub fn os_string_id(string: &OsStr) -> u32 {
    let bytes = string.as_encoded_bytes();
    let mut upper = Vec::with_capacity(bytes.len());
    // Convert lowercase chars to uppercase
    for byte in bytes {
        if *byte >= 0x61 && *byte <= 0x7A {
            upper.push(*byte - 0x20);
        } else {
            upper.push(*byte);
        }
    }
    ubi_crc(&upper)
}

/// Calculates the Ubisoft string id for two strings.
///
/// Implementation based on the Python implementation by github.com/InvoxiPlayGames
#[must_use]
pub fn string_id_2(one: &str, two: &str) -> u32 {
    let bytes_one = one.as_bytes();
    let bytes_two = two.as_bytes();
    let mut upper = Vec::with_capacity(bytes_one.len() + bytes_two.len());
    // Convert lowercase chars to uppercase
    for byte in bytes_one {
        if *byte >= 0x61 && *byte <= 0x7A {
            upper.push(*byte - 0x20);
        } else {
            upper.push(*byte);
        }
    }
    for byte in bytes_two {
        if *byte >= 0x61 && *byte <= 0x7A {
            upper.push(*byte - 0x20);
        } else {
            upper.push(*byte);
        }
    }
    ubi_crc(&upper)
}

#[must_use]
/// Implementation of the UbiArt CRC function
#[allow(
    clippy::as_conversions,
    clippy::cast_possible_truncation,
    reason = "Truncating is wanted"
)]
pub fn ubi_crc(data: &[u8]) -> u32 {
    let length = data.len();
    let mut a: u32 = 0x9E37_79B9;
    let mut b: u32 = 0x9E37_79B9;
    let mut c: u32 = 0;

    let mut pos = 0u64;
    while (pos as usize) + 12 <= length {
        a = a.wrapping_add(
            data.read_at::<u32le>(&mut pos)
                .unwrap_or_else(|_| unreachable!()),
        );
        b = b.wrapping_add(
            data.read_at::<u32le>(&mut pos)
                .unwrap_or_else(|_| unreachable!()),
        );
        c = c.wrapping_add(
            data.read_at::<u32le>(&mut pos)
                .unwrap_or_else(|_| unreachable!()),
        );
        (a, b, c) = shifter(a, b, c);
    }

    let pos = pos as usize;
    c = c.wrapping_add(length as u32);
    let left = length - pos;

    if left > 0 {
        if left >= 11 {
            c = c.wrapping_add(u32::from(data[pos + 10]) << 24);
        }
        if left >= 10 {
            c = c.wrapping_add(u32::from(data[pos + 9]) << 16);
        }
        if left >= 9 {
            c = c.wrapping_add(u32::from(data[pos + 8]) << 8);
        }
        if left >= 8 {
            b = b.wrapping_add(u32::from(data[pos + 7]) << 24);
        }
        if left >= 7 {
            b = b.wrapping_add(u32::from(data[pos + 6]) << 16);
        }
        if left >= 6 {
            b = b.wrapping_add(u32::from(data[pos + 5]) << 8);
        }
        if left >= 5 {
            b = b.wrapping_add(u32::from(data[pos + 4]));
        }
        if left >= 4 {
            a = a.wrapping_add(u32::from(data[pos + 3]) << 24);
        }
        if left >= 3 {
            a = a.wrapping_add(u32::from(data[pos + 2]) << 16);
        }
        if left >= 2 {
            a = a.wrapping_add(u32::from(data[pos + 1]) << 8);
        }
        if left >= 1 {
            a = a.wrapping_add(u32::from(data[pos]));
        }
    }

    (_, _, c) = shifter(a, b, c);
    c
}

/// Shifting implementation for ubicrc
const fn shifter(mut a: u32, mut b: u32, mut c: u32) -> (u32, u32, u32) {
    a = (a.wrapping_sub(b).wrapping_sub(c)) ^ (c >> 0xD);
    b = (b.wrapping_sub(a).wrapping_sub(c)) ^ (a << 0x8);
    c = (c.wrapping_sub(a).wrapping_sub(b)) ^ (b >> 0xD);
    a = (a.wrapping_sub(c).wrapping_sub(b)) ^ (c >> 0xC);
    let d = (b.wrapping_sub(a).wrapping_sub(c)) ^ (a << 0x10);
    c = (c.wrapping_sub(a).wrapping_sub(d)) ^ (d >> 0x5);
    a = (a.wrapping_sub(c).wrapping_sub(d)) ^ (c >> 0x3);
    b = (d.wrapping_sub(a).wrapping_sub(c)) ^ (a << 0xA);
    c = (c.wrapping_sub(a).wrapping_sub(b)) ^ (b >> 0xF);
    (a, b, c)
}

#[allow(clippy::missing_panics_doc)]
#[cfg(test)]
mod tests {
    use dotstar_toolkit_utils::vfs::VirtualPathBuf;
    use hipstr::HipStr;

    use super::{string_id, PathId, SplitPath};

    #[test]
    fn test_string_id() {
        assert_eq!(
            string_id("world/maps/adoreyou/videoscoach/adoreyou.vp9.720.webm"),
            0x45CC_A9CA
        );
    }

    #[test]
    fn test_splitpath_try_from_path() {
        let path = VirtualPathBuf::from("world/maps/adoreyou/videoscoach/adoreyou.vp9.720.webm");
        let sp = SplitPath::try_from(path.as_path()).unwrap();
        assert_eq!(&PathId::from(&sp), &PathId::from(0x45CC_A9CA));
    }

    #[test]
    fn test_splitpath_starts_with() {
        let split_path = SplitPath::new(
            HipStr::borrowed("cache/itf_cooked/nx/"),
            HipStr::borrowed("atlascontainer.ckd"),
        )
        .unwrap();
        assert!(split_path.starts_with("cache"));
        assert!(split_path.starts_with("cache/itf_cooked/nx/"));
        assert!(split_path.starts_with("cache/itf_cooked/nx/atlas"));
        assert!(split_path.starts_with("cache/itf_cooked/nx/atlascontainer.ckd"));
    }
}
