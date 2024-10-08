use crate::{
    db::{local_db::LocalDb, types::IpV4},
    models::Contact,
};

type Result<T> = crate::db::local_db::Result<T>;

struct ContactsSurface<'a> {
    db: &'a LocalDb,
}

impl<'a> ContactsSurface<'a> {
    pub async fn new(db: &'a LocalDb) -> Result<Self> {
        let mut me = Self { db };
        Ok(me)
    }

    pub async fn write(&self, contact: Contact) -> Result<()> {
        let name = contact.get_name();
        let ip = contact.get_ip();
        sqlx::query!("INSERT INTO contacts (name, ip) VALUES (?, ?)", name, ip)
            .execute(self.db.get_pool())
            .await?;
        Ok(())
    }

    pub async fn get_all(&self) -> Result<Vec<Contact>> {
        sqlx::query_as("SELECT * FROM contacts")
            .fetch_all(self.db.get_pool())
            .await
    }
}
