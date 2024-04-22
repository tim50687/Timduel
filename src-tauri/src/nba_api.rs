
// This struct will be used to store the data of the matches
strcut nba_match {
    game_id: String,
    game_date_time_est: String,
    game_date_time_utc: String,
    arena_city: String,
    arena_state: String,
    arena_name: String,
    home_team: String,
    away_team: String,
}

fn get_today_matches() -> Vec<nba_match> {
    return Vec::new();
}