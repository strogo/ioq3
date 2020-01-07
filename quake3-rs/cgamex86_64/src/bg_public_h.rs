pub type weapon_t = u32;
pub type holdable_t = u32;
pub const WP_NUM_WEAPONS: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 11;
pub const WP_GRAPPLING_HOOK: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 10;
pub const WP_BFG: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 9;
pub const WP_PLASMAGUN: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 8;
pub const WP_RAILGUN: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 7;
pub const WP_LIGHTNING: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 6;
pub const WP_ROCKET_LAUNCHER: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 5;
pub const WP_GRENADE_LAUNCHER: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 4;
pub const WP_SHOTGUN: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 3;
pub const WP_MACHINEGUN: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 2;
pub const WP_GAUNTLET: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 1;
pub const WP_NONE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 0;
pub const ET_EVENTS: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 13;
// any of the EV_* events can be added freestanding

// by setting eType to ET_EVENTS + eventNum

// this avoids having to set eFlags and eventNum

// grapple hooked on wall
pub const ET_TEAM: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 12;
pub const ET_GRAPPLE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 11;
pub const ET_INVISIBLE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 10;
pub const ET_TELEPORT_TRIGGER: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 9;
pub const ET_PUSH_TRIGGER: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 8;
pub const ET_SPEAKER: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 7;
pub const ET_PORTAL: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 6;
pub const ET_BEAM: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 5;
pub const ET_MOVER: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 4;
pub const ET_MISSILE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 3;
pub const ET_ITEM: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 2;
pub const ET_PLAYER: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 1;
pub const ET_GENERAL: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 0;
pub const HI_NUM_HOLDABLE: crate::bg_public_h::holdable_t = 6;
pub const HI_INVULNERABILITY: crate::bg_public_h::holdable_t = 5;
pub const HI_PORTAL: crate::bg_public_h::holdable_t = 4;
pub const HI_KAMIKAZE: crate::bg_public_h::holdable_t = 3;
pub const HI_MEDKIT: crate::bg_public_h::holdable_t = 2;
pub const HI_TELEPORTER: crate::bg_public_h::holdable_t = 1;
pub const HI_NONE: crate::bg_public_h::holdable_t = 0;
pub const EV_TAUNT_PATROL: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 82;
pub const EV_TAUNT_GUARDBASE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 81;
pub const EV_TAUNT_GETFLAG: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 80;
pub const EV_TAUNT_FOLLOWME: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 79;
pub const EV_TAUNT_NO: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 78;
pub const EV_TAUNT_YES: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 77;
pub const EV_TAUNT: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 76;
pub const EV_STOPLOOPINGSOUND: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 75;
pub const EV_DEBUG_LINE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 74;
pub const EV_LIGHTNINGBOLT: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 73;
pub const EV_JUICED: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 72;
pub const EV_INVUL_IMPACT: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 71;
pub const EV_OBELISKPAIN: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 70;
pub const EV_OBELISKEXPLODE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 69;
pub const EV_KAMIKAZE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 68;
pub const EV_PROXIMITY_MINE_TRIGGER: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 67;
pub const EV_PROXIMITY_MINE_STICK: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 66;
pub const EV_SCOREPLUM: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 65;
pub const EV_GIB_PLAYER: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 64;
pub const EV_POWERUP_REGEN: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 63;
pub const EV_POWERUP_BATTLESUIT: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 62;
pub const EV_POWERUP_QUAD: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 61;
pub const EV_OBITUARY: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 60;
pub const EV_DEATH3: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 59;
pub const EV_DEATH2: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 58;
pub const EV_DEATH1: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 57;
pub const EV_PAIN: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 56;
pub const EV_BULLET: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 55;
pub const EV_SHOTGUN: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 54;
pub const EV_RAILTRAIL: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 53;
pub const EV_MISSILE_MISS_METAL: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 52;
pub const EV_MISSILE_MISS: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 51;
pub const EV_MISSILE_HIT: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 50;
pub const EV_BULLET_HIT_WALL: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 49;
pub const EV_BULLET_HIT_FLESH: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 48;
pub const EV_GLOBAL_TEAM_SOUND: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 47;
pub const EV_GLOBAL_SOUND: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 46;
pub const EV_GENERAL_SOUND: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 45;
pub const EV_GRENADE_BOUNCE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 44;
pub const EV_PLAYER_TELEPORT_OUT: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 43;
pub const EV_PLAYER_TELEPORT_IN: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 42;
pub const EV_ITEM_POP: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 41;
pub const EV_ITEM_RESPAWN: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 40;
pub const EV_USE_ITEM15: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 39;
pub const EV_USE_ITEM14: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 38;
pub const EV_USE_ITEM13: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 37;
pub const EV_USE_ITEM12: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 36;
pub const EV_USE_ITEM11: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 35;
pub const EV_USE_ITEM10: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 34;
pub const EV_USE_ITEM9: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 33;
pub const EV_USE_ITEM8: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 32;
pub const EV_USE_ITEM7: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 31;
pub const EV_USE_ITEM6: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 30;
pub const EV_USE_ITEM5: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 29;
pub const EV_USE_ITEM4: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 28;
pub const EV_USE_ITEM3: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 27;
pub const EV_USE_ITEM2: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 26;
pub const EV_USE_ITEM1: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 25;
pub const EV_USE_ITEM0: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 24;
pub const EV_FIRE_WEAPON: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 23;
pub const EV_CHANGE_WEAPON: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 22;
pub const EV_NOAMMO: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 21;
pub const EV_GLOBAL_ITEM_PICKUP: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 20;
pub const EV_ITEM_PICKUP: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 19;
pub const EV_WATER_CLEAR: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 18;
pub const EV_WATER_UNDER: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 17;
pub const EV_WATER_LEAVE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 16;
pub const EV_WATER_TOUCH: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 15;
pub const EV_JUMP: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 14;
pub const EV_JUMP_PAD: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 13;
pub const EV_FALL_FAR: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 12;
pub const EV_FALL_MEDIUM: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 11;
pub const EV_FALL_SHORT: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 10;
pub const EV_STEP_16: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 9;
pub const EV_STEP_12: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 8;
pub const EV_STEP_8: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 7;
pub const EV_STEP_4: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 6;
pub const EV_SWIM: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 5;
pub const EV_FOOTWADE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 4;
pub const EV_FOOTSPLASH: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 3;
pub const EV_FOOTSTEP_METAL: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 2;
pub const EV_FOOTSTEP: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 1;
pub const EV_NONE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 0;
pub const GTS_KAMIKAZE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 13;
pub const GTS_TEAMS_ARE_TIED: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 12;
pub const GTS_BLUETEAM_TOOK_LEAD: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 11;
pub const GTS_REDTEAM_TOOK_LEAD: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 10;
pub const GTS_BLUETEAM_SCORED: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 9;
pub const GTS_REDTEAM_SCORED: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 8;
pub const GTS_BLUEOBELISK_ATTACKED: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 7;
pub const GTS_REDOBELISK_ATTACKED: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 6;
pub const GTS_BLUE_TAKEN: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 5;
pub const GTS_RED_TAKEN: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 4;
pub const GTS_BLUE_RETURN: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 3;
pub const GTS_RED_RETURN: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 2;
pub const GTS_BLUE_CAPTURE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 1;
pub const GTS_RED_CAPTURE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 0;
pub const MAX_TOTALANIMATIONS: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 37;
pub const FLAG_STAND2RUN: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 36;
pub const FLAG_STAND: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 35;
pub const FLAG_RUN: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 34;
pub const LEGS_BACKWALK: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 33;
pub const LEGS_BACKCR: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 32;
pub const MAX_ANIMATIONS: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 31;
pub const TORSO_NEGATIVE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 30;
pub const TORSO_AFFIRMATIVE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 29;
pub const TORSO_FOLLOWME: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 28;
pub const TORSO_PATROL: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 27;
pub const TORSO_GUARDBASE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 26;
pub const TORSO_GETFLAG: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 25;
pub const LEGS_TURN: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 24;
pub const LEGS_IDLECR: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 23;
pub const LEGS_IDLE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 22;
pub const LEGS_LANDB: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 21;
pub const LEGS_JUMPB: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 20;
pub const LEGS_LAND: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 19;
pub const LEGS_JUMP: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 18;
pub const LEGS_SWIM: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 17;
pub const LEGS_BACK: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 16;
pub const LEGS_RUN: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 15;
pub const LEGS_WALK: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 14;
pub const LEGS_WALKCR: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 13;
pub const TORSO_STAND2: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 12;
pub const TORSO_STAND: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 11;
pub const TORSO_RAISE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 10;
pub const TORSO_DROP: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 9;
pub const TORSO_ATTACK2: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 8;
pub const TORSO_ATTACK: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 7;
pub const TORSO_GESTURE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 6;
pub const BOTH_DEAD3: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 5;
pub const BOTH_DEATH3: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 4;
pub const BOTH_DEAD2: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 3;
pub const BOTH_DEATH2: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 2;
pub const BOTH_DEAD1: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 1;
pub const BOTH_DEATH1: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 0;
pub const MOD_GRAPPLE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 23;
pub const MOD_TRIGGER_HURT: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 22;
pub const MOD_TARGET_LASER: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 21;
pub const MOD_SUICIDE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 20;
pub const MOD_FALLING: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 19;
pub const MOD_TELEFRAG: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 18;
pub const MOD_CRUSH: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 17;
pub const MOD_LAVA: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 16;
pub const MOD_SLIME: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 15;
pub const MOD_WATER: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 14;
pub const MOD_BFG_SPLASH: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 13;
pub const MOD_BFG: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 12;
pub const MOD_LIGHTNING: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 11;
pub const MOD_RAILGUN: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 10;
pub const MOD_PLASMA_SPLASH: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 9;
pub const MOD_PLASMA: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 8;
pub const MOD_ROCKET_SPLASH: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 7;
pub const MOD_ROCKET: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 6;
pub const MOD_GRENADE_SPLASH: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 5;
pub const MOD_GRENADE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 4;
pub const MOD_MACHINEGUN: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 3;
pub const MOD_GAUNTLET: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 2;
pub const MOD_SHOTGUN: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 1;
pub const MOD_UNKNOWN: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct animation_s {
    pub firstFrame: i32,
    pub numFrames: i32,
    pub loopFrames: i32,
    pub frameLerp: i32,
    pub initialLerp: i32,
    pub reversed: i32,
    pub flipflop: i32,
}
pub type animation_t = crate::bg_public_h::animation_s;
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

// bg_public.h -- definitions shared by both the server game and client game modules

// because games can change separately from the main system version, we need a

// second version that must match between game and cgame

// item sizes are needed for client side pickup detection

// for the CS_SCORES[12] when only one player is present

// 30 seconds before vote times out

//

// config strings are a general means of communicating variable length strings

// from the server to all connected clients.

//

// CS_SERVERINFO and CS_SYSTEMINFO are defined in q_shared.h

// from the map worldspawn's message field

// g_motd string for server message of the day

// server time when the match will be restarted

// so the timer only shows the current level

// when 1, fraglimit/timelimit has been hit and intermission will start in a second or two

// string indicating flag status in CTF

// string of 0's and 1's that tell which items are present
pub type gametype_t = u32;
pub type gender_t = u32;
// entityState_t->eFlags

// don't draw a foe marker over players with EF_DEAD

// toggled every time the origin abruptly changes

// draw an excellent sprite

// for missiles

// for missiles

// draw a gauntlet sprite

// may have an event, but no model (unspawned items)

// for lightning gun

// will push otherwise

// draw the capture sprite

// draw a talk balloon

// draw a connection trouble sprite

// already cast a vote

// draw an impressive sprite

// draw a defend sprite

// draw a assist sprite

// denied

// already cast a team vote

// NOTE: may not have more than 16
pub type powerup_t = u32;
// true if animation should flipflop back to base

// flip the togglebit every time an animation

// changes so a restart of the same anim can be detected
pub type team_t = u32;
//---------------------------------------------------------

// gitem_t->type
pub type itemType_t = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gitem_s {
    pub classname: *mut i8,
    pub pickup_sound: *mut i8,
    pub world_model: [*mut i8; 4],
    pub icon: *mut i8,
    pub pickup_name: *mut i8,
    pub quantity: i32,
    pub giType: crate::bg_public_h::itemType_t,
    pub giTag: i32,
    pub precaches: *mut i8,
    pub sounds: *mut i8,
}
pub type gitem_t = crate::bg_public_h::gitem_s;
pub const GT_MAX_GAME_TYPE: crate::bg_public_h::gametype_t = 8;
pub const GT_HARVESTER: crate::bg_public_h::gametype_t = 7;
pub const GT_OBELISK: crate::bg_public_h::gametype_t = 6;
// capture the flag
pub const GT_1FCTF: crate::bg_public_h::gametype_t = 5;
// team deathmatch
pub const GT_CTF: crate::bg_public_h::gametype_t = 4;
// single player ffa

//-- team games go after this --
pub const GT_TEAM: crate::bg_public_h::gametype_t = 3;
// one on one tournament
pub const GT_SINGLE_PLAYER: crate::bg_public_h::gametype_t = 2;
// free for all
pub const GT_TOURNAMENT: crate::bg_public_h::gametype_t = 1;
pub const GT_FFA: crate::bg_public_h::gametype_t = 0;
pub const GENDER_NEUTER: crate::bg_public_h::gender_t = 2;
pub const GENDER_FEMALE: crate::bg_public_h::gender_t = 1;
pub const GENDER_MALE: crate::bg_public_h::gender_t = 0;
// no movement or status bar

// no movement or status bar
pub const PM_SPINTERMISSION: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 6;
// stuck in place with no control
pub const PM_INTERMISSION: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 5;
// no acceleration or turning, but free falling
pub const PM_FREEZE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 4;
// still run into walls
pub const PM_DEAD: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 3;
// noclip movement
pub const PM_SPECTATOR: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 2;
// can accelerate and turn
pub const PM_NOCLIP: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 1;
pub const PM_NORMAL: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 0;
pub const WEAPON_FIRING: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 3;
pub const WEAPON_DROPPING: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 2;
pub const WEAPON_RAISING: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 1;
pub const WEAPON_READY: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 0;
// health / armor limit, changeable by handicap

// bit mask of clients wishing to exit the intermission (FIXME: configstring?)
pub const STAT_MAX_HEALTH: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 6;
// look this direction when dead (FIXME: get rid of?)
pub const STAT_CLIENTS_READY: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 5;
pub const STAT_DEAD_YAW: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 4;
// 16 bit fields
pub const STAT_ARMOR: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 3;
pub const STAT_WEAPONS: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 2;
pub const STAT_HOLDABLE_ITEM: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 1;
pub const STAT_HEALTH: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 0;
// captures

// kills with the guantlet
pub const PERS_CAPTURES: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 14;
// assist awards
pub const PERS_GAUNTLET_FRAG_COUNT: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 13;
// defend awards
pub const PERS_ASSIST_COUNT: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 12;
// two successive kills in a short amount of time
pub const PERS_DEFEND_COUNT: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 11;
// two railgun hits in a row
pub const PERS_EXCELLENT_COUNT: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 10;
// count of the number of times you died

// player awards tracking
pub const PERS_IMPRESSIVE_COUNT: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 9;
// health/armor of last person we attacked
pub const PERS_KILLED: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 8;
// clientnum of last damage inflicter
pub const PERS_ATTACKEE_ARMOR: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 7;
// 16 bits that can be flipped for events
pub const PERS_ATTACKER: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 6;
// incremented every respawn
pub const PERS_PLAYEREVENTS: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 5;
// player team
pub const PERS_SPAWN_COUNT: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 4;
// player rank or team rank
pub const PERS_TEAM: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 3;
// total points damage inflicted so damage beeps can sound on change
pub const PERS_RANK: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 2;
// !!! MUST NOT CHANGE, SERVER AND GAME BOTH REFERENCE !!!
pub const PERS_HITS: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 1;
pub const PERS_SCORE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 0;
pub const PW_NUM_POWERUPS: crate::bg_public_h::powerup_t = 15;
pub const PW_INVULNERABILITY: crate::bg_public_h::powerup_t = 14;
pub const PW_AMMOREGEN: crate::bg_public_h::powerup_t = 13;
pub const PW_DOUBLER: crate::bg_public_h::powerup_t = 12;
pub const PW_GUARD: crate::bg_public_h::powerup_t = 11;
pub const PW_SCOUT: crate::bg_public_h::powerup_t = 10;
pub const PW_NEUTRALFLAG: crate::bg_public_h::powerup_t = 9;
pub const PW_BLUEFLAG: crate::bg_public_h::powerup_t = 8;
pub const PW_REDFLAG: crate::bg_public_h::powerup_t = 7;
pub const PW_FLIGHT: crate::bg_public_h::powerup_t = 6;
pub const PW_REGEN: crate::bg_public_h::powerup_t = 5;
pub const PW_INVIS: crate::bg_public_h::powerup_t = 4;
pub const PW_HASTE: crate::bg_public_h::powerup_t = 3;
pub const PW_BATTLESUIT: crate::bg_public_h::powerup_t = 2;
pub const PW_QUAD: crate::bg_public_h::powerup_t = 1;
pub const PW_NONE: crate::bg_public_h::powerup_t = 0;
pub const TEAM_NUM_TEAMS: crate::bg_public_h::team_t = 4;
pub const TEAM_SPECTATOR: crate::bg_public_h::team_t = 3;
pub const TEAM_BLUE: crate::bg_public_h::team_t = 2;
pub const TEAM_RED: crate::bg_public_h::team_t = 1;
pub const TEAM_FREE: crate::bg_public_h::team_t = 0;
pub const IT_TEAM: crate::bg_public_h::itemType_t = 8;
// single use, holdable item

// EFX: rotate + bob
pub const IT_PERSISTANT_POWERUP: crate::bg_public_h::itemType_t = 7;
// instant on, timer based

// EFX: rotate + external ring that rotates
pub const IT_HOLDABLE: crate::bg_public_h::itemType_t = 6;
// EFX: static external sphere + rotating internal
pub const IT_POWERUP: crate::bg_public_h::itemType_t = 5;
// EFX: rotate + minlight
pub const IT_HEALTH: crate::bg_public_h::itemType_t = 4;
// EFX: rotate
pub const IT_ARMOR: crate::bg_public_h::itemType_t = 3;
// EFX: rotate + upscale + minlight
pub const IT_AMMO: crate::bg_public_h::itemType_t = 2;
pub const IT_WEAPON: crate::bg_public_h::itemType_t = 1;
pub const IT_BAD: crate::bg_public_h::itemType_t = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmove_t {
    pub ps: *mut crate::src::qcommon::q_shared::playerState_t,
    pub cmd: crate::src::qcommon::q_shared::usercmd_t,
    pub tracemask: i32,
    pub debugLevel: i32,
    pub noFootsteps: crate::src::qcommon::q_shared::qboolean,
    pub gauntletHit: crate::src::qcommon::q_shared::qboolean,
    pub framecount: i32,
    pub numtouch: i32,
    pub touchents: [i32; 32],
    pub mins: crate::src::qcommon::q_shared::vec3_t,
    pub maxs: crate::src::qcommon::q_shared::vec3_t,
    pub watertype: i32,
    pub waterlevel: i32,
    pub xyspeed: f32,
    pub pmove_fixed: i32,
    pub pmove_msec: i32,
    pub trace: Option<
        unsafe extern "C" fn(
            _: *mut crate::src::qcommon::q_shared::trace_t,
            _: *const crate::src::qcommon::q_shared::vec_t,
            _: *const crate::src::qcommon::q_shared::vec_t,
            _: *const crate::src::qcommon::q_shared::vec_t,
            _: *const crate::src::qcommon::q_shared::vec_t,
            _: i32,
            _: i32,
        ) -> (),
    >,
    pub pointcontents:
        Option<unsafe extern "C" fn(_: *const crate::src::qcommon::q_shared::vec_t, _: i32) -> i32>,
}
