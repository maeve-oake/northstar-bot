use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::all::*;

use super::prefixes::check_db_prefix;

#[command]
pub async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(
            ctx,
            "```Diff
+ Here is a list of all available commands: \n
-[HELP]
${config.prefix}help                   - displays this message
${config.prefix}prefix [prefix]        - allows an admin to set the prefix
-[Northstar]
${config.prefix}status                 - a general overview of northstar.tf
${config.prefix}search title [string]  - searches server titles
${config.prefix}search mode [gamemode] - searches all servers running that mode
${config.prefix}search map [map]       - searches all servers running that map
-[Titanfall 2 Lists]
${config.prefix}playlistvars           - lists some useful playlist vars
${config.prefix}modes                  - lists all Titanfall 2 gamemodes
${config.prefix}maps                   - lists all Titanfall 2 maps
-[Links]
${config.prefix}info                   - display info about the bot
${config.prefix}host                   - links hummusbird's server tutorial
${config.prefix}git                    - links the github
${config.prefix}wiki                   - links the wiki
```"
            .replace(
                "${config.prefix}",
                check_db_prefix(msg.guild_id).unwrap().as_str(),
            ),
        )
        .await?;
    Ok(())
}

#[command]
#[aliases(map)]
pub async fn maps(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(
            ctx,
            "```Diff
+ Multiplayer Maps
mp_angel_city        - Angel City
mp_black_water_canal - Black Water Canal
mp_grave             - Boomtown
mp_colony02          - Colony
mp_complex3          - Complex
mp_crashsite3        - Crashsite
mp_drydock           - DryDock
mp_eden              - Eden
mp_thaw              - Exoplanet
mp_forwardbase_kodai - Forward Base Kodai
mp_glitch            - Glitch
mp_homestead         - Homestead
mp_relic02           - Relic
mp_rise              - Rise
mp_wargames          - Wargames
mp_lobby             - Lobby
mp_lf_deck           - Deck
mp_lf_meadow         - Meadow
mp_lf_stacks         - Stacks
mp_lf_township       - Township
mp_lf_traffic        - Traffic
mp_lf_uma            - UMA
mp_coliseum          - The Coliseum
mp_coliseum_column   - Pillars
mp_box               - Box
+ Singleplayer Maps
sp_training          -The Pilot's Gauntlet
sp_crashsite         -BT-7274
sp_sewers1           -Blood and Rust
sp_boomtown_start    -Into the Abyss - Part 1
sp_boomtown          -Into the Abyss - Part 2
sp_boomtown_end      -Into the Abyss - Part 2
sp_hub_timeshift     -Effect and Cause - Part 1 or 3
sp_timeshift_spoke02 -Effect and Cause - Part 2
sp_beacon            -The Beacon - Part 1 or 3
sp_beacon_spoke0     -The Beacon - Part 2
sp_tday              -Trial by Fire
sp_s2s               -The Ark
sp_skyway_v1         -The Fold Weapon```",
        )
        .await?;
    Ok(())
}

#[command]
#[aliases(mode, gamemode, gamemodes)]
pub async fn modes(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(
            ctx,
            "```Diff
+ Vanilla
aitdm               -Attrition
at                  -Bounty Hunt
coliseum            -Coliseum
cp                  -Amped Hardpoint
ctf                 -Capture the Flag
fd_easy             -Frontier Defense Easy
fd_normal           -Frontier Defense Regular
fd_hard             -Frontier Defense Hard
fd_insane           -Frontier Defense Insane
fd_master           -Frontier Defense Master
lts                 -Last Titan Standing
mfd                 -Marked For Death
ps                  -Pilots vs. Pilots
solo                -Campaign
tdm                 -Skirmish
ttdm                -Titan Brawl
lf                  -Live Fire
+ Vanilla (Featured)
alts                -Aegis Last Titan Standing
attdm               -Aegis Titan Brawl
ffa                 -Free For All
fra                 -Free Agents
holopilot_lf        -The Great Bamboozle
rocket_lf           -Rocket Arena
turbo_lts           -Turbo Last Titan Standing
turbo_ttdm          -Turbo Titan Brawl
+ Northstar.Custom
chamber             -One in the Chamber
ctf_comp            -Competitive CTF
fastball            -Fastball
gg                  -Gun Game
hidden              -The Hidden
hs                  -Hide and Seek
inf                 -Infection
kr                  -Amped Killrace
sns                 -Sticks and Stones
tffa                -Titan FFA
tt                  -Titan Tag
fw                  -Frontier War
+ Northstar.Coop
sp_coop             -Singleplayer Coop```",
        )
        .await?;
    Ok(())
}

#[command]
#[aliases(playlist, playlistvar)]
pub async fn playlistvars(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(
            ctx,
            "```Diff
+ Playlist Vars
custom_air_accel_pilot
pilot_health_multiplier
run_epilogue
respawn_delay
boosts_enabled
earn_meter_pilot_overdrive
earn_meter_pilot_multiplier
earn_meter_titan_multiplier
aegis_upgrades
infinite_doomed_state
titan_shield_regen
scorelimit
roundscorelimit
timelimit
oob_timer_enabled
roundtimelimit
classic_rodeo
classic_mp
fp_embark_enabled
promode_enable
riff_floorislava
featured_mode_all_holopilot
featured_mode_all_grapple
featured_mode_all_phase
featured_mode_all_ticks
featured_mode_tactikill
featured_mode_amped_tacticals
featured_mode_rocket_arena
featured_mode_shotguns_snipers
iron_rules
riff_player_bleedout
player_bleedout_forceHolster
player_bleedout_forceDeathOnTeamBleedout
player_bleedout_bleedoutTime
player_bleedout_firstAidTime
player_bleedout_firstAidTimeSelf
player_bleedout_firstAidHealPercent
player_bleedout_aiBleedingPlayerMissChance

+ Sticks and Stones Playlist Vars
sns_softball_enabled
sns_softball_kill_value
sns_wme_kill_value
sns_offhand_kill_value
sns_reset_kill_value
sns_melee_kill_value
sns_reset_pulse_blade_cooldown_on_pulse_blade_kill
```",
        )
        .await?;
    Ok(())
}
