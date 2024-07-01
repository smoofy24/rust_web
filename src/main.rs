use warp::Filter;
use std::collections::HashMap;
use is_prime::*;
use rand::Rng;
use log::info;

#[tokio::main]
async fn main() {

    env_logger::init();

    // HTML index
    let index = warp::path::end()
        .map(|| {
            warp::reply::html(
                r#"
                <!DOCTYPE html>
                <html lang="en">
                <head>
                    <meta charset="UTF-8">
                    <meta name="viewport" content="width=device-width, initial-scale=1.0">
                    <title>Primes</title>
                </head>
                <body>
                    <h1>Choose an Action</h1>
                    <form action="/prime_check" method="post">
                        <button type="submit">Prime checker</button>
                    </form>
                    <form action="/prime_generator" method="post">
                        <button type="submit">Prime generator</button>
                    </form>
                    <form action="/prime_game" method="post">
                        <button type="submit">Prime game</button>
                    </form>
                </body>
                </html>
                "#,
            )
        });

    // HTML form for prime checker
    let form_html_checker = r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Enter a Digit</title>
            <script>
                async function handleSubmit(event) {
                    event.preventDefault();
                    const form = event.target;
                    const formData = new FormData(form);
                    const response = await fetch(form.action, {
                        method: form.method,
                        body: new URLSearchParams(formData)
                    });
                    const result = await response.text();
                    document.getElementById('result').innerHTML = result;
                }
            </script>
        </head>
        <body>
            <h1>Enter a Digit to Check Prime</h1>
            <form action="/submit_number" method="post" onsubmit="handleSubmit(event)">
                <input type="number" name="digit" required>
                <button type="submit">Submit</button>
            </form>
            <div id="result"></div>
        </body>
        </html>
    "#;

    // HTML form for prime generator
    let form_html_generator = r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Enter a Number</title>
            <script>
                async function handleSubmit(event) {
                    event.preventDefault();
                    const form = event.target;
                    const formData = new FormData(form);
                    const response = await fetch(form.action, {
                        method: form.method,
                        body: new URLSearchParams(formData)
                    });
                    const result = await response.text();
                    document.getElementById('result').innerHTML = result;
                }
            </script>
        </head>
        <body>
            <h1>Enter a Number to Generate Primes</h1>
            <form action="/generate_primes" method="post" onsubmit="handleSubmit(event)">
                <input type="number" name="digit" required>
                <button type="submit">Submit</button>
            </form>
            <div id="result"></div>
        </body>
        </html>
    "#;

    // HTML form for prime game
    let form_html_game = r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Prime Game</title>
            <script>
                async function handleSubmit(event) {
                    event.preventDefault();
                    const form = event.target;
                    const formData = new FormData(form);
                    const response = await fetch(form.action, {
                        method: form.method,
                        body: new URLSearchParams(formData)
                    });
                    const result = await response.text();
                    document.getElementById('result').innerHTML = result;
                }
            </script>
        </head>
        <body>
            <h1>Is it Prime?</h1>
            <form action="/check_prime_game" method="post" onsubmit="handleSubmit(event)">
                <input type="hidden" name="number" value="{number}">
                <p>Is {number} a prime number?</p>
                <button type="submit" name="answer" value="yes">Yes</button>
                <button type="submit" name="answer" value="no">No</button>
            </form>
            <div id="result"></div>
        </body>
        </html>
    "#;

    // Handle prime checker
    let prime_check = warp::path("prime_check")
        .and(warp::post())
        .map(move || {
            warp::reply::html(form_html_checker)
        });

    // Handle prime generator
    let prime_generator = warp::path("prime_generator")
        .and(warp::post())
        .map(move || {
            warp::reply::html(form_html_generator)
        });

    // Handle prime game
    let prime_game = warp::path("prime_game")
        .and(warp::post())
        .map(move || {
            let number = rand::thread_rng().gen_range(2..101);
            warp::reply::html(form_html_game.replace("{number}", &number.to_string()))
        });

    // Handle form submission for prime checker
    let submit_number = warp::path("submit_number")
        .and(warp::post())
        .and(warp::body::form())
        .map(|form: HashMap<String, String>| {
            if let Some(digit) = form.get("digit") {
                let digit: u64 = digit.parse().unwrap_or(0);
                if is_prime(&digit.to_string()) {
                    format!("<p>{} is a prime number.</p>", digit)
                } else {
                    format!("<p>{} is not a prime number.</p>", digit)
                }
            } else {
                "<p>No digit entered.</p>".to_string()
            }
        });

    // Handle form submission for prime generator
    let generate_primes = warp::path("generate_primes")
        .and(warp::post())
        .and(warp::body::form())
        .map(|form: HashMap<String, String>| {
            if let Some(digit) = form.get("digit") {
                let max: u64 = digit.parse().unwrap_or(0);
                let primes: Vec<u64> = (2..=max).filter(|&n| is_prime(&n.to_string())).collect();
                format!("<p>Primes up to {}: {:?}</p>", max, primes)
            } else {
                "<p>No digit entered.</p>".to_string()
            }
        });

    // Handle form submission for prime game
    let check_prime_game = warp::path("check_prime_game")
        .and(warp::post())
        .and(warp::body::form())
        .map(|form: HashMap<String, String>| {
            info!("Received form: {:?}", form);

            if let Some(number_str) = form.get("number") {
                info!("Number received: {}", number_str);

                let number: u64 = number_str.parse().unwrap_or(0);
                info!("Parsed number: {}", number);

                let correct_answer = is_prime(&number.to_string());
                info!("Correct answer (is prime): {}", correct_answer);

                let user_answer = form.get("answer").map(|s| s == "yes").unwrap_or(false);
                info!("User answer: {}", user_answer);

                if user_answer == correct_answer {
                    format!("<p>Correct! {} is {}a prime number.</p>", number, if correct_answer { "" } else { "not " })
                } else {
                    format!("<p>Incorrect. {} is {}a prime number.</p>", number, if correct_answer { "" } else { "not " })
                }
            } else {
                info!("No number provided.");
                "<p>No number provided.</p>".to_string()
            }
        });

    // Combine the routes
    let routes = index.or(prime_check).or(prime_generator).or(prime_game).or(submit_number).or(generate_primes).or(check_prime_game);

    // Start the server
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
