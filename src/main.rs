extern crate mysql;

use mysql::*;
use mysql::prelude::*;
//use mysql::Error;

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
    let _selected_payments = conn
        .query_map(
            "SELECT customer_id, amount, account_name from payment",
            |(customer_id, amount, account_name)| {
                Payment { customer_id, amount, account_name }
            },
        ).expect("Select didn't work");

    let row_count: Result<std::vec::Vec<String>> = conn.query("SELECT count(1) from payment");
    println!("History row count: {:?}", row_count);
    println!("Yay!\n\n");
}

fn main() -> (){
    let url = "mysql://root:@localhost:3306/pulse_test";
    let pool = Pool::new(url).expect("url didn't parse");
    while ! std::path::Path::new("./disable-test.txt").exists() {
        let conn = pool.get_conn().expect("Connection didn't work");        
        check_db_pulse(conn);
    }
}
