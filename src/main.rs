//#![deny(warnings)]
#![allow(non_snake_case)]
#![allow(unused)]
use std::fs;

use warp::Filter;
use warp::body::json;
use warp::reply;

use serde_json::json;
use serde_json::{Result, Value};
use serde::{Deserialize, Serialize};

use sqlx::sqlite::SqlitePoolOptions;
use sqlx::sqlite::SqlitePool;
use sqlx::FromRow;

#[derive(Clone)]
#[derive(Debug, FromRow,Serialize)]
struct Article{
    ID:i64,
    TITLE: String,
    AUTHOR: String,
    STATE: String,
    PERMISSION: i32,
    TEXT: String,
}

struct ArticleList {
    articles: Vec<Article>,
}

#[derive(Clone)]
#[derive(Serialize, Deserialize,Debug)]
struct NavItem {
    navItem: String,
    navItemUrl: String,
}

#[derive(Clone)]
#[derive(Serialize, Deserialize,Debug)]
struct Config {
    avatarUrl: String,
    introduction: String,
    navItems: Vec<NavItem>,
    articleRootPath: String
}

//读取配置文件
fn read_config() -> String{
    let file_path = "./config.cfg";
    let config = 
        fs::read_to_string(&file_path)
        .expect("Failed to read config file");
    //println!("{}", config);
    config
}

// fn get_article_list(database:sqlite::Connection) -> Value{

// }

#[tokio::main]
async fn main() {
    let config:Config = serde_json::from_str(read_config().as_str()).unwrap();//初始化配置文件
    let mut database = SqlitePoolOptions::new()
        .connect("./database.db")
        .await.unwrap();

    let result = sqlx::query_as::<_,Article>(r"SELECT * FROM articles")
        //.map(move |a|{})
        .fetch_all(&database)
        .await.unwrap();
    println!("{:?}", result);
    println!("---------------------------");

    println!("Welcome to little guy's sever console!");
    println!("{:?}", config);
    
    //FUNC展示主页
    //GET / -> Text
    //let show_HomePage = warp::fs::dir("../");
    //成功连接后发送一条欢迎信息
    let say_hello = 
        warp::path::end().map(|| {
            println!("Someone connected!");
            "Welcome to Little Guy's HomePage~
            try to visit ./HomePage.html".replace("    ", "")
        });

    //FUNC获取文章
    //GET /getArticles -> ArticleList
    let get_article = 
        warp::path("getArticles")
        .and(warp::path::param())//参数是文章分区
        .map(move |article_partition: String| {
            format!("The article_partition you request is: {}", article_partition);
            let article_list = json!(result);
            warp::reply::json(&article_list)
        });

    //FUNC 获取个人简介
    //Get /getIntroduction -> Introduction
    let introduction = config.introduction.clone();
    let get_introduction = 
        warp::path("getIntroduction")
        .map(move||{
            let introduction = json!({
                "introduction": {
                    "introduction": introduction
                }
            });
            warp::reply::json(&introduction)
        });

    //FUNC 获取头像
    //Get /getAvatarUrl -> AvataUrl
    let avatar = config.avatarUrl.clone();
    let get_avatar = 
        warp::path("getAvatarUrl")
        .map( move ||{
            let avatar = json!({
                "avatar":{
                    "avatar":avatar
                }
            });
            warp::reply::json(&avatar)
        });

    //FUNC 获取导航栏条目与其绑定的URL
    //GET /getNavItems -> navItems
    let navItems = config.navItems.clone();
    let get_navItems = 
        warp::path("getNavItems")
        .map(move ||{
            let navItems = json!({
                "navItems":navItems
            });
            warp::reply::json(&navItems)
        });

    let routes = 
        //show_HomePage
        say_hello
        .or(get_article)
        .or(get_introduction)
        .or(get_avatar)
        .or(get_navItems);

    warp::serve(routes)
        .run(([0, 0, 0, 0], 80))
        .await;
}