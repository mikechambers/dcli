/*
* Copyright 2022 Mike Chambers
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

use std::{
    collections::HashMap,
    io::{self, Write},
};

use chrono::{DateTime, Utc};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

use crate::response::ggms::{GetGroupMemberResponse, GroupMemberResponse};
use crate::response::gpr::{CharacterActivitiesData, GetProfileResponse};
use crate::response::pgcr::{DestinyPostGameCarnageReportData, PGCRResponse};
use crate::response::sdpr::{
    DestinyLinkedProfilesResponse, LinkedProfilesResponse,
    SearchDestinyPlayerResponse,
};
use crate::response::stats::{
    AllTimePvPStatsResponse, DailyPvPStatsResponse, DailyPvPStatsValuesData,
    PvpStatsData,
};
use crate::response::{
    activities::{ActivitiesResponse, Activity, MAX_ACTIVITIES_REQUEST_COUNT},
    sdpr::SearchDestinyPlayerPostData,
};
use crate::utils::Period;
use crate::{apiclient::ApiClient, crucible::Player};
use crate::{
    apiutils::{API_BASE_URL, PGCR_BASE_URL},
    character::PlayerInfo,
};
use crate::{crucible::Member, response::drs::API_RESPONSE_STATUS_SUCCESS};
use crate::{crucible::PlayerName, error::Error};
use crate::{enums::mode::Mode, response::pgcr::UserInfoCard};
use crate::{
    enums::platform::Platform, response::pgcr::DestinyProfileUserInfoCard,
};

use crate::character::Characters;

pub struct ApiInterface {
    client: ApiClient,
}

impl ApiInterface {
    pub fn new(print_url: bool) -> Result<ApiInterface, Error> {
        let client = ApiClient::new(print_url)?;
        Ok(ApiInterface { client })
    }

    pub fn new_with_key(
        print_url: bool,
        key: &str,
    ) -> Result<ApiInterface, Error> {
        let client = ApiClient::new_with_key(print_url, key)?;
        Ok(ApiInterface { client })

        //Have an option on to take a manifest, if manifest is avaliable it will use it
        //some methods may require it and will throw errors if its not set
    }

    pub async fn retrieve_group_members(
        &self,
        group_id: u32,
    ) -> Result<Vec<Member>, Error> {
        let url = format!(
            "{base}/Platform/GroupV2/{group_id}/Members/",
            base = API_BASE_URL,
            group_id = group_id
        );

        let r: GetGroupMemberResponse = self
            .client
            .call_and_parse::<GetGroupMemberResponse>(&url)
            .await?;

        let response: GroupMemberResponse = match r.response {
            Some(e) => e,
            None => {
                return Err(Error::ApiRequest {
                    description: String::from(
                        "No response data from API Call.",
                    ),
                })
            }
        };

        let mut out: Vec<Member> = Vec::new();
        for info in response.results.iter() {
            out.push(info.destiny_user_info.to_member());
        }

        Ok(out)
    }

    /// Retrieves characters for specified member_id and platform
    pub async fn retrieve_current_activity(
        &self,
        member_id: String,
        platform: Platform,
    ) -> Result<Option<CharacterActivitiesData>, Error> {
        let url = format!(
            "{base}/Platform/Destiny2/{platform_id}/Profile/{member_id}/?components=204",
            base = API_BASE_URL,
            platform_id = platform.as_id(),
            member_id = utf8_percent_encode(&member_id, NON_ALPHANUMERIC)
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

    pub async fn search_destiny_player(
        &self,
        name: &PlayerName,
    ) -> Result<UserInfoCard, Error> {
        let url = format!(
            "{base}/Platform/Destiny2/SearchDestinyPlayerByBungieName/-1/",
            base = API_BASE_URL
        );

        let display_name = match &name.bungie_display_name {
            Some(e) => e.to_string(),
            None => "".to_string(),
        };

        let display_name_code = match &name.bungie_display_name_code {
            Some(e) => e.to_string(),
            None => "".to_string(),
        };

        let post_data = SearchDestinyPlayerPostData {
            display_name,
            display_name_code,
        };

        //this throws an error if it cant be serialized to json
        let post_data_json = post_data.to_json()?;

        let profile: SearchDestinyPlayerResponse = self
            .client
            .call_post_and_parse::<SearchDestinyPlayerResponse>(
                &url,
                &post_data_json,
            )
            .await?;

        //will return none if no response data. No results will return empty vector
        let mut response: Vec<UserInfoCard> = match profile.response {
            Some(e) => e,
            None => {
                return Err(Error::ApiRequest {
                    description: String::from(
                        "No response data from API Call.",
                    ),
                })
            }
        };

        if response.is_empty() {
            return Err(Error::BungieNameNotFound);
        };

        if response.len() == 1 {
            return Ok(response.remove(0));
        };

        let mut cross_save_disabled_found: bool = false;
        let mut cross_save_override_id: Platform = Platform::Unknown;

        for user in response.iter() {
            let tmp: Platform = user.cross_save_override;
            if tmp == Platform::Unknown {
                cross_save_disabled_found = true;
            } else {
                cross_save_override_id = tmp;
            }
        }

        let out = if !cross_save_disabled_found {
            let mut out: UserInfoCard = response.remove(0);
            for user in response.into_iter() {
                if user.cross_save_override == Platform::Unknown
                    || user.membership_type == cross_save_override_id
                {
                    out = user;
                }
            }

            out
        } else {
            let user_info = &response[0];
            let linked_profiles: DestinyLinkedProfilesResponse = self
                .retrieve_linked_profiles(
                    &user_info.membership_id,
                    &user_info.membership_type,
                )
                .await?;

            //TODO: should we have an error here if no profiles are returned?
            //that should not happen

            let mut most_recent: &DestinyProfileUserInfoCard =
                &linked_profiles.profiles[0];

            for profile in linked_profiles.profiles.iter().skip(1) {
                if profile.date_last_played.gt(&most_recent.date_last_played) {
                    most_recent = profile;
                }
            }

            most_recent.to_user_info_card()
        };

        Ok(out)
    }

    pub async fn retrieve_linked_profiles(
        &self,
        member_id: &str,
        platform: &Platform,
    ) -> Result<DestinyLinkedProfilesResponse, Error> {
        let url = format!(
            "{base}/Platform/Destiny2/{platform_id}/Profile/{member_id}/LinkedProfiles/",
            base = API_BASE_URL,
            platform_id = platform.as_id(),
            member_id = utf8_percent_encode(&member_id, NON_ALPHANUMERIC)
        );

        let profile: LinkedProfilesResponse = self
            .client
            .call_and_parse::<LinkedProfilesResponse>(&url)
            .await?;

        let response: DestinyLinkedProfilesResponse = match profile.response {
            Some(e) => e,
            None => {
                return Err(Error::ApiRequest {
                    description: String::from(
                        "No response data from API Call.",
                    ),
                })
            }
        };

        Ok(response)
    }

    pub async fn get_player_info(
        &self,
        member_id: &str,
        platform: &Platform,
    ) -> Result<PlayerInfo, Error> {
        let url = format!(
            "{base}/Platform/Destiny2/{platform_id}/Profile/{member_id}/?components=100,200",
            base = API_BASE_URL,
            platform_id = platform.as_id(),
            member_id = utf8_percent_encode(&member_id, NON_ALPHANUMERIC)
        );

        let profile: GetProfileResponse = self
            .client
            .call_and_parse::<GetProfileResponse>(&url)
            .await?;

        let response = match profile.response {
            Some(e) => e,
            None => {
                return Err(Error::ApiRequest {
                    description: String::from(
                        "No response data from API Call.",
                    ),
                })
            }
        };

        //characters should never be empty
        //todo: test with player with no chars created
        let c = response
            .characters
            .unwrap()
            .data
            .into_iter()
            .map(|(_id, m)| m)
            .collect();

        let characters = Characters::with_characters(c);

        //profile should never be empty
        let user_info = response.profile.unwrap().data.user_info;

        Ok(PlayerInfo {
            characters,
            user_info,
        })
    }

    /// Retrieves characters for specified member_id and platform
    pub async fn retrieve_characters(
        &self,
        member_id: &str,
        platform: &Platform,
    ) -> Result<Option<Characters>, Error> {
        let player_info = self.get_player_info(member_id, platform).await?;

        Ok(Some(player_info.characters))
    }

    pub async fn retrieve_combat_ratings(
        &self,
        players: &[&Player],
        mode: &Mode,
    ) -> Result<HashMap<u64, f32>, Error> {
        let mut futures = Vec::new();
        for p in players {
            let f = self.retrieve_alltime_crucible_stats(
                &p.member_id,
                &p.character_id,
                &p.platform,
                mode,
            );
            futures.push(f);
        }

        let results = futures::future::join_all(futures).await;

        let mut hash: HashMap<u64, f32> = HashMap::new();
        for (i, r) in results.iter().enumerate() {
            let rating = match r {
                Ok(e) => match e {
                    Some(e) => e.combat_rating,
                    None => 0.0,
                },
                Err(_e) => 0.0,
            };

            hash.insert(players[i].calculate_hash(), rating);
        }

        Ok(hash)
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
        format!("{base}/Platform/Destiny2/{platform_id}/Account/{member_id}/Character/{character_id}/Stats/?modes={mode_id}&periodType=2&groups=1,2,3",
            base=API_BASE_URL,
            platform_id = platform.as_id(),
            member_id=utf8_percent_encode(&member_id, NON_ALPHANUMERIC),
            character_id=utf8_percent_encode(&character_id, NON_ALPHANUMERIC),
            mode_id = mode.as_id(),
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
        format!("{base}/Platform/Destiny2/{platform_id}/Account/{member_id}/Character/{character_id}/Stats/?modes={mode_id}&periodType=1&groups=1,2,3&daystart={day_start}&dayend={day_end}",
            base=API_BASE_URL,
            platform_id = platform.as_id(),
            member_id=utf8_percent_encode(&member_id, NON_ALPHANUMERIC),
            character_id=utf8_percent_encode(&character_id, NON_ALPHANUMERIC),
            mode_id = mode.as_id(),
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
            eprint!(".");
            io::stderr().flush().unwrap();

            // TODO: if we call more pages that there is data, it will return back with no Response
            // property. Usually this means an error but in this case, it just means we have
            // got all of the data. This is only an issue, if they user has a number of activities
            // divisible by MAX_ACTIVITIES_REQUEST_COUNT.
            // We could catch the error and see if its because the response header is missing, and if
            // so assume we are out of data. (maybe compare to whether we have found any items).
            // This would mean we might miss legitimate API errors though.
            let activities = self
                .retrieve_activities(
                    member_id,
                    character_id,
                    platform,
                    mode,
                    count,
                    page,
                )
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
        activity_id: i64,
    ) -> Result<Option<Vec<Activity>>, Error> {
        let mut out: Vec<Activity> = Vec::new();
        let mut page = 0;
        let count = MAX_ACTIVITIES_REQUEST_COUNT;

        eprint!("[");
        //TODO: if error occurs on an individual call, retry?
        loop {
            eprint!(".");
            io::stderr().flush().unwrap();

            // TODO: if we call more pages that there is data, it will return back with no Response
            // property. Usually this means an error but in this case, it just means we have
            // got all of the data. This is only an issue, if they user has a number of activities
            // divisible by MAX_ACTIVITIES_REQUEST_COUNT.
            // We could catch the error and see if its because the response header is missing, and if
            // so assume we are out of data. (maybe compare to whether we have found any items).
            // This would mean we might miss legitimate API errors though.
            let activities = self
                .retrieve_activities(
                    member_id,
                    character_id,
                    platform,
                    mode,
                    count,
                    page,
                )
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
                if activity.details.instance_id == activity_id {
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

        eprintln!("]");

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
        format!("{base}/Platform/Destiny2/{platform_id}/Account/{member_id}/Character/{character_id}/Stats/Activities/?mode={mode_id}&count={count}&page={page}",
            base=API_BASE_URL,
            platform_id = platform.as_id(),
            member_id=utf8_percent_encode(&member_id, NON_ALPHANUMERIC),
            character_id=utf8_percent_encode(&character_id, NON_ALPHANUMERIC),
            mode_id = mode.as_id(),
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
                        description: String::from(
                            "No response data from API Call.",
                        ),
                    });
                }
            }
        }
        .activities;

        //let activities: Option<Vec<Activity>> = response.activities;

        Ok(activities)
    }

    pub async fn retrieve_post_game_carnage_report(
        &self,
        instance_id: i64,
    ) -> Result<Option<DestinyPostGameCarnageReportData>, Error> {
        //TODO: do we need to use baseurls?
        let url = format!(
            "{base}/Platform/Destiny2/Stats/PostGameCarnageReport/{instance_id}/",
            base = PGCR_BASE_URL,
            instance_id = instance_id,
        );

        let response: PGCRResponse =
            self.client.call_and_parse::<PGCRResponse>(&url).await?;

        let data: DestinyPostGameCarnageReportData = match response.response {
            Some(e) => e,
            None => {
                if response.status.error_code == API_RESPONSE_STATUS_SUCCESS {
                    return Ok(None);
                } else {
                    return Err(Error::ApiRequest {
                        description: String::from(
                            "No response data from API Call.",
                        ),
                    });
                }
            }
        };

        Ok(Some(data))
    }
}
