use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};


#[tokio::main]
async fn main() {
    // Инициализация tracing (логирование)
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "aelysion_backend=debug,tower_http=debug".into()), // Уровни логирования по умолчанию
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Initializing Aelysion backend...");

    // Создаем роутер с одним маршрутом
    let app = Router::new()
        .route("/", get(root_handler)); // GET / будет обрабатываться функцией root_handler

    // Определяем адрес и порт для сервера
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000)); // Слушаем на http://127.0.0.1:3000
    tracing::info!("Listening on {}", addr);

    // Запускаем сервер
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service()) // Используем axum::serve
        .await
        .unwrap();
}

// Простой обработчик для корневого маршрута
async fn root_handler() -> &'static str {
    tracing::debug!("Root handler accessed");
    "Welcome to Aelysion!" // Простое текстовое приветствие
}