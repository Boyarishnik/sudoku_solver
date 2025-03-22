use askama::Template;
use axum::{
    extract::Form,
    response::Html,
    routing::{get, post},
    Router,
};
use std::{collections::HashMap, net::SocketAddr};
use sudoku_rules::Field;
use tokio::net::TcpListener;
// use sudoku_rules::Field;

#[derive(Template)]
#[template(path = "index.html")]
struct SudokuTemplate {
    funcs: AskamaFunctions,
}

struct AskamaFunctions;

impl AskamaFunctions {
    fn get_color(&self, i: &i32, j: &i32) -> String {
        let col = j / 3;
        let row = i / 3;

        format!("rgb{:?}", ((2 - col) * 100, col * 100, row * 100))
    }
}

async fn render_sudoku() -> Html<String> {
    let template = SudokuTemplate {
        funcs: AskamaFunctions,
    };
    Html(template.render().unwrap())
}

async fn solve_sudoku(Form(data): Form<HashMap<String, String>>) -> Html<String> {
    let mut f = Field::new();

    for (i, j) in data {
        if let [row, col] = i[4..]
            .split('-')
            .map(|x| x.parse().unwrap())
            .collect::<Vec<_>>()[..]
        {
            if !j.is_empty() {
                if let Err(_) = f.try_push((row, col), j.parse().unwrap_or(0)) {
                    return Html(String::from("Некорректно заполнены данные"));
                }
            }
        }
    }

    if let Err(_) = f.solve() {
        return Html(String::from("Оно нерешаемо:("));
    }
    Html(format!("{f}").replace("\n", "<br>"))
}

#[tokio::main]
async fn main() {
    let router: Router = Router::new()
        .route("/", get(render_sudoku))
        .route("/solve", post(solve_sudoku));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

    let listener = TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, router).await.unwrap();
}
