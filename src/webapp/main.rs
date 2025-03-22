use askama::Template;
use axum::{
    response::Response,
    extract::Form,
    response::Html,
    routing::{get, post},
    Router,
};
use std::{collections::HashMap, net::SocketAddr};
use sudoku_rules::Field;
use tokio::{fs::File, io::AsyncReadExt, net::TcpListener};

#[derive(Template)]
#[template(path = "index.html")]
struct SudokuTemplate {
    funcs: AskamaFunctions,
}

#[derive(Template)]
#[template(path = "solve.html")]
struct SolveTemplate {
    field: Field,
    funcs: AskamaFunctions,
}

struct AskamaFunctions;

impl AskamaFunctions {
    fn get_color(&self, i: &usize, j: &usize) -> String {
        let col = j / 3;
        let row = i / 3;

        format!("rgb{:?}", ((2 - col) * 100, col * 100, row * 100))
    }

    fn get_shadow_color(&self, i: &usize, j: &usize) -> String {
        let col = j / 3;
        let row = i / 3;

        format!("rgb{:?}", ((2 - col) * 40, col * 40, row * 40))
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
        if !j.is_empty() {
            if let [row, col] = i[4..]
                .split('-')
                .map(|x| x.parse().unwrap())
                .collect::<Vec<_>>()[..]
            {
                if let Err(_) = f.try_push((row, col), j.parse().unwrap()) {
                    return Html(String::from("Некорректно заполнены данные"));
                }
            }
        }
    }

    if let Err(_) = f.solve() {
        return Html(String::from("Оно нерешаемо:("));
    }

    let template = SolveTemplate {
        field: f,
        funcs: AskamaFunctions,
    };
    
    Html(template.render().unwrap())
}

async fn styles() -> Response<String> {
    let path = "templates/styles.css";
    let mut css = File::open(path).await.unwrap();
    let mut buf = String::new();

    css.read_to_string(&mut buf).await.unwrap();

    Response::builder()
        .header("Content-Type", "text/css")
        .body(buf)
        .unwrap()
}

#[tokio::main]
async fn main() {
    let router: Router = Router::new()
        .route("/", get(render_sudoku))
        .route("/solve", post(solve_sudoku))
        .route("/styles", get(styles));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    let listener = TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, router).await.unwrap();
}
