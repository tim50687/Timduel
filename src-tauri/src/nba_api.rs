use reqwest::Error; // Import Error type from reqwest

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
    pub async fn get_today_matches(&self) -> Result<Vec<NbaMatch>, Error> {
        let body =
            reqwest::get("https://cdn.nba.com/static/json/staticData/scheduleLeagueV2_1.json")
                .await?
                .text()
                .await?;
        println!("{:?}", body);
        println!("dsadas");
        Ok(self.data.clone())
    }
}
