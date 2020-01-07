use ::libc;

pub mod stdlib_h {
    #[inline]

    pub unsafe extern "C" fn atoi(mut __nptr: *const i8) -> i32 {
        return crate::stdlib::strtol(__nptr, 0 as *mut *mut i8, 10) as i32;
    }
}

pub use crate::src::qcommon::q_shared::connstate_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::sfxHandle_t;
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::vec4_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::C2RustUnnamed_0;
pub use crate::src::qcommon::q_shared::Com_sprintf;
pub use crate::src::qcommon::q_shared::Info_ValueForKey;
pub use crate::src::qcommon::q_shared::Q_CleanStr;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::CA_ACTIVE;
pub use crate::src::qcommon::q_shared::CA_AUTHORIZING;
pub use crate::src::qcommon::q_shared::CA_CHALLENGING;
pub use crate::src::qcommon::q_shared::CA_CINEMATIC;
pub use crate::src::qcommon::q_shared::CA_CONNECTED;
pub use crate::src::qcommon::q_shared::CA_CONNECTING;
pub use crate::src::qcommon::q_shared::CA_DISCONNECTED;
pub use crate::src::qcommon::q_shared::CA_LOADING;
pub use crate::src::qcommon::q_shared::CA_PRIMED;
pub use crate::src::qcommon::q_shared::CA_UNINITIALIZED;
pub use crate::src::qcommon::q_shared::EXEC_APPEND;
pub use crate::src::qcommon::q_shared::EXEC_INSERT;
pub use crate::src::qcommon::q_shared::EXEC_NOW;
pub use crate::tr_types_h::glDriverType_t;
pub use crate::tr_types_h::glHardwareType_t;
pub use crate::tr_types_h::glconfig_t;
pub use crate::tr_types_h::textureCompression_t;
pub use crate::tr_types_h::GLDRV_ICD;
pub use crate::tr_types_h::GLDRV_STANDALONE;
pub use crate::tr_types_h::GLDRV_VOODOO;
pub use crate::tr_types_h::GLHW_3DFX_2D3D;
pub use crate::tr_types_h::GLHW_GENERIC;
pub use crate::tr_types_h::GLHW_PERMEDIA2;
pub use crate::tr_types_h::GLHW_RAGEPRO;
pub use crate::tr_types_h::GLHW_RIVA128;
pub use crate::tr_types_h::TC_NONE;
pub use crate::tr_types_h::TC_S3TC;
pub use crate::tr_types_h::TC_S3TC_ARB;
pub use crate::ui_public_h::uiClientState_t;

pub use crate::bg_public_h::GT_1FCTF;
pub use crate::bg_public_h::GT_CTF;
pub use crate::bg_public_h::GT_FFA;
pub use crate::bg_public_h::GT_HARVESTER;
pub use crate::bg_public_h::GT_MAX_GAME_TYPE;
pub use crate::bg_public_h::GT_OBELISK;
pub use crate::bg_public_h::GT_SINGLE_PLAYER;
pub use crate::bg_public_h::GT_TEAM;
pub use crate::bg_public_h::GT_TOURNAMENT;
pub use crate::bg_public_h::TEAM_BLUE;
pub use crate::bg_public_h::TEAM_FREE;
pub use crate::bg_public_h::TEAM_NUM_TEAMS;
pub use crate::bg_public_h::TEAM_RED;
pub use crate::bg_public_h::TEAM_SPECTATOR;
pub use crate::keycodes_h::K_ALT;
pub use crate::keycodes_h::K_AUX1;
pub use crate::keycodes_h::K_AUX10;
pub use crate::keycodes_h::K_AUX11;
pub use crate::keycodes_h::K_AUX12;
pub use crate::keycodes_h::K_AUX13;
pub use crate::keycodes_h::K_AUX14;
pub use crate::keycodes_h::K_AUX15;
pub use crate::keycodes_h::K_AUX16;
pub use crate::keycodes_h::K_AUX2;
pub use crate::keycodes_h::K_AUX3;
pub use crate::keycodes_h::K_AUX4;
pub use crate::keycodes_h::K_AUX5;
pub use crate::keycodes_h::K_AUX6;
pub use crate::keycodes_h::K_AUX7;
pub use crate::keycodes_h::K_AUX8;
pub use crate::keycodes_h::K_AUX9;
pub use crate::keycodes_h::K_BACKSPACE;
pub use crate::keycodes_h::K_BREAK;
pub use crate::keycodes_h::K_CAPSLOCK;
pub use crate::keycodes_h::K_COMMAND;
pub use crate::keycodes_h::K_COMPOSE;
pub use crate::keycodes_h::K_CONSOLE;
pub use crate::keycodes_h::K_CTRL;
pub use crate::keycodes_h::K_DEL;
pub use crate::keycodes_h::K_DOWNARROW;
pub use crate::keycodes_h::K_END;
pub use crate::keycodes_h::K_ENTER;
pub use crate::keycodes_h::K_ESCAPE;
pub use crate::keycodes_h::K_EURO;
pub use crate::keycodes_h::K_F1;
pub use crate::keycodes_h::K_F10;
pub use crate::keycodes_h::K_F11;
pub use crate::keycodes_h::K_F12;
pub use crate::keycodes_h::K_F13;
pub use crate::keycodes_h::K_F14;
pub use crate::keycodes_h::K_F15;
pub use crate::keycodes_h::K_F2;
pub use crate::keycodes_h::K_F3;
pub use crate::keycodes_h::K_F4;
pub use crate::keycodes_h::K_F5;
pub use crate::keycodes_h::K_F6;
pub use crate::keycodes_h::K_F7;
pub use crate::keycodes_h::K_F8;
pub use crate::keycodes_h::K_F9;
pub use crate::keycodes_h::K_HELP;
pub use crate::keycodes_h::K_HOME;
pub use crate::keycodes_h::K_INS;
pub use crate::keycodes_h::K_JOY1;
pub use crate::keycodes_h::K_JOY10;
pub use crate::keycodes_h::K_JOY11;
pub use crate::keycodes_h::K_JOY12;
pub use crate::keycodes_h::K_JOY13;
pub use crate::keycodes_h::K_JOY14;
pub use crate::keycodes_h::K_JOY15;
pub use crate::keycodes_h::K_JOY16;
pub use crate::keycodes_h::K_JOY17;
pub use crate::keycodes_h::K_JOY18;
pub use crate::keycodes_h::K_JOY19;
pub use crate::keycodes_h::K_JOY2;
pub use crate::keycodes_h::K_JOY20;
pub use crate::keycodes_h::K_JOY21;
pub use crate::keycodes_h::K_JOY22;
pub use crate::keycodes_h::K_JOY23;
pub use crate::keycodes_h::K_JOY24;
pub use crate::keycodes_h::K_JOY25;
pub use crate::keycodes_h::K_JOY26;
pub use crate::keycodes_h::K_JOY27;
pub use crate::keycodes_h::K_JOY28;
pub use crate::keycodes_h::K_JOY29;
pub use crate::keycodes_h::K_JOY3;
pub use crate::keycodes_h::K_JOY30;
pub use crate::keycodes_h::K_JOY31;
pub use crate::keycodes_h::K_JOY32;
pub use crate::keycodes_h::K_JOY4;
pub use crate::keycodes_h::K_JOY5;
pub use crate::keycodes_h::K_JOY6;
pub use crate::keycodes_h::K_JOY7;
pub use crate::keycodes_h::K_JOY8;
pub use crate::keycodes_h::K_JOY9;
pub use crate::keycodes_h::K_KP_5;
pub use crate::keycodes_h::K_KP_DEL;
pub use crate::keycodes_h::K_KP_DOWNARROW;
pub use crate::keycodes_h::K_KP_END;
pub use crate::keycodes_h::K_KP_ENTER;
pub use crate::keycodes_h::K_KP_EQUALS;
pub use crate::keycodes_h::K_KP_HOME;
pub use crate::keycodes_h::K_KP_INS;
pub use crate::keycodes_h::K_KP_LEFTARROW;
pub use crate::keycodes_h::K_KP_MINUS;
pub use crate::keycodes_h::K_KP_NUMLOCK;
pub use crate::keycodes_h::K_KP_PGDN;
pub use crate::keycodes_h::K_KP_PGUP;
pub use crate::keycodes_h::K_KP_PLUS;
pub use crate::keycodes_h::K_KP_RIGHTARROW;
pub use crate::keycodes_h::K_KP_SLASH;
pub use crate::keycodes_h::K_KP_STAR;
pub use crate::keycodes_h::K_KP_UPARROW;
pub use crate::keycodes_h::K_LEFTARROW;
pub use crate::keycodes_h::K_MENU;
pub use crate::keycodes_h::K_MODE;
pub use crate::keycodes_h::K_MOUSE1;
pub use crate::keycodes_h::K_MOUSE2;
pub use crate::keycodes_h::K_MOUSE3;
pub use crate::keycodes_h::K_MOUSE4;
pub use crate::keycodes_h::K_MOUSE5;
pub use crate::keycodes_h::K_MWHEELDOWN;
pub use crate::keycodes_h::K_MWHEELUP;
pub use crate::keycodes_h::K_PAD0_A;
pub use crate::keycodes_h::K_PAD0_B;
pub use crate::keycodes_h::K_PAD0_BACK;
pub use crate::keycodes_h::K_PAD0_DPAD_DOWN;
pub use crate::keycodes_h::K_PAD0_DPAD_LEFT;
pub use crate::keycodes_h::K_PAD0_DPAD_RIGHT;
pub use crate::keycodes_h::K_PAD0_DPAD_UP;
pub use crate::keycodes_h::K_PAD0_GUIDE;
pub use crate::keycodes_h::K_PAD0_LEFTSHOULDER;
pub use crate::keycodes_h::K_PAD0_LEFTSTICK_CLICK;
pub use crate::keycodes_h::K_PAD0_LEFTSTICK_DOWN;
pub use crate::keycodes_h::K_PAD0_LEFTSTICK_LEFT;
pub use crate::keycodes_h::K_PAD0_LEFTSTICK_RIGHT;
pub use crate::keycodes_h::K_PAD0_LEFTSTICK_UP;
pub use crate::keycodes_h::K_PAD0_LEFTTRIGGER;
pub use crate::keycodes_h::K_PAD0_RIGHTSHOULDER;
pub use crate::keycodes_h::K_PAD0_RIGHTSTICK_CLICK;
pub use crate::keycodes_h::K_PAD0_RIGHTSTICK_DOWN;
pub use crate::keycodes_h::K_PAD0_RIGHTSTICK_LEFT;
pub use crate::keycodes_h::K_PAD0_RIGHTSTICK_RIGHT;
pub use crate::keycodes_h::K_PAD0_RIGHTSTICK_UP;
pub use crate::keycodes_h::K_PAD0_RIGHTTRIGGER;
pub use crate::keycodes_h::K_PAD0_START;
pub use crate::keycodes_h::K_PAD0_X;
pub use crate::keycodes_h::K_PAD0_Y;
pub use crate::keycodes_h::K_PAUSE;
pub use crate::keycodes_h::K_PGDN;
pub use crate::keycodes_h::K_PGUP;
pub use crate::keycodes_h::K_POWER;
pub use crate::keycodes_h::K_PRINT;
pub use crate::keycodes_h::K_RIGHTARROW;
pub use crate::keycodes_h::K_SCROLLOCK;
pub use crate::keycodes_h::K_SHIFT;
pub use crate::keycodes_h::K_SPACE;
pub use crate::keycodes_h::K_SUPER;
pub use crate::keycodes_h::K_SYSREQ;
pub use crate::keycodes_h::K_TAB;
pub use crate::keycodes_h::K_UNDO;
pub use crate::keycodes_h::K_UPARROW;
pub use crate::keycodes_h::K_WORLD_0;
pub use crate::keycodes_h::K_WORLD_1;
pub use crate::keycodes_h::K_WORLD_10;
pub use crate::keycodes_h::K_WORLD_11;
pub use crate::keycodes_h::K_WORLD_12;
pub use crate::keycodes_h::K_WORLD_13;
pub use crate::keycodes_h::K_WORLD_14;
pub use crate::keycodes_h::K_WORLD_15;
pub use crate::keycodes_h::K_WORLD_16;
pub use crate::keycodes_h::K_WORLD_17;
pub use crate::keycodes_h::K_WORLD_18;
pub use crate::keycodes_h::K_WORLD_19;
pub use crate::keycodes_h::K_WORLD_2;
pub use crate::keycodes_h::K_WORLD_20;
pub use crate::keycodes_h::K_WORLD_21;
pub use crate::keycodes_h::K_WORLD_22;
pub use crate::keycodes_h::K_WORLD_23;
pub use crate::keycodes_h::K_WORLD_24;
pub use crate::keycodes_h::K_WORLD_25;
pub use crate::keycodes_h::K_WORLD_26;
pub use crate::keycodes_h::K_WORLD_27;
pub use crate::keycodes_h::K_WORLD_28;
pub use crate::keycodes_h::K_WORLD_29;
pub use crate::keycodes_h::K_WORLD_3;
pub use crate::keycodes_h::K_WORLD_30;
pub use crate::keycodes_h::K_WORLD_31;
pub use crate::keycodes_h::K_WORLD_32;
pub use crate::keycodes_h::K_WORLD_33;
pub use crate::keycodes_h::K_WORLD_34;
pub use crate::keycodes_h::K_WORLD_35;
pub use crate::keycodes_h::K_WORLD_36;
pub use crate::keycodes_h::K_WORLD_37;
pub use crate::keycodes_h::K_WORLD_38;
pub use crate::keycodes_h::K_WORLD_39;
pub use crate::keycodes_h::K_WORLD_4;
pub use crate::keycodes_h::K_WORLD_40;
pub use crate::keycodes_h::K_WORLD_41;
pub use crate::keycodes_h::K_WORLD_42;
pub use crate::keycodes_h::K_WORLD_43;
pub use crate::keycodes_h::K_WORLD_44;
pub use crate::keycodes_h::K_WORLD_45;
pub use crate::keycodes_h::K_WORLD_46;
pub use crate::keycodes_h::K_WORLD_47;
pub use crate::keycodes_h::K_WORLD_48;
pub use crate::keycodes_h::K_WORLD_49;
pub use crate::keycodes_h::K_WORLD_5;
pub use crate::keycodes_h::K_WORLD_50;
pub use crate::keycodes_h::K_WORLD_51;
pub use crate::keycodes_h::K_WORLD_52;
pub use crate::keycodes_h::K_WORLD_53;
pub use crate::keycodes_h::K_WORLD_54;
pub use crate::keycodes_h::K_WORLD_55;
pub use crate::keycodes_h::K_WORLD_56;
pub use crate::keycodes_h::K_WORLD_57;
pub use crate::keycodes_h::K_WORLD_58;
pub use crate::keycodes_h::K_WORLD_59;
pub use crate::keycodes_h::K_WORLD_6;
pub use crate::keycodes_h::K_WORLD_60;
pub use crate::keycodes_h::K_WORLD_61;
pub use crate::keycodes_h::K_WORLD_62;
pub use crate::keycodes_h::K_WORLD_63;
pub use crate::keycodes_h::K_WORLD_64;
pub use crate::keycodes_h::K_WORLD_65;
pub use crate::keycodes_h::K_WORLD_66;
pub use crate::keycodes_h::K_WORLD_67;
pub use crate::keycodes_h::K_WORLD_68;
pub use crate::keycodes_h::K_WORLD_69;
pub use crate::keycodes_h::K_WORLD_7;
pub use crate::keycodes_h::K_WORLD_70;
pub use crate::keycodes_h::K_WORLD_71;
pub use crate::keycodes_h::K_WORLD_72;
pub use crate::keycodes_h::K_WORLD_73;
pub use crate::keycodes_h::K_WORLD_74;
pub use crate::keycodes_h::K_WORLD_75;
pub use crate::keycodes_h::K_WORLD_76;
pub use crate::keycodes_h::K_WORLD_77;
pub use crate::keycodes_h::K_WORLD_78;
pub use crate::keycodes_h::K_WORLD_79;
pub use crate::keycodes_h::K_WORLD_8;
pub use crate::keycodes_h::K_WORLD_80;
pub use crate::keycodes_h::K_WORLD_81;
pub use crate::keycodes_h::K_WORLD_82;
pub use crate::keycodes_h::K_WORLD_83;
pub use crate::keycodes_h::K_WORLD_84;
pub use crate::keycodes_h::K_WORLD_85;
pub use crate::keycodes_h::K_WORLD_86;
pub use crate::keycodes_h::K_WORLD_87;
pub use crate::keycodes_h::K_WORLD_88;
pub use crate::keycodes_h::K_WORLD_89;
pub use crate::keycodes_h::K_WORLD_9;
pub use crate::keycodes_h::K_WORLD_90;
pub use crate::keycodes_h::K_WORLD_91;
pub use crate::keycodes_h::K_WORLD_92;
pub use crate::keycodes_h::K_WORLD_93;
pub use crate::keycodes_h::K_WORLD_94;
pub use crate::keycodes_h::K_WORLD_95;
pub use crate::keycodes_h::MAX_KEYS;
pub use crate::src::q3_ui::ui_atoms::uis;
pub use crate::src::q3_ui::ui_atoms::UI_CursorInRect;
pub use crate::src::q3_ui::ui_atoms::UI_DrawProportionalString;
pub use crate::src::q3_ui::ui_atoms::UI_PopMenu;
pub use crate::src::q3_ui::ui_atoms::UI_PushMenu;
pub use crate::src::q3_ui::ui_qmenu::color_orange;
pub use crate::src::q3_ui::ui_qmenu::color_white;
pub use crate::src::q3_ui::ui_qmenu::color_yellow;
pub use crate::src::q3_ui::ui_qmenu::menu_move_sound;
pub use crate::src::q3_ui::ui_qmenu::menu_null_sound;
pub use crate::src::q3_ui::ui_qmenu::Menu_AddItem;
pub use crate::src::q3_ui::ui_qmenu::Menu_DefaultKey;
pub use crate::src::q3_ui::ui_qmenu::Menu_ItemAtCursor;
pub use crate::src::q3_ui::ui_teamorders::stdlib_h::atoi;
pub use crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText;
pub use crate::src::ui::ui_syscalls::trap_GetClientState;
pub use crate::src::ui::ui_syscalls::trap_GetConfigString;
pub use crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip;
use crate::stdlib::memset;
pub use crate::stdlib::strtol;
pub use crate::ui_local_h::_tag_menuframework;
pub use crate::ui_local_h::menubitmap_s;
pub use crate::ui_local_h::menucommon_s;
pub use crate::ui_local_h::menuframework_s;
pub use crate::ui_local_h::menulist_s;
pub use crate::ui_local_h::menutext_s;
pub use crate::ui_local_h::uiStatic_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct teamOrdersMenuInfo_t {
    pub menu: crate::ui_local_h::menuframework_s,
    pub banner: crate::ui_local_h::menutext_s,
    pub frame: crate::ui_local_h::menubitmap_s,
    pub list: crate::ui_local_h::menulist_s,
    pub back: crate::ui_local_h::menubitmap_s,
    pub gametype: i32,
    pub numBots: i32,
    pub selectedBot: i32,
    pub bots: [*mut i8; 9],
    pub botNames: [[i8; 16]; 9],
}

static mut teamOrdersMenuInfo: teamOrdersMenuInfo_t = teamOrdersMenuInfo_t {
    menu: crate::ui_local_h::menuframework_s {
        cursor: 0,
        cursor_prev: 0,
        nitems: 0,
        items: [0 as *mut libc::c_void; 64],
        draw: None,
        key: None,
        wrapAround: crate::src::qcommon::q_shared::qfalse,
        fullscreen: crate::src::qcommon::q_shared::qfalse,
        showlogo: crate::src::qcommon::q_shared::qfalse,
    },
    banner: crate::ui_local_h::menutext_s {
        generic: crate::ui_local_h::menucommon_s {
            type_0: 0,
            name: 0 as *const i8,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *mut crate::ui_local_h::menuframework_s,
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        string: 0 as *mut i8,
        style: 0,
        color: 0 as *mut f32,
    },
    frame: crate::ui_local_h::menubitmap_s {
        generic: crate::ui_local_h::menucommon_s {
            type_0: 0,
            name: 0 as *const i8,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *mut crate::ui_local_h::menuframework_s,
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        focuspic: 0 as *mut i8,
        errorpic: 0 as *mut i8,
        shader: 0,
        focusshader: 0,
        width: 0,
        height: 0,
        focuscolor: 0 as *mut f32,
    },
    list: crate::ui_local_h::menulist_s {
        generic: crate::ui_local_h::menucommon_s {
            type_0: 0,
            name: 0 as *const i8,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *mut crate::ui_local_h::menuframework_s,
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        oldvalue: 0,
        curvalue: 0,
        numitems: 0,
        top: 0,
        itemnames: 0 as *mut *const i8,
        width: 0,
        height: 0,
        columns: 0,
        separation: 0,
    },
    back: crate::ui_local_h::menubitmap_s {
        generic: crate::ui_local_h::menucommon_s {
            type_0: 0,
            name: 0 as *const i8,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *mut crate::ui_local_h::menuframework_s,
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        focuspic: 0 as *mut i8,
        errorpic: 0 as *mut i8,
        shader: 0,
        focusshader: 0,
        width: 0,
        height: 0,
        focuscolor: 0 as *mut f32,
    },
    gametype: 0,
    numBots: 0,
    selectedBot: 0,
    bots: [0 as *mut i8; 9],
    botNames: [[0; 16]; 9],
};

static mut ctfOrders: [*const i8; 8] = [
    b"I Am the Leader\x00" as *const u8 as *const i8,
    b"Defend the Base\x00" as *const u8 as *const i8,
    b"Follow Me\x00" as *const u8 as *const i8,
    b"Get Enemy Flag\x00" as *const u8 as *const i8,
    b"Camp Here\x00" as *const u8 as *const i8,
    b"Report\x00" as *const u8 as *const i8,
    b"I Relinquish Command\x00" as *const u8 as *const i8,
    0 as *const i8,
];

static mut ctfMessages: [*const i8; 8] = [
    b"i am the leader\x00" as *const u8 as *const i8,
    b"%s defend the base\x00" as *const u8 as *const i8,
    b"%s follow me\x00" as *const u8 as *const i8,
    b"%s get enemy flag\x00" as *const u8 as *const i8,
    b"%s camp here\x00" as *const u8 as *const i8,
    b"%s report\x00" as *const u8 as *const i8,
    b"i stop being the leader\x00" as *const u8 as *const i8,
    0 as *const i8,
];

static mut teamOrders: [*const i8; 7] = [
    b"I Am the Leader\x00" as *const u8 as *const i8,
    b"Follow Me\x00" as *const u8 as *const i8,
    b"Roam\x00" as *const u8 as *const i8,
    b"Camp Here\x00" as *const u8 as *const i8,
    b"Report\x00" as *const u8 as *const i8,
    b"I Relinquish Command\x00" as *const u8 as *const i8,
    0 as *const i8,
];

static mut teamMessages: [*const i8; 7] = [
    b"i am the leader\x00" as *const u8 as *const i8,
    b"%s follow me\x00" as *const u8 as *const i8,
    b"%s roam\x00" as *const u8 as *const i8,
    b"%s camp here\x00" as *const u8 as *const i8,
    b"%s report\x00" as *const u8 as *const i8,
    b"i stop being the leader\x00" as *const u8 as *const i8,
    0 as *const i8,
];
/*
===============
UI_TeamOrdersMenu_BackEvent
===============
*/

unsafe extern "C" fn UI_TeamOrdersMenu_BackEvent(mut ptr: *mut libc::c_void, mut event: i32) {
    if event != 3 {
        return;
    }
    crate::src::q3_ui::ui_atoms::UI_PopMenu();
}
/*
===============
UI_TeamOrdersMenu_SetList
===============
*/

unsafe extern "C" fn UI_TeamOrdersMenu_SetList(mut id: i32) {
    match id {
        11 => {
            teamOrdersMenuInfo.list.generic.id = id;
            teamOrdersMenuInfo.list.numitems = 7;
            teamOrdersMenuInfo.list.itemnames = ctfOrders.as_mut_ptr()
        }
        12 => {
            teamOrdersMenuInfo.list.generic.id = id;
            teamOrdersMenuInfo.list.numitems = 6;
            teamOrdersMenuInfo.list.itemnames = teamOrders.as_mut_ptr()
        }
        10 | _ => {
            teamOrdersMenuInfo.list.generic.id = id;
            teamOrdersMenuInfo.list.numitems = teamOrdersMenuInfo.numBots;
            teamOrdersMenuInfo.list.itemnames =
                teamOrdersMenuInfo.bots.as_mut_ptr() as *mut *const i8
        }
    }
    teamOrdersMenuInfo.list.generic.bottom =
        teamOrdersMenuInfo.list.generic.top + teamOrdersMenuInfo.list.numitems * 27;
}
/*
=================
UI_TeamOrdersMenu_Key
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_TeamOrdersMenu_Key(
    mut key: i32,
) -> crate::src::qcommon::q_shared::sfxHandle_t {
    let mut l: *mut crate::ui_local_h::menulist_s = 0 as *mut crate::ui_local_h::menulist_s;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut index: i32 = 0;
    l = crate::src::q3_ui::ui_qmenu::Menu_ItemAtCursor(&mut teamOrdersMenuInfo.menu)
        as *mut crate::ui_local_h::menulist_s;
    if l != &mut teamOrdersMenuInfo.list as *mut crate::ui_local_h::menulist_s {
        return crate::src::q3_ui::ui_qmenu::Menu_DefaultKey(&mut teamOrdersMenuInfo.menu, key);
    }
    match key {
        178 => {
            x = (*l).generic.left;
            y = (*l).generic.top;
            if crate::src::q3_ui::ui_atoms::UI_CursorInRect(
                x,
                y,
                (*l).generic.right - x,
                (*l).generic.bottom - y,
            ) as u64
                != 0
            {
                index = (crate::src::q3_ui::ui_atoms::uis.cursory - y) / 27;
                (*l).oldvalue = (*l).curvalue;
                (*l).curvalue = index;
                if (*l).generic.callback.is_some() {
                    (*l).generic.callback.expect("non-null function pointer")(
                        l as *mut libc::c_void,
                        3,
                    );
                    return crate::src::q3_ui::ui_qmenu::menu_move_sound;
                }
            }
            return crate::src::q3_ui::ui_qmenu::menu_null_sound;
        }
        161 | 132 => {
            (*l).oldvalue = (*l).curvalue;
            if (*l).curvalue == 0 {
                (*l).curvalue = (*l).numitems - 1
            } else {
                (*l).curvalue -= 1
            }
            return crate::src::q3_ui::ui_qmenu::menu_move_sound;
        }
        167 | 133 => {
            (*l).oldvalue = (*l).curvalue;
            if (*l).curvalue == (*l).numitems - 1 {
                (*l).curvalue = 0
            } else {
                (*l).curvalue += 1
            }
            return crate::src::q3_ui::ui_qmenu::menu_move_sound;
        }
        _ => {}
    }
    return crate::src::q3_ui::ui_qmenu::Menu_DefaultKey(&mut teamOrdersMenuInfo.menu, key);
}
/*
=================
UI_TeamOrdersMenu_ListDraw
=================
*/

unsafe extern "C" fn UI_TeamOrdersMenu_ListDraw(mut self_0: *mut libc::c_void) {
    let mut l: *mut crate::ui_local_h::menulist_s = 0 as *mut crate::ui_local_h::menulist_s; //l->generic.x;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut i: i32 = 0;
    let mut color: *mut f32 = 0 as *mut f32;
    let mut hasfocus: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    let mut style: i32 = 0;
    l = self_0 as *mut crate::ui_local_h::menulist_s;
    hasfocus = ((*(*l).generic.parent).cursor == (*l).generic.menuPosition)
        as crate::src::qcommon::q_shared::qboolean;
    x = 320;
    y = (*l).generic.y;
    i = 0;
    while i < (*l).numitems {
        style = 0 | 0x10 | 0x1;
        if i == (*l).curvalue {
            color = crate::src::q3_ui::ui_qmenu::color_yellow.as_mut_ptr();
            if hasfocus as u64 != 0 {
                style |= 0x4000
            }
        } else {
            color = crate::src::q3_ui::ui_qmenu::color_orange.as_mut_ptr()
        }
        crate::src::q3_ui::ui_atoms::UI_DrawProportionalString(
            x,
            y,
            *(*l).itemnames.offset(i as isize),
            style,
            color,
        );
        y += 27;
        i += 1
    }
}
/*
===============
UI_TeamOrdersMenu_ListEvent
===============
*/

unsafe extern "C" fn UI_TeamOrdersMenu_ListEvent(mut ptr: *mut libc::c_void, mut event: i32) {
    let mut id: i32 = 0;
    let mut selection: i32 = 0;
    let mut message: [i8; 256] = [0; 256];
    if event != 3 {
        return;
    }
    id = (*(ptr as *mut crate::ui_local_h::menulist_s)).generic.id;
    selection = (*(ptr as *mut crate::ui_local_h::menulist_s)).curvalue;
    if id == 10 {
        teamOrdersMenuInfo.selectedBot = selection;
        if teamOrdersMenuInfo.gametype == crate::bg_public_h::GT_CTF as i32 {
            UI_TeamOrdersMenu_SetList(11i32);
        } else {
            UI_TeamOrdersMenu_SetList(12i32);
        }
        return;
    }
    if id == 11 {
        crate::src::qcommon::q_shared::Com_sprintf(
            message.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 256]>() as i32,
            ctfMessages[selection as usize],
            teamOrdersMenuInfo.botNames[teamOrdersMenuInfo.selectedBot as usize].as_mut_ptr(),
        );
    } else {
        crate::src::qcommon::q_shared::Com_sprintf(
            message.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 256]>() as i32,
            teamMessages[selection as usize],
            teamOrdersMenuInfo.botNames[teamOrdersMenuInfo.selectedBot as usize].as_mut_ptr(),
        );
    }
    crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText(
        crate::src::qcommon::q_shared::EXEC_APPEND as i32,
        crate::src::qcommon::q_shared::va(
            b"say_team \"%s\"\n\x00" as *const u8 as *mut i8,
            message.as_mut_ptr(),
        ),
    );
    crate::src::q3_ui::ui_atoms::UI_PopMenu();
}
/*
===============
UI_TeamOrdersMenu_BuildBotList
===============
*/

unsafe extern "C" fn UI_TeamOrdersMenu_BuildBotList() {
    let mut cs: crate::ui_public_h::uiClientState_t = crate::ui_public_h::uiClientState_t {
        connState: crate::src::qcommon::q_shared::CA_UNINITIALIZED,
        connectPacketCount: 0,
        clientNum: 0,
        servername: [0; 1024],
        updateInfoString: [0; 1024],
        messageString: [0; 1024],
    };
    let mut numPlayers: i32 = 0;
    let mut isBot: i32 = 0;
    let mut n: i32 = 0;
    let mut playerTeam: i8 = '3' as i8;
    let mut botTeam: i8 = 0;
    let mut info: [i8; 1024] = [0; 1024];
    n = 0;
    while n < 9 {
        teamOrdersMenuInfo.bots[n as usize] = teamOrdersMenuInfo.botNames[n as usize].as_mut_ptr();
        n += 1
    }
    crate::src::ui::ui_syscalls::trap_GetClientState(&mut cs);
    crate::src::qcommon::q_shared::Q_strncpyz(
        teamOrdersMenuInfo.botNames[0].as_mut_ptr(),
        b"Everyone\x00" as *const u8 as *const i8,
        16,
    );
    teamOrdersMenuInfo.numBots = 1;
    crate::src::ui::ui_syscalls::trap_GetConfigString(
        0,
        info.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 1024]>() as i32,
    );
    numPlayers = atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
        info.as_mut_ptr(),
        b"sv_maxclients\x00" as *const u8 as *const i8,
    ));
    teamOrdersMenuInfo.gametype = atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
        info.as_mut_ptr(),
        b"g_gametype\x00" as *const u8 as *const i8,
    ));
    n = 0;
    while n < numPlayers && teamOrdersMenuInfo.numBots < 9 {
        crate::src::ui::ui_syscalls::trap_GetConfigString(
            32 + 256 + 256 + n,
            info.as_mut_ptr(),
            1024,
        );
        if n == cs.clientNum {
            playerTeam = *crate::src::qcommon::q_shared::Info_ValueForKey(
                info.as_mut_ptr(),
                b"t\x00" as *const u8 as *const i8,
            )
        } else {
            isBot = atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
                info.as_mut_ptr(),
                b"skill\x00" as *const u8 as *const i8,
            ));
            if !(isBot == 0) {
                botTeam = *crate::src::qcommon::q_shared::Info_ValueForKey(
                    info.as_mut_ptr(),
                    b"t\x00" as *const u8 as *const i8,
                );
                if !(botTeam as i32 != playerTeam as i32) {
                    crate::src::qcommon::q_shared::Q_strncpyz(
                        teamOrdersMenuInfo.botNames[teamOrdersMenuInfo.numBots as usize]
                            .as_mut_ptr(),
                        crate::src::qcommon::q_shared::Info_ValueForKey(
                            info.as_mut_ptr(),
                            b"n\x00" as *const u8 as *const i8,
                        ),
                        16,
                    );
                    crate::src::qcommon::q_shared::Q_CleanStr(
                        teamOrdersMenuInfo.botNames[teamOrdersMenuInfo.numBots as usize]
                            .as_mut_ptr(),
                    );
                    teamOrdersMenuInfo.numBots += 1
                }
            }
        }
        n += 1
    }
}
/*
===============
UI_TeamOrdersMenu_Init
===============
*/

unsafe extern "C" fn UI_TeamOrdersMenu_Init() {
    UI_TeamOrdersMenu_Cache();
    crate::stdlib::memset(
        &mut teamOrdersMenuInfo as *mut teamOrdersMenuInfo_t as *mut libc::c_void,
        0,
        ::std::mem::size_of::<teamOrdersMenuInfo_t>(),
    );
    teamOrdersMenuInfo.menu.fullscreen = crate::src::qcommon::q_shared::qfalse;
    teamOrdersMenuInfo.menu.key = Some(
        UI_TeamOrdersMenu_Key
            as unsafe extern "C" fn(_: i32) -> crate::src::qcommon::q_shared::sfxHandle_t,
    );
    UI_TeamOrdersMenu_BuildBotList();
    teamOrdersMenuInfo.banner.generic.type_0 = 10;
    teamOrdersMenuInfo.banner.generic.x = 320;
    teamOrdersMenuInfo.banner.generic.y = 16;
    teamOrdersMenuInfo.banner.string = b"TEAM ORDERS\x00" as *const u8 as *mut i8;
    teamOrdersMenuInfo.banner.color = crate::src::q3_ui::ui_qmenu::color_white.as_mut_ptr();
    teamOrdersMenuInfo.banner.style = 0x1;
    teamOrdersMenuInfo.frame.generic.type_0 = 6;
    teamOrdersMenuInfo.frame.generic.flags = 0x4000;
    teamOrdersMenuInfo.frame.generic.name = b"menu/art/addbotframe\x00" as *const u8 as *const i8;
    teamOrdersMenuInfo.frame.generic.x = 320 - 233;
    teamOrdersMenuInfo.frame.generic.y = 240 - 166;
    teamOrdersMenuInfo.frame.width = 466;
    teamOrdersMenuInfo.frame.height = 332;
    teamOrdersMenuInfo.list.generic.type_0 = 8;
    teamOrdersMenuInfo.list.generic.flags = 0x100;
    teamOrdersMenuInfo.list.generic.ownerdraw =
        Some(UI_TeamOrdersMenu_ListDraw as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    teamOrdersMenuInfo.list.generic.callback = Some(
        UI_TeamOrdersMenu_ListEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> (),
    );
    teamOrdersMenuInfo.list.generic.x = 320 - 64;
    teamOrdersMenuInfo.list.generic.y = 120;
    teamOrdersMenuInfo.back.generic.type_0 = 6;
    teamOrdersMenuInfo.back.generic.name = b"menu/art/back_0\x00" as *const u8 as *const i8;
    teamOrdersMenuInfo.back.generic.flags = 0x4 | 0x100;
    teamOrdersMenuInfo.back.generic.callback = Some(
        UI_TeamOrdersMenu_BackEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> (),
    );
    teamOrdersMenuInfo.back.generic.x = 0;
    teamOrdersMenuInfo.back.generic.y = 480 - 64;
    teamOrdersMenuInfo.back.width = 128;
    teamOrdersMenuInfo.back.height = 64;
    teamOrdersMenuInfo.back.focuspic = b"menu/art/back_1\x00" as *const u8 as *mut i8;
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut teamOrdersMenuInfo.menu,
        &mut teamOrdersMenuInfo.banner as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut teamOrdersMenuInfo.menu,
        &mut teamOrdersMenuInfo.frame as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut teamOrdersMenuInfo.menu,
        &mut teamOrdersMenuInfo.list as *mut crate::ui_local_h::menulist_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut teamOrdersMenuInfo.menu,
        &mut teamOrdersMenuInfo.back as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    teamOrdersMenuInfo.list.generic.left = 220;
    teamOrdersMenuInfo.list.generic.top = teamOrdersMenuInfo.list.generic.y;
    teamOrdersMenuInfo.list.generic.right = 420;
    UI_TeamOrdersMenu_SetList(10);
}
/*
=================
UI_TeamOrdersMenu_Cache
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_TeamOrdersMenu_Cache() {
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/addbotframe\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/back_0\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/back_1\x00" as *const u8 as *const i8,
    );
}
/*
===============
UI_TeamOrdersMenu
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_TeamOrdersMenu() {
    UI_TeamOrdersMenu_Init();
    crate::src::q3_ui::ui_atoms::UI_PushMenu(&mut teamOrdersMenuInfo.menu);
}
/*
===========================================================================
Copyright (C) 1999-2005 Id Software, Inc.

This file is part of Quake III Arena source code.

Quake III Arena source code is free software; you can redistribute it
and/or modify it under the terms of the GNU General Public License as
published by the Free Software Foundation; either version 2 of the License,
or (at your option) any later version.

Quake III Arena source code is distributed in the hope that it will be
useful, but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with Quake III Arena source code; if not, write to the Free Software
Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
===========================================================================
*/
//
//NOTE: include the ui_public.h from the new UI
//redefine to old API version
//
// ui_qmenu.c
//
// edit field is only numbers
// steady focus
// pulse if focus
// only mouse input allowed
// skips drawing
// grays and disables
// disables any input
// skip default initialization
// edit field is all lower case
// edit field is all upper case
// callback notifications
//
// ui_mfield.c
//
//
// ui_menu.c
//
//
// ui_credits.c
//
//
// ui_ingame.c
//
//
// ui_confirm.c
//
//
// ui_setup.c
//
//
// ui_team.c
//
//
// ui_connect.c
//
//
// ui_controls2.c
//
//
// ui_demo2.c
//
//
// ui_cinematics.c
//
//
// ui_mods.c
//
//
// ui_cdkey.c
//
//
// ui_playermodel.c
//
//
// ui_playersettings.c
//
//
// ui_preferences.c
//
//
// ui_specifyleague.c
//
//
// ui_specifyserver.c
//
//
// ui_servers2.c
//
//
// ui_startserver.c
//
//
// ui_serverinfo.c
//
//
// ui_video.c
//
//
// ui_players.c
//
//FIXME ripped from cg_local.h
// time when ->oldFrame was exactly on
// time when ->frame will be exactly on
// may include ANIM_TOGGLEBIT
// time when the first frame of the animation will be exact
// model info
// true if legs yaw is always the same as torso yaw
// true if torso never changes yaw
// currently in use drawing parms
// animation vars
//
// ui_atoms.c
//
//
// ui_spLevel.c
//
//
// ui_spArena.c
//
//
// ui_spPostgame.c
//
//
// ui_spSkill.c
//
//
// ui_syscalls.c
//
// don't use EXEC_NOW!
// fsOrigin_t
//
// ui_addbots.c
//
//
// ui_removebots.c
//
//
// ui_teamorders.c
//
/*
===============
UI_TeamOrdersMenu_f
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_TeamOrdersMenu_f() {
    let mut cs: crate::ui_public_h::uiClientState_t = crate::ui_public_h::uiClientState_t {
        connState: crate::src::qcommon::q_shared::CA_UNINITIALIZED,
        connectPacketCount: 0,
        clientNum: 0,
        servername: [0; 1024],
        updateInfoString: [0; 1024],
        messageString: [0; 1024],
    };
    let mut info: [i8; 1024] = [0; 1024];
    let mut team: i32 = 0;
    // make sure it's a team game
    crate::src::ui::ui_syscalls::trap_GetConfigString(
        0,
        info.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 1024]>() as i32,
    );
    teamOrdersMenuInfo.gametype = atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
        info.as_mut_ptr(),
        b"g_gametype\x00" as *const u8 as *const i8,
    ));
    if teamOrdersMenuInfo.gametype < crate::bg_public_h::GT_TEAM as i32 {
        return;
    }
    // not available to spectators
    crate::src::ui::ui_syscalls::trap_GetClientState(&mut cs);
    crate::src::ui::ui_syscalls::trap_GetConfigString(
        32 + 256 + 256 + cs.clientNum,
        info.as_mut_ptr(),
        1024,
    );
    team = atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
        info.as_mut_ptr(),
        b"t\x00" as *const u8 as *const i8,
    ));
    if team == crate::bg_public_h::TEAM_SPECTATOR as i32 {
        return;
    }
    UI_TeamOrdersMenu();
}
