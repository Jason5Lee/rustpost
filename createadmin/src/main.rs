use mysql::prelude::Queryable;
use structopt::StructOpt;
use mysql::params;

#[derive(Debug, StructOpt)]
#[structopt(name = "rustpost-createadmin", about = "CLI for creating admin for rustpost")]
struct Opt {
    /// URI of rustpost MySQL DB. Can be overriden by MYSQL_URL environment variable.
    #[structopt(long)]
    mysql_uri: Option<String>,
    /// The password of admin
    #[structopt(short, long)]
    password: String,
    /// Cost of bcrypt
    #[structopt(long, default_value = "10")]
    cost: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    let mysql_uri = std::env::var("MYSQL_URI")
        .ok().or(opt.mysql_uri)
        .ok_or("cannot found MySQL URI")?;

    let encrypted = bcrypt::hash(opt.password, opt.cost)?;

    let pool = mysql::Pool::new_manual(1, 1, mysql::Opts::from_url(&mysql_uri)?)?;
    let mut conn = pool.get_conn()?;

    let mut id_bytes = [0u8; 8];
    getrandom::getrandom(&mut id_bytes)?;

    conn.exec_drop("INSERT INTO admins (id, bcrypted_password) VALUES (:id, :bcrypted_password)",
        params! {
            "id" => u64::from_le_bytes(id_bytes),
            "bcrypted_password" => encrypted,
        })?;
    println!("Admin Id: {}", base64::encode_config(&id_bytes, base64::URL_SAFE_NO_PAD));
    Ok(())
}

