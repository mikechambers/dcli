/*
* Copyright 2021 Mike Chambers
* https://github.com/mikechambers/dcli
*
* Permission is hereby granted, free of charge, to any person obtaining a copy of
* this software and associated documentation files (the "Software"), to deal in
* the Software without restriction, including without limitation the rights to
* use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
* of the Software, and to permit persons to whom the Software is furnished to do
* so, subject to the following conditions:
*
* The above copyright notice and this permission notice shall be included in all
* copies or substantial portions of the Software.
*
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
* FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
* COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
* IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
* CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(PartialEq, Debug, Clone, Deserialize_repr, Serialize_repr)]
#[repr(i32)]
pub enum ItemType {
    Unknown = -1,
    None = 0,
    Currency = 1,
    Armor = 2,
    Weapon = 3,
    Message = 7,
    Engram = 8,
    Consumable = 9,
    ExchangeMaterial = 10,
    MissionReward = 11,
    QuestStep = 12,
    QuestStepComplete = 13,
    Emblem = 14,
    Quest = 15,
    Subclass = 16,
    ClanBanner = 17,
    Aura = 18,
    Mod = 19,
    Dummy = 20,
    Ship = 21,
    Vehicle = 22,
    Emote = 23,
    Ghost = 24,
    Package = 25,
    Bounty = 26,
    Wrapper = 27,
    SeasonalArtifact = 28,
    Finisher = 29,
}

#[derive(PartialEq, Debug, Clone, Deserialize_repr, Serialize_repr)]
#[repr(i32)]
pub enum ItemSubType {
    Unknown = -1,
    None = 0,
    Crucible = 1,
    Vanguard = 2,
    Exotic = 5,
    AutoRifle = 6,
    Shotgun = 7,
    Machinegun = 8,
    HandCannon = 9,
    RocketLauncher = 10,
    FusionRifle = 11,
    SniperRifle = 12,
    PulseRifle = 13,
    ScoutRifle = 14,
    Crm = 16,
    Sidearm = 17,
    Sword = 18,
    Mask = 19,
    Shader = 20,
    Ornament = 21,
    FusionRifleLine = 22,
    GrenadeLauncher = 23,
    SubmachineGun = 24,
    TraceRifle = 25,
    HelmetArmor = 26,
    GauntletsArmor = 27,
    ChestArmor = 28,
    LegArmor = 29,
    ClassArmor = 30,
    Bow = 31,
    DummyRepeatableBounty = 32,
}

impl std::fmt::Display for ItemSubType {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let out = match self {
            ItemSubType::Unknown => "Unknown".to_string(),
            ItemSubType::AutoRifle => "Auto Rifle".to_string(),
            ItemSubType::Machinegun => "Machine Gun".to_string(),
            ItemSubType::HandCannon => "Hand Cannon".to_string(),
            ItemSubType::RocketLauncher => "Rocket Launcher".to_string(),
            ItemSubType::FusionRifle => "Fusion Rifle".to_string(),
            ItemSubType::SniperRifle => "Sniper Rifle".to_string(),
            ItemSubType::PulseRifle => "Pulse Rifle".to_string(),
            ItemSubType::ScoutRifle => "Scout Rifle".to_string(),
            ItemSubType::FusionRifleLine => "Linear Fusion Rifle".to_string(),
            ItemSubType::GrenadeLauncher => "Grenade Launcher".to_string(),
            ItemSubType::SubmachineGun => "Submachine Gun".to_string(),
            ItemSubType::TraceRifle => "Trace Rifle".to_string(),
            ItemSubType::HelmetArmor => "Helmet".to_string(),
            ItemSubType::GauntletsArmor => "Gauntlets".to_string(),
            ItemSubType::ChestArmor => "Chest".to_string(),
            ItemSubType::LegArmor => "Legs".to_string(),
            ItemSubType::ClassArmor => "Class Armor".to_string(),
            _ => format!("{:?}", self),
        };

        write!(f, "{}", out)
    }
}
