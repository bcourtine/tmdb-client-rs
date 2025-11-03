//! Example of using the async TMDB API client
//!
//! Run with:
//! ```bash
//! export TMDB_API_KEY="your_api_key_here"
//! cargo run --example async_example --features async
//! ```

#[cfg(feature = "async")]
use tmdb_client::async_apis::AsyncAPIClient;
#[cfg(feature = "async")]
use tmdb_client::Error;

#[cfg(feature = "async")]
#[tokio::main]
async fn main() -> Result<(), Error> {
    // Initialize the async client from environment variable
    let client = AsyncAPIClient::new_from_env();

    println!("🎬 TMDB Async API Client Example\n");

    // Example 1: Get popular movies
    println!("📊 Fetching popular movies...");
    let popular = client
        .movies_api()
        .get_movie_popular_paginated(None, Some(1), None)
        .await?;

    if let Some(ref results) = popular.results {
        println!("✓ Found {} popular movies on page 1", results.len());
        if let Some(first_movie) = results.first() {
            println!(
                "  Top movie: {}",
                first_movie.title.as_ref().unwrap_or(&"Unknown".to_string())
            );
        }
    }
    println!();

    // Example 2: Get specific movie details (Fight Club - ID: 550)
    println!("🎥 Fetching movie details for Fight Club (ID: 550)...");
    let movie = client
        .movies_api()
        .get_movie_details(550, None, None, None)
        .await?;

    println!("✓ Title: {}", movie.title.unwrap_or_default());
    println!("  Overview: {}", movie.overview.unwrap_or_default().chars().take(100).collect::<String>());
    println!("  Release date: {}", movie.release_date.unwrap_or_default());
    println!("  Rating: {}", movie.vote_average.unwrap_or(0.0));
    println!();

    // Example 3: Search for movies
    println!("🔍 Searching for 'Inception'...");
    let search_results = client
        .search_api()
        .get_search_movie_paginated("Inception", None, None, None, None, None, None)
        .await?;

    if let Some(ref results) = search_results.results {
        println!("✓ Found {} results", results.len());
        for (i, movie) in results.iter().take(5).enumerate() {
            let title = movie.title.as_ref().map(|s| s.as_str()).unwrap_or("Unknown");
            println!("  {}. {}", i + 1, title);
        }
    }
    println!();

    // Example 4: Concurrent requests (demonstrating async power)
    println!("⚡ Fetching multiple movie details concurrently...");
    let movie_ids = vec![550, 13, 680]; // Fight Club, Forrest Gump, Pulp Fiction

    let futures: Vec<_> = movie_ids
        .iter()
        .map(|&id| client.movies_api().get_movie_details(id, None, None, None))
        .collect();

    let movies = futures::future::join_all(futures).await;

    println!("✓ Fetched {} movies concurrently:", movies.len());
    for movie_result in movies {
        match movie_result {
            Ok(movie) => println!("  - {}", movie.title.unwrap_or_default()),
            Err(e) => println!("  - Error: {}", e),
        }
    }

    println!("\n✅ All examples completed successfully!");
    Ok(())
}

#[cfg(not(feature = "async"))]
fn main() {
    eprintln!("This example requires the 'async' feature to be enabled.");
    eprintln!("Run with: cargo run --example async_example --features async");
    std::process::exit(1);
}
