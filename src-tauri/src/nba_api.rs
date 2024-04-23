use reqwest::Error; // Import Error type from reqwest
use serde_json::Value; // Import Value type from serde_json
use chrono; // Import chrono

// This struct will be used to stored today's matches
#[derive(Debug)]
pub struct TodayMatches {
    data: Vec<NbaMatch>,
}

// This struct will be used to store match details
#[derive(Debug, Clone)]
pub struct NbaMatch {
    game_id: String,
    game_date_time_est: String,
    game_date_time_utc: String,
    arena_city: String,
    arena_state: String,
    arena_name: String,
    home_team: String,
    away_team: String,
}

impl TodayMatches {
    // Constructor for today_matches
    pub fn new() -> TodayMatches {
        TodayMatches { data: Vec::new() }
    }
    // Add matches to today_matches
    pub async fn get_today_matches(&mut self) -> Result<Vec<NbaMatch>, Error> {
        let body = reqwest
            ::get("https://cdn.nba.com/static/json/staticData/scheduleLeagueV2_1.json").await?
            .json::<Value>().await?;
        // Turn the JSON into an array
        let all_matches = body["leagueSchedule"]["gameDates"].as_array().unwrap();

        // Get today's date
        let today = chrono::Local::now();
        let today = today.format("%m/%d/%Y 00:00:00").to_string();

        // Use binary search to find today's match
        let len = all_matches.len() as u16;
        let today_match_index = self.binary_search_today_match(&all_matches, len, &today).unwrap();
        // Get today's match
        self.extract_matches_from_json(&all_matches[today_match_index]["games"]);

        Ok(self.data.clone())
    }

    // Binary search to find today's match
    fn binary_search_today_match(
        &self,
        all_matches: &Vec<Value>,
        len: u16,
        today: &str
    ) -> Option<usize> {
        let mut low = 0;
        let mut high = len - 1;
        // get the date, month, and year from today
        let dmy = today.split(" ").collect::<Vec<&str>>()[0].split("/").collect::<Vec<&str>>();
        let today_month = dmy[0].parse::<u16>().unwrap();
        let today_day = dmy[1].parse::<u16>().unwrap();
        let today_year = dmy[2].parse::<u16>().unwrap();

        while low <= high {
            let mid = low + (high - low) / 2;
            let mid_index = mid as usize;
            // Get current date, month, and year
            let current_date = all_matches[mid_index]["gameDate"]
                .as_str()
                .unwrap()
                .split(" ")
                .collect::<Vec<&str>>()[0]
                .split("/")
                .collect::<Vec<&str>>();
            let current_month = current_date[0].parse::<u16>().unwrap();
            let current_day = current_date[1].parse::<u16>().unwrap();
            let current_year = current_date[2].parse::<u16>().unwrap();
            // If today's date is equal to current date
            if
                today_month == current_month &&
                today_day == current_day &&
                today_year == current_year
            {
                return Some(mid_index);
            }
            // Check if year is greater than today's year or less than today's year
            if current_year < today_year {
                low = mid + 1;
            } else if current_year > today_year {
                high = mid - 1;
            }
            // Check if month is greater than today's month or less than today's month
            if current_month < today_month {
                low = mid + 1;
            } else if current_month > today_month {
                high = mid - 1;
            }
            // Check if date is greater than today's date or less than today's date
            if current_day < today_day {
                low = mid + 1;
            } else if current_day > today_day {
                high = mid - 1;
            }
        }
        None
    }

    // Extract matches from today's match JSON
    fn extract_matches_from_json(&mut self, today_match: &Value) {
        let match_array = today_match.as_array().unwrap();
        // println!("{:?}", match_array);
        for _match in match_array {
            // println!("{:?}", _match);
            // println!("dsadasdas");
            let game_id = _match["gameId"].as_str().unwrap().to_string();
            let game_date_time_est = _match["gameTimeEst"].as_str().unwrap().to_string();
            let game_date_time_utc = _match["gameTimeUTC"].as_str().unwrap().to_string();
            let arena_city = _match["arenaCity"].as_str().unwrap().to_string();
            let arena_state = _match["arenaState"].as_str().unwrap().to_string();
            let arena_name = _match["arenaName"].as_str().unwrap().to_string();
            let home_team = _match["homeTeam"]["teamName"].as_str().unwrap().to_string();
            let away_team = _match["awayTeam"]["teamName"].as_str().unwrap().to_string();

            // Create a new NbaMatch object
            let nba_match = NbaMatch {
                game_id,
                game_date_time_est,
                game_date_time_utc,
                arena_city,
                arena_state,
                arena_name,
                home_team,
                away_team,
            };

            // Add the match to today_matches
            self.data.push(nba_match);
        }
    }
}
