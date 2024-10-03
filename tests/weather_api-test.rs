use mockito::Server;
use team_viewer::weather_api::get_weather;

#[test]
fn test_get_weather() {
    let mut server = Server::new();
    let mock_server = server
        .mock("GET", "/data/2.5/weather")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("q".into(), "London".into()),
            mockito::Matcher::UrlEncoded("appid".into(), "test_api_key".into()),
            mockito::Matcher::UrlEncoded("units".into(), "metric".into()),
        ]))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"
            {
                "main": {
                    "temp": 15.5,
                    "humidity": 70
                },
                "weather": [
                    {
                        "description": "Cloudy"
                    }
                ],
                "timezone": 3600
            }
            "#,
        )
        .create();

    let city = "London";
    let api_key = "test_api_key";

    // Override the base URL for the API call
    std::env::set_var("WEATHER_API_BASE_URL", &server.url());

    // Create a new runtime
    let rt = tokio::runtime::Runtime::new().unwrap();

    // Use block_on to run the async code
    let weather = rt.block_on(async { get_weather(city, api_key).await.unwrap() });

    assert_eq!(weather.main.temp, 15.5);
    assert_eq!(weather.main.humidity, 70);
    assert_eq!(weather.weather[0].description, "Cloudy");
    assert_eq!(weather.timezone, 3600);

    mock_server.assert();
}
