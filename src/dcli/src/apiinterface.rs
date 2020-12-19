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
use crate::error::Error;
use crate::mode::Mode;
use crate::platform::Platform;
use crate::response::activities::{ActivitiesResponse, Activity, MAX_ACTIVITIES_REQUEST_COUNT};
use crate::response::character::CharacterData;
use crate::response::drs::API_RESPONSE_STATUS_SUCCESS;
use crate::response::gpr::{CharacterActivitiesData, GetProfileResponse};
use crate::response::stats::{
    AllTimePvPStatsResponse, DailyPvPStatsResponse, DailyPvPStatsValuesData, PvpStatsData,
};

use crate::utils::Period;

use chrono::{DateTime, Utc};
use std::io::{self, Write};

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
        let response = profile.response.ok_or(Error::ApiRequest {
            description: String::from("No response data from API Call."),
        })?;

        //see if any activities were returned
        //this should never be none when this API is called
        let character_activities = match response.character_activities {
            Some(e) => e,
            None => {
                return Ok(None);
            }
        };

        //store whether use is in an activity
        let mut current_activity: Option<CharacterActivitiesData> = None;

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

        let r_characters = match response.characters {
            Some(e) => e,
            None => {
                return Ok(characters);
            }
        };

        for c in r_characters.data.values() {
            characters.push(c.clone());
        }

        Ok(characters)
    }

    pub async fn retrieve_alltime_crucible_stats(
        &self,
        member_id: &str,
        character_id: &str,
        platform: &Platform,
        mode: &Mode,
    ) -> Result<Option<PvpStatsData>, Error> {
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

        let data: Option<PvpStatsData> = response
            .response
            .ok_or(Error::ApiRequest {
                description: String::from("No response data from API Call."),
            })?
            .data
            .ok_or(Error::ApiRequest {
                description: String::from("No all_pvp data from API Call."),
            })?
            .all_time;

        Ok(data)
    }

    pub async fn retrieve_aggregate_crucible_stats<T: Period>(
        &self,
        member_id: &str,
        character_id: &str,
        platform: &Platform,
        mode: &Mode,
        period: &T,
    ) -> Result<Option<Vec<DailyPvPStatsValuesData>>, Error> {
        let day_start = period.get_start().to_rfc3339();
        let day_end = period.get_end().to_rfc3339();

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

        let data: Option<Vec<DailyPvPStatsValuesData>> = response
            .response
            .ok_or(Error::ApiRequest {
                description: String::from("No response data from API Call."),
            })?
            .data
            .ok_or(Error::ApiRequest {
                description: String::from("No all_pvp data from API Call."),
            })?
            .daily;

        Ok(data)
    }

    pub async fn retrieve_last_activity(
        &self,
        member_id: &str,
        character_id: &str,
        platform: &Platform,
        mode: &Mode,
    ) -> Result<Option<Activity>, Error> {
        let activities = self
            .retrieve_activities(member_id, character_id, platform, mode, 1, 0)
            .await?;

        let activity: Option<Activity> = match activities {
            Some(mut e) => {
                if e.is_empty() {
                    None
                } else {
                    Some(e.remove(0))
                }
            }
            None => None,
        };

        Ok(activity)
    }

    pub async fn retrieve_activities_since(
        &self,
        member_id: &str,
        character_id: &str,
        platform: &Platform,
        mode: &Mode,
        start_time: &DateTime<Utc>,
    ) -> Result<Option<Vec<Activity>>, Error> {
        let mut out: Vec<Activity> = Vec::new();
        let mut page = 0;
        let count = MAX_ACTIVITIES_REQUEST_COUNT;

        eprint!("[");
        //TODO: if error occurs on an individual call, retry?
        loop {
            eprint!("#");
            io::stderr().flush().unwrap();

            // TODO: if we call more pages that there is data, it will return back with no Response
            // property. Usually this means an error but in this case, it just means we have
            // got all of the data. This is only an issue, if they user has a number of activities
            // divisible by MAX_ACTIVITIES_REQUEST_COUNT.
            // We could catch the error and see if its because the response header is missing, and if
            // so assume we are out of data. (maybe compare to whether we have found any items).
            // This would mean we might miss legitimate API errors though.
            let activities = self
                .retrieve_activities(member_id, character_id, platform, mode, count, page)
                .await?;

            if activities.is_none() {
                break;
            }

            let mut t: Vec<Activity> = activities.unwrap();

            let len = t.len() as i32;

            if len == 0 {
                break;
            }

            //filter vector based on whether they are after the filter date / time
            t.retain(|x| x.period > *start_time);

            //if any items were removed, then we dont need to call apis anymore
            let should_break = t.len() as i32 != len;

            //move the items from the temp vec to the out
            out.append(&mut t);

            if should_break || len < count {
                break;
            }

            page += 1;

            //if we try to page past where there is valid data, bungie will return
            //empty response, which we detect retrieve_activities (and returns None)
        }

        eprintln!("] : COMPLETE");

        if out.is_empty() {
            return Ok(None);
        }

        Ok(Some(out))
    }

    pub async fn retrieve_activities_since_id(
        &self,
        member_id: &str,
        character_id: &str,
        platform: &Platform,
        mode: &Mode,
        activity_id: &str,
    ) -> Result<Option<Vec<Activity>>, Error> {

        let mut out: Vec<Activity> = Vec::new();
        let mut page = 0;
        let count = MAX_ACTIVITIES_REQUEST_COUNT;

        let activity_id_str = activity_id.to_string();

        eprint!("[");
        //TODO: if error occurs on an individual call, retry?
        loop {
            eprint!("#");
            io::stderr().flush().unwrap();

            // TODO: if we call more pages that there is data, it will return back with no Response
            // property. Usually this means an error but in this case, it just means we have
            // got all of the data. This is only an issue, if they user has a number of activities
            // divisible by MAX_ACTIVITIES_REQUEST_COUNT.
            // We could catch the error and see if its because the response header is missing, and if
            // so assume we are out of data. (maybe compare to whether we have found any items).
            // This would mean we might miss legitimate API errors though.
            let activities = self
                .retrieve_activities(member_id, character_id, platform, mode, count, page)
                .await?;

            if activities.is_none() {
                break;
            }

            let t: Vec<Activity> = activities.unwrap();

            let len = t.len() as i32;

            if len == 0 {
                break;
            }

            let mut should_break = false;
            for activity in t.into_iter() {
                if activity.details.instance_id == activity_id_str {
                    should_break = true;
                    break;
                }

                out.push(activity);
            }

            if should_break || len < count {
                break;
            }

            page += 1;

            //if we try to page past where there is valid data, bungie will return
            //empty response, which we detect retrieve_activities (and returns None)
        }

        eprintln!("] : COMPLETE");

        if out.is_empty() {
            return Ok(None);
        }

        Ok(Some(out))
    }

    pub async fn retrieve_activities(
        &self,
        member_id: &str,
        character_id: &str,
        platform: &Platform,
        mode: &Mode,
        count: i32,
        page: i32,
    ) -> Result<Option<Vec<Activity>>, Error> {
        //
        let url =
        format!("https://www.bungie.net/Platform/Destiny2/{platform_id}/Account/{member_id}/Character/{character_id}/Stats/Activities/?mode={mode_id}&count={count}&page={page}",
            platform_id = platform.to_id(),
            member_id=utf8_percent_encode(&member_id, NON_ALPHANUMERIC),
            character_id=utf8_percent_encode(&character_id, NON_ALPHANUMERIC),
            mode_id = mode.to_id(),
            count=count,
            page=page,
        );

        let response: ActivitiesResponse = self
            .client
            .call_and_parse::<ActivitiesResponse>(&url)
            .await?;

        //It would be nice to handle the missing response property in call_and_parse
        //but we cant set a trait that requires a property, and since every Response
        //is actually a different type / struct, we cant dynamically specify it to the
        //call_and_parse function. Maybe there is a way to do it, but I have figured it
        //out so for now, we need to write some boilerplate code
        //
        //If we get a response with no Response data, we first check to see if
        //error_status == 1, if it is, it just means there is no more data, and it
        //doesnt mean some error occured.
        let activities = match response.response {
            Some(e) => e,
            None => {
                if response.status.error_code == API_RESPONSE_STATUS_SUCCESS {
                    return Ok(None);
                } else {
                    return Err(Error::ApiRequest {
                        description: String::from("No response data from API Call."),
                    });
                }
            }
        }
        .activities;

        //let activities: Option<Vec<Activity>> = response.activities;

        Ok(activities)
    }
}
