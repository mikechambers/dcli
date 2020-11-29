/*
* Copyright 2020 Mike Chambers
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

use crate::apiclient::ApiClient;
use crate::response::character::CharacterData;
use crate::error::Error;
use crate::platform::Platform;
use crate::mode::CrucibleMode;
use crate::response::gpr::{GetProfileResponse, CharacterActivitiesData};
use crate::response::stats::{AllTimePvPStatsResponse, PvpStatsData, DailyPvPStatsResponse, DailyPvPStatsValuesData};
use chrono::{DateTime, Utc};

use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

pub struct ApiInterface {
    client: ApiClient,
}

impl ApiInterface {
    pub fn new(print_url: bool) -> ApiInterface {
        ApiInterface {
            client: ApiClient::new(print_url),
        }

        //Have an option on to take a manifest, if manifest is avaliable it will use it
        //some methods may require it and will throw errors if its not set
    }

    
    /// Retrieves characters for specified member_id and platform
    pub async fn retrieve_current_activity(
        &self,
        member_id: String,
        platform: Platform,
    ) -> Result<Option<CharacterActivitiesData>, Error> {

        let url =
            format!("https://www.bungie.net/Platform/Destiny2/{platform_id}/Profile/{member_id}/?components=204",
                platform_id = platform.to_id(),
                member_id=utf8_percent_encode(&member_id, NON_ALPHANUMERIC)
            );

        let profile: GetProfileResponse = self
            .client
            .call_and_parse::<GetProfileResponse>(&url)
            .await?;

        //note: can you ok_or_else if error comp is expensive, since its call
        //everytime with ok_or, but lazily with ok_or_else
        //Note: this should never be None when this API is called
        let response = profile.response.ok_or(
            Error::ApiRequest {
                description: String::from("No response data from API Call."),
            }
        )?;

        //see if any activities were returned
        //this should never be none when this API is called
        let character_activities = match response.character_activities {
            Some(e) => e,
            None => {
                return Ok(None);
            }
        };

        //store whether use is in an activity
        let mut current_activity:Option<CharacterActivitiesData> = None;

        //note, we could grab the char id from the key, and pass it out
        //or even get the char data from the getprofile call
        for c in character_activities.data.values() {

            //if there is a value here, it means this character is currently in
            //an activity
            if c.current_activity_mode_hash != 0 {
                current_activity = Some(c.clone());
                break;
            }
        }

        if current_activity.is_none() {
            //no chars in an activity, so we return None
            return Ok(None);
        }

        //return the raw data for the current activity
        Ok(current_activity)
    }

    /// Retrieves characters for specified member_id and platform
    pub async fn retrieve_characters(
        &self,
        member_id: String,
        platform: Platform,
    ) -> Result<Vec<CharacterData>, Error> {
        let url =
            format!("https://www.bungie.net/Platform/Destiny2/{platform_id}/Profile/{member_id}/?components=200",
                platform_id = platform.to_id(),
                member_id=utf8_percent_encode(&member_id, NON_ALPHANUMERIC)
            );

        let profile: GetProfileResponse = self
            .client
            .call_and_parse::<GetProfileResponse>(&url)
            .await?;

        let response = match profile.response {
            Some(e) => e,
            None => {
                return Err(Error::ApiRequest {
                    description: String::from("No response data from API Call."),
                })
            }
        };

        let mut characters: Vec<CharacterData> = Vec::new();

        let r_characters = match response.characters{
            Some(e) => e,
            None => {
                return Ok(characters);
            },
        };

        for c in r_characters.data.values() {
            characters.push(c.clone());
        }

        Ok(characters)
    }

/*
  static const int daily = 1;
  static const int allTime = 2;
  */

    /*
    async fn retrieve_daily_crucible_stats(member_id:u32, character_id:u32, platform:Platform, mode:CrucibleMode, start_date:DateTime) {
        //"/Platform/Destiny2/1/Account/$memberId/Character/$characterId/Stats/?modes=$modesString$dateRangeString&periodType=$periodTypeId&groups=1,2,3";
    }
    */

    pub async fn retrieve_alltime_crucible_stats(&self, member_id:&str, character_id:&str, platform:Platform, mode:CrucibleMode) -> Result<Option<PvpStatsData>, Error> {
        //"/Platform/Destiny2/1/Account/$memberId/Character/$characterId/Stats/?modes=$modesString$dateRangeString&periodType=$periodTypeId&groups=1,2,3";
        let url =
        format!("https://www.bungie.net/Platform/Destiny2/{platform_id}/Account/{member_id}/Character/{character_id}/Stats/?modes={mode_id}&periodType=2&groups=1,2,3",
            platform_id = platform.to_id(),
            member_id=utf8_percent_encode(&member_id, NON_ALPHANUMERIC),
            character_id=utf8_percent_encode(&character_id, NON_ALPHANUMERIC),
            mode_id = mode.to_id(),
        );

        let response: AllTimePvPStatsResponse = self
        .client
        .call_and_parse::<AllTimePvPStatsResponse>(&url)
        .await?;


        let data:Option<PvpStatsData> = response.response.ok_or(
            Error::ApiRequest {
                description: String::from("No response data from API Call."),
            }
        )?.data.ok_or(
            Error::ApiRequest {
                description: String::from("No all_pvp data from API Call."),
            }
        )?.all_time;

        Ok(data)
    }

    pub async fn retrieve_aggregate_crucible_stats(&self, member_id:&str, character_id:&str, platform:Platform, mode:CrucibleMode, start_time:DateTime<Utc>) -> Result<Option<Vec<DailyPvPStatsValuesData>>, Error> {

        let day_start = start_time.to_rfc3339();
        let day_end = Utc::now().to_rfc3339();

        //
        let url =
        format!("https://www.bungie.net/Platform/Destiny2/{platform_id}/Account/{member_id}/Character/{character_id}/Stats/?modes={mode_id}&periodType=1&groups=1,2,3&daystart={day_start}&dayend={day_end}",
            platform_id = platform.to_id(),
            member_id=utf8_percent_encode(&member_id, NON_ALPHANUMERIC),
            character_id=utf8_percent_encode(&character_id, NON_ALPHANUMERIC),
            mode_id = mode.to_id(),
            day_start = utf8_percent_encode(&day_start, NON_ALPHANUMERIC),
            day_end = utf8_percent_encode(&day_end, NON_ALPHANUMERIC),
        );

        let response: DailyPvPStatsResponse = self
        .client
        .call_and_parse::<DailyPvPStatsResponse>(&url)
        .await?;


        let data:Option<Vec<DailyPvPStatsValuesData>> = response.response.ok_or(
            Error::ApiRequest {
                description: String::from("No response data from API Call."),
            }
        )?.data.ok_or(
            Error::ApiRequest {
                description: String::from("No all_pvp data from API Call."),
            }
        )?.daily;

        Ok(data)
    }

}
