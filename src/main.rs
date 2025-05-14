use warp::Filter;

#[tokio::main]
async fn main() {
    // Определим путь для сложения
    let add = warp::path("add")
        .and(warp::get())
        .and(warp::query::<std::collections::HashMap<String, String>>())
        .map(|query: std::collections::HashMap<String, String>| {
            let a = query.get("a").and_then(|a| a.parse::<f64>().ok());
            let b = query.get("b").and_then(|b| b.parse::<f64>().ok());

            match (a, b) {
                (Some(a), Some(b)) => warp::reply::json(&serde_json::json!({ "result": a + b })),
                _ => warp::reply::json(&serde_json::json!({ "error": "Параметры 'a' и 'b' обязательны" })),
            }
        });

    // Определим путь для вычитания
    let subtract = warp::path("subtract")
        .and(warp::get())
        .and(warp::query::<std::collections::HashMap<String, String>>())
        .map(|query: std::collections::HashMap<String, String>| {
            let a = query.get("a").and_then(|a| a.parse::<f64>().ok());
            let b = query.get("b").and_then(|b| b.parse::<f64>().ok());

            match (a, b) {
                (Some(a), Some(b)) => warp::reply::json(&serde_json::json!({ "result": a - b })),
                _ => warp::reply::json(&serde_json::json!({ "error": "Параметры 'a' и 'b' обязательны" })),
            }
        });

    // Объединяем все пути в один роутер
    let routes = add.or(subtract);

    // Запуск сервера на порту 3005
    warp::serve(routes)
        .run(([0, 0, 0, 0], 3005)) // было: [127, 0, 0, 1]
        .await;

}
