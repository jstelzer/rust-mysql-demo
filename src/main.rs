extern crate mysql;
extern crate clap;

use mysql::*;
use mysql::prelude::*;
use clap::{Arg, App};
use std::io::{self};

#[derive(Debug, PartialEq, Eq)]
struct Payment {
    customer_id: i32,
    amount: i32,
    account_name: Option<String>,
}

fn check_db_pulse(mut conn: mysql::PooledConn){

    // Let's create a table for payments.
    conn.query_drop(
        r"CREATE  TABLE IF NOT EXISTS payment (
            row_num int NOT NULL AUTO_INCREMENT,
            customer_id int not null,
            amount int not null,
            account_name text,
            PRIMARY KEY (row_num)
        )").unwrap();
    // mocked up payment data but it'll write to the db...
    let payments = vec![
        Payment { customer_id: 1, amount: 2, account_name: None },
        Payment { customer_id: 3, amount: 4, account_name: Some("foo".into()) },
        Payment { customer_id: 5, amount: 6, account_name: None },
        Payment { customer_id: 7, amount: 8, account_name: None },
        Payment { customer_id: 9, amount: 10, account_name: Some("bar".into()) },
    ];

    // Now let's insert payments to the database
    conn.exec_batch(
        r"INSERT INTO payment (customer_id, amount, account_name)
          VALUES (:customer_id, :amount, :account_name)",
        payments.into_iter().map(|p| params! {
            "customer_id" => p.customer_id,
            "amount" => p.amount,
            "account_name" => &p.account_name,
        })
    ).expect("Inserts failed");

    // Let's select payments from database. Type inference should do the trick here.
    // i was originally getting it all, but that just slows down over time. So I capped it.
    let _selected_payments = conn
        .query_map(
            "SELECT customer_id, amount, account_name from payment limit 10",
            |(customer_id, amount, account_name)| {
                Payment { customer_id, amount, account_name }
            },
        ).expect("Select didn't work");
    // all the sum() work here is on the db. We get a nice single row with one cell for the
    // count. So this is pretty fixed size.
    let row_count: Result<std::vec::Vec<String>> = conn.query("SELECT count(1) from payment");
    println!("History row count: {:?}", row_count);
    println!("Yay!\n\n");
}

/*
i suppose i could do the right thing with args.....

hostname
username
prompt-for-password
port
database-name
*/
fn generate_mysql_url() -> String {
    let matches = App::new("Mysql pulse checker 9000")
    .version("0.0.1")
    .author("Jason Stelzer <jason.stelzer@gmail.com>")
    .about("Keeps a mysql database busy while you do potentially disruptive RDS things to it.")
    .arg(Arg::with_name("hostname")
        .short('h')
        .long("hostname")
        .about("Hostname to connect to")
        .required(true)
        .takes_value(true))
    .arg(Arg::with_name("username")
        .short('u')
        .long("username")
        .about("Username to connect as")
        .required(true)
        .takes_value(true))  
    .arg(Arg::with_name("port")
        .short('p')
        .long("port")
        .about("The tcp port number to connect to")
        .required(true)
        .takes_value(true))
    .arg(Arg::with_name("dbname")
        .short('d')
        .long("dbname")
        .required(true)
        .about("The database name to 'use' on mysql")
        .takes_value(true))
    .arg(Arg::with_name("prompt")
        .short('P')
        .long("prompt")
        .about("Prompt for a password via stdin")
        .takes_value(false))
    .get_matches();
    let mut password: String = String::new();
    if matches.is_present("prompt"){
        password = get_password();
    }
    let p: &str = &password[..];
    let h = matches.value_of("hostname").unwrap();
    let u = matches.value_of("username").unwrap();
    let port = matches.value_of("port").unwrap();
    let dbname = matches.value_of("dbname").unwrap();
    format!("mysql://{}:{}@{}:{}/{}", u, p, h, port, dbname)
}

fn get_password() -> String {
    let mut buffer = String::new();
    println!("Enter your password:");
    io::stdin().read_line(&mut buffer).expect("Unable to read stdin.");
    if buffer.ends_with('\n') {
        buffer.pop();
        if buffer.ends_with('\r') {
            buffer.pop();
        }
    }
    buffer
}

fn main() -> (){
    // this is a little contrived.
    // I'm doing the db setup and going into a loop. Thought process is:
    //
    // If the conn breaks, the code should stop.
    //
    // I'm intentionally not doing a ping() check or reconnect in order to make
    // this stuff as brittle as possible. 
    //
    // Essentially, if this works then we know we aren't dropping anything.
    let url = generate_mysql_url();
    println!("Connecting to: {}", url);
    let pool = Pool::new(url).expect("url didn't parse");
    while ! std::path::Path::new("./disable-test.txt").exists() {
        let conn = pool.get_conn().expect("Connection didn't work");        
        check_db_pulse(conn);
    }
}
