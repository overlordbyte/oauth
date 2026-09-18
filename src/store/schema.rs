// diesel CLI orqali avtomatik generatsiya qilinadi: diesel migration run
diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
    }
}
