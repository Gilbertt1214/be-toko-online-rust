use async_graphql::Enum;

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug)]
pub enum Role {
    Pengguna,
    Pengusaha,
    Admin,
}
