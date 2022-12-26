use std::{ops::Sub, sync::mpsc::channel, thread, time::Duration};

use sqlx::{SqliteConnection, Connection, sqlite::SqliteValue};

use futures::TryStreamExt;

#[derive(Debug)]
struct Timer {
    total: Duration,
    remaining: Duration,
}

impl Timer {
    pub fn new(total: Duration) -> Self {
        Self {
            total,
            remaining: total,
        }
    }
}


fn build_database(conn: &SqliteConnection) {

}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {

    let mut conn = SqliteConnection::connect("file:data.db").await.unwrap();
    let (tx, rx) = channel::<usize>();

    let resp =  match sqlx::query("SELECT *")
        .fetch_one(&mut conn)
        .await {
            Ok(row) => Some(row),
            Err(_) => {
                build_database(&conn);
                None
            } 
        };


    let row = sqlx::query("INSERT INTO pomodoro (task_id, start, end) VALUES (? ? ?)");

    let mut t = Timer::new(Duration::from_secs(10));
    println!("total: {}", t.total.as_secs());



    let timer = thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(1));
        match t.remaining {
            Duration::ZERO => {
                break;
            }

            _ => {
                t.remaining = t.remaining.sub(Duration::from_secs(1));
                tx.send(t.remaining.as_secs().try_into().unwrap()).unwrap();
            }
        }
    });

    thread::spawn(move || loop {
        if let Ok(msg) = rx.recv() {
            println!("remaining: {}", msg);
        }
    });

    if timer.join().is_ok() {
        println!("finished!")
    }

    Ok(())
}
